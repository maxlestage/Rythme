// Drives the debugger `rythme dap` as VS Code does, with the launch
// configuration that package.json gives to new launch.json files:
// `npm run check`. RYTHME_BIN chooses the program.

'use strict';

const assert = require('assert');
const cp = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');

const repo = path.resolve(__dirname, '..', '..', '..');
const bin = process.env.RYTHME_BIN || path.join(repo, 'target', 'debug', process.platform === 'win32' ? 'rythme.exe' : 'rythme');

/** A Debug Adapter Protocol client: requests, answers and events. */
class Client {
  constructor(child) {
    this.child = child;
    this.seq = 0;
    this.buffer = Buffer.alloc(0);
    this.messages = [];
    this.waiting = [];
    child.stdout.on('data', (chunk) => {
      this.buffer = Buffer.concat([this.buffer, chunk]);
      this.parse();
    });
  }

  parse() {
    for (;;) {
      const end = this.buffer.indexOf('\r\n\r\n');
      if (end < 0) {
        return;
      }
      const length = Number(/Content-Length: (\d+)/i.exec(this.buffer.slice(0, end).toString())[1]);
      if (this.buffer.length < end + 4 + length) {
        return;
      }
      const message = JSON.parse(this.buffer.slice(end + 4, end + 4 + length).toString('utf8'));
      this.buffer = this.buffer.slice(end + 4 + length);
      this.messages.push(message);
      this.check();
    }
  }

  check() {
    for (const w of [...this.waiting]) {
      const i = this.messages.findIndex(w.test);
      if (i >= 0) {
        const [m] = this.messages.splice(i, 1);
        this.waiting.splice(this.waiting.indexOf(w), 1);
        w.resolve(m);
      }
    }
  }

  wait(test) {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error('no answer in time; received: ' + JSON.stringify(this.messages))), 20000);
      this.waiting.push({ test, resolve: (m) => (clearTimeout(timer), resolve(m)) });
      this.check();
    });
  }

  event(name) {
    return this.wait((m) => m.type === 'event' && m.event === name);
  }

  async request(command, args = {}) {
    const seq = ++this.seq;
    const body = JSON.stringify({ seq, type: 'request', command, arguments: args });
    this.child.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
    return this.wait((m) => m.type === 'response' && m.request_seq === seq);
  }
}

async function main() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'rythme-dap-'));
  const file = path.join(dir, 'fact.ry');
  fs.writeFileSync(file, 'fn fact(0) = 1\nfn fact(n) =\n  n * fact(n - 1)\n\nperson = {name: "Ada", age: 36}\nprint(fact(3))\n');
  // The configuration of a new launch.json, with VS Code's variables in place.
  const manifest = require('../package.json');
  const initial = manifest.contributes.debuggers.find((d) => d.type === 'rythme').initialConfigurations[0];
  const config = JSON.parse(JSON.stringify(initial).replace('${fileDirname}', dir.replace(/\\/g, '\\\\')).replace('${file}', file.replace(/\\/g, '\\\\')));
  assert.strictEqual(config.request, 'launch');

  const child = cp.spawn(bin, ['dap'], { env: { ...process.env, RYTHME_LANG: 'en' } });
  const client = new Client(child);
  const init = await client.request('initialize', { adapterID: 'rythme', linesStartAt1: true, columnsStartAt1: true });
  assert.ok(init.success && init.body.supportsConfigurationDoneRequest);
  const initialized = client.event('initialized');
  assert.ok((await client.request('launch', config)).success);
  await initialized;
  const set = await client.request('setBreakpoints', { source: { path: file }, breakpoints: [{ line: 3 }, { line: 4 }] });
  // Line 4 is empty: its breakpoint goes to the next line with code.
  assert.deepStrictEqual(set.body.breakpoints.map((b) => [b.verified, b.line]), [[true, 3], [true, 5]]);
  await client.request('setExceptionBreakpoints', { filters: [] });
  const stopped = client.event('stopped');
  await client.request('configurationDone');
  assert.strictEqual((await stopped).body.reason, 'breakpoint');

  // At `person = ...` (line 5), then in fact(3).
  let trace = await client.request('stackTrace', { threadId: 1 });
  assert.strictEqual(trace.body.stackFrames[0].line, 5);
  let next = client.event('stopped');
  await client.request('continue', { threadId: 1 });
  await next;
  trace = await client.request('stackTrace', { threadId: 1 });
  assert.deepStrictEqual(trace.body.stackFrames.map((f) => f.name), ['fact(n = 3)', 'top level']);
  assert.strictEqual(trace.body.stackFrames[0].source.path, fs.realpathSync(file));

  const scopes = await client.request('scopes', { frameId: 0 });
  const [locals, globals] = scopes.body.scopes;
  const vars = await client.request('variables', { variablesReference: locals.variablesReference });
  assert.deepStrictEqual(vars.body.variables.map((v) => [v.name, v.value]), [['n', '3']]);
  const top = await client.request('variables', { variablesReference: globals.variablesReference });
  const person = top.body.variables.find((v) => v.name === 'person');
  assert.strictEqual(person.value, '{name: "Ada", age: 36}');
  const fields = await client.request('variables', { variablesReference: person.variablesReference });
  assert.deepStrictEqual(fields.body.variables.map((v) => [v.name, v.value]), [['name', '"Ada"'], ['age', '36']]);

  const value = await client.request('evaluate', { expression: 'n * 10 + person.age', frameId: 0, context: 'watch' });
  assert.strictEqual(value.body.result, '66');
  const refused = await client.request('evaluate', { expression: 'print(n)', frameId: 0, context: 'repl' });
  assert.ok(!refused.success && refused.message.includes('effect'), refused.message);

  // Out of fact(3): its result, where it was called.
  await client.request('setBreakpoints', { source: { path: file }, breakpoints: [] });
  next = client.event('stopped');
  await client.request('stepOut', { threadId: 1 });
  const out = await next;
  assert.ok(out.body.description.includes('`fact` gave 6'), JSON.stringify(out.body));

  const output = client.wait((m) => m.type === 'event' && m.event === 'output' && m.body.category === 'stdout');
  const exited = client.event('exited');
  const terminated = client.event('terminated');
  await client.request('continue', { threadId: 1 });
  assert.strictEqual((await output).body.output, '6\n');
  assert.strictEqual((await exited).body.exitCode, 0);
  await terminated;
  const closed = new Promise((resolve) => child.on('exit', resolve));
  await client.request('disconnect', {});
  assert.strictEqual(await closed, 0);
  fs.rmSync(dir, { recursive: true, force: true });
  console.log('ok: VS Code\'s launch configuration debugs a program with `rythme dap`');
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
