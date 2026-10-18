// Checks the extension without VS Code: `npm run check`.
//
// 1. extension.js and the real vscode-languageclient load, with a stand-in
//    for the `vscode` module (which only exists inside VS Code).
// 2. activate() registers the commands and starts `rythme lsp` with the
//    right options; a missing program gives a message instead.
//
// RYTHME_BIN chooses the program (default: the debug build of the repository).

'use strict';

const assert = require('assert');
const Module = require('module');
const path = require('path');

const repo = path.resolve(__dirname, '..', '..', '..');
const bin = process.env.RYTHME_BIN || path.join(repo, 'target', 'debug', process.platform === 'win32' ? 'rythme.exe' : 'rythme');

/** A `vscode` where every name exists: enough for modules to load. */
function anything() {
  const f = function () {};
  return new Proxy(f, {
    get: (target, key) => (key === 'prototype' ? target.prototype : key === Symbol.toPrimitive ? () => '' : anything()),
    apply: () => anything(),
    construct: () => anything(),
  });
}

/** A `vscode` that records what the extension does. */
function fakeVscode(config) {
  const done = { commands: {}, messages: [], sent: [] };
  const disposable = { dispose() {} };
  class EventEmitter {
    constructor() {
      this.event = () => disposable;
    }
    fire() {}
    dispose() {}
  }
  const vscode = {
    done,
    env: { language: 'fr', shell: '/bin/bash', openExternal() {} },
    EventEmitter,
    Uri: {
      parse: (s) => ({ toString: () => s, fsPath: decodeURIComponent(s.replace(/^file:\/\//, '')) }),
      from: (o) => ({ toString: () => `${o.scheme}:${o.path}` }),
    },
    ViewColumn: { Beside: -2 },
    window: {
      createOutputChannel: () => ({ appendLine() {}, append() {}, show() {}, dispose() {} }),
      showErrorMessage: (m) => {
        done.messages.push(m);
        return Promise.resolve(undefined);
      },
      showInformationMessage: (m) => {
        done.messages.push(m);
        return Promise.resolve(undefined);
      },
      setStatusBarMessage() {},
      terminals: [],
      createTerminal: () => ({ name: 'Rythme', show() {}, sendText: (text) => done.sent.push(text) }),
      activeTextEditor: undefined,
    },
    workspace: {
      getConfiguration: () => ({ get: (key) => config[key] }),
      workspaceFolders: undefined,
      textDocuments: [],
      openTextDocument: (uri) => Promise.resolve({ uri, isUntitled: false, isDirty: false, languageId: 'rythme' }),
      registerTextDocumentContentProvider: () => disposable,
      onDidChangeConfiguration: () => disposable,
      createFileSystemWatcher: () => disposable,
    },
    commands: {
      registerCommand: (name, f) => {
        done.commands[name] = f;
        return disposable;
      },
      executeCommand() {},
    },
    languages: { getDiagnostics: () => [] },
  };
  return vscode;
}

/** Loads extension.js afresh, with these modules standing in for others. */
function load(stubs) {
  const original = Module._load;
  Module._load = function (request, ...rest) {
    if (request in stubs) {
      return stubs[request];
    }
    return original.call(this, request, ...rest);
  };
  try {
    for (const key of Object.keys(require.cache)) {
      delete require.cache[key];
    }
    return require('../extension.js');
  } finally {
    Module._load = original;
  }
}

async function main() {
  // 1. The real language client loads.
  const ext = load({ vscode: anything() });
  assert.strictEqual(typeof ext.activate, 'function');
  assert.strictEqual(typeof ext.deactivate, 'function');

  // 2. activate() with a stand-in client.
  const started = [];
  class FakeClient {
    constructor(id, name, server, options) {
      Object.assign(this, { id, name, server, options });
    }
    start() {
      started.push(this);
      return Promise.resolve();
    }
    stop() {
      return Promise.resolve();
    }
  }
  const vscode = fakeVscode({ path: bin, language: 'auto' });
  const withClient = load({ vscode, 'vscode-languageclient/node': { LanguageClient: FakeClient } });
  await withClient.activate({ subscriptions: [] });
  for (const name of ['rythme.restartServer', 'rythme.runFile', 'rythme.testFile', 'rythme.runTest', 'rythme.explainError', 'rythme.showHelp']) {
    assert.ok(vscode.done.commands[name], `${name} is registered`);
  }
  assert.strictEqual(started.length, 1, `the server starts (${vscode.done.messages.join(' ')})`);
  const c = started[0];
  assert.deepStrictEqual(c.server, { command: bin, args: ['lsp'] });
  assert.deepStrictEqual(c.options.documentSelector, [{ language: 'rythme' }]);
  assert.deepStrictEqual(c.options.initializationOptions, { language: 'auto' });
  assert.strictEqual(c.options.synchronize.configurationSection, 'rythme');
  await vscode.done.commands['rythme.restartServer']();
  assert.strictEqual(started.length, 2, 'the server restarts');
  // The "Run test" code lens of the server: the file and the name of the test, quoted for the shell.
  await vscode.done.commands['rythme.runTest']('file:///tmp/my%20tests.ry', "it's $HOME");
  const lang = process.env.RYTHME_LANG ? '' : '--lang fr ';
  assert.deepStrictEqual(vscode.done.sent, [`${bin} ${lang}test '/tmp/my tests.ry' --filter 'it'\\''s $HOME'`]);
  await vscode.done.commands['rythme.runFile']('file:///tmp/a.ry');
  assert.strictEqual(vscode.done.sent[1], `${bin} ${lang}/tmp/a.ry`);
  assert.strictEqual(withClient.quote('C:\\Users\\a b.ry', 'powershell'), "'C:\\Users\\a b.ry'");
  assert.strictEqual(withClient.quote('say "hi"', 'cmd'), '"say ""hi"""');
  await withClient.deactivate();

  // A missing program: a message in the language of VS Code, and no server.
  started.length = 0;
  const missing = fakeVscode({ path: path.join(repo, 'no-such-rythme'), language: 'fr' });
  const without = load({ vscode: missing, 'vscode-languageclient/node': { LanguageClient: FakeClient } });
  await without.activate({ subscriptions: [] });
  assert.strictEqual(started.length, 0);
  assert.ok(missing.done.messages.some((m) => m.includes('introuvable')), missing.done.messages.join('\n'));

  console.log('ok: the extension loads, registers its commands and starts `rythme lsp`');
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
