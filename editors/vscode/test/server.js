// Talks to the real `rythme lsp` with vscode-jsonrpc, the library that VS Code
// uses for the protocol: `npm run check`. RYTHME_BIN chooses the program.

'use strict';

const assert = require('assert');
const cp = require('child_process');
const path = require('path');
const rpc = require('vscode-jsonrpc/node');

const repo = path.resolve(__dirname, '..', '..', '..');
const bin = process.env.RYTHME_BIN || path.join(repo, 'target', 'debug', process.platform === 'win32' ? 'rythme.exe' : 'rythme');

async function main() {
  const server = cp.spawn(bin, ['lsp'], { env: { ...process.env, RYTHME_LANG: '' } });
  const connection = rpc.createMessageConnection(new rpc.StreamMessageReader(server.stdout), new rpc.StreamMessageWriter(server.stdin));
  const published = [];
  let resolveDiagnostics;
  const diagnostics = new Promise((resolve) => (resolveDiagnostics = resolve));
  connection.onNotification('textDocument/publishDiagnostics', (params) => {
    published.push(params);
    resolveDiagnostics(params);
  });
  connection.listen();

  const init = await connection.sendRequest('initialize', {
    processId: process.pid,
    rootUri: null,
    locale: 'fr',
    capabilities: { textDocument: { hover: { contentFormat: ['markdown', 'plaintext'] } } },
  });
  assert.strictEqual(init.serverInfo.name, 'rythme');
  assert.strictEqual(init.capabilities.positionEncoding, 'utf-16');
  connection.sendNotification('initialized', {});

  const uri = 'file:///nowhere/essai.ry';
  const text = '## Le double.\nfn double(x) = x * 2\nprint("été 🎉", doubel(2))\n';
  connection.sendNotification('textDocument/didOpen', { textDocument: { uri, languageId: 'rythme', version: 1, text } });
  const diags = await diagnostics;
  assert.strictEqual(diags.uri, uri);
  assert.strictEqual(diags.diagnostics.length, 1);
  const d = diags.diagnostics[0];
  assert.strictEqual(d.code, 'E003');
  assert.ok(d.message.includes('vouliez-vous dire `double` ?'), d.message);
  // `print("été 🎉", ` is 16 UTF-16 units long (the emoji counts for two).
  assert.deepStrictEqual(d.range, { start: { line: 2, character: 16 }, end: { line: 2, character: 22 } });

  const hover = await connection.sendRequest('textDocument/hover', { textDocument: { uri }, position: { line: 1, character: 5 } });
  assert.ok(hover.contents.value.includes('Le double.'), hover.contents.value);
  const items = await connection.sendRequest('textDocument/completion', { textDocument: { uri }, position: { line: 2, character: 0 } });
  assert.ok(items.some((i) => i.label === 'double') && items.some((i) => i.label === 'map'));

  // The extras: the types of names, colors, "Run" above the code, the names of the workspace.
  assert.deepStrictEqual(init.capabilities.semanticTokensProvider.full, { delta: true });
  const typed = 'nom = upper("ada")\nprint(nom)\ntest "nom" = assert_eq(nom, "ADA")\n';
  const typedUri = 'file:///nowhere/types.ry';
  connection.sendNotification('textDocument/didOpen', { textDocument: { uri: typedUri, languageId: 'rythme', version: 1, text: typed } });
  const whole = { start: { line: 0, character: 0 }, end: { line: 3, character: 0 } };
  const hints = await connection.sendRequest('textDocument/inlayHint', { textDocument: { uri: typedUri }, range: whole });
  assert.deepStrictEqual(hints[0], { position: { line: 0, character: 3 }, label: ': Text', kind: 1 });
  const tokens = await connection.sendRequest('textDocument/semanticTokens/full', { textDocument: { uri: typedUri } });
  assert.deepStrictEqual(tokens.data.slice(0, 5), [0, 0, 3, 1, 1]);
  const lenses = await connection.sendRequest('textDocument/codeLens', { textDocument: { uri: typedUri } });
  assert.deepStrictEqual(
    lenses.map((l) => l.command.command),
    ['rythme.runFile', 'rythme.runTest']
  );
  assert.deepStrictEqual(lenses[1].command.arguments, [typedUri, 'nom']);
  const symbols = await connection.sendRequest('workspace/symbol', { query: 'doub' });
  assert.deepStrictEqual(symbols.map((x) => x.name), ['double']);

  // A request the server does not know: an error answer, and the server goes on.
  await assert.rejects(connection.sendRequest('rythme/unknown', {}), /unknown method/);

  assert.strictEqual(await connection.sendRequest('shutdown'), null);
  const exited = new Promise((resolve) => server.on('exit', resolve));
  connection.sendNotification('exit');
  assert.strictEqual(await exited, 0);
  connection.dispose();
  console.log('ok: vscode-jsonrpc and `rythme lsp` understand each other');
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
