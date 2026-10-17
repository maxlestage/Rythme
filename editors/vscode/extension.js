// The Rythme extension for VS Code: colors and snippets (declared in
// package.json), and the language server `rythme lsp` for help everywhere:
// problems as you type, documentation on hover, completion, the parameters of
// a call, go to definition, references, renaming, an outline, quick fixes.

'use strict';

const vscode = require('vscode');
const cp = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { LanguageClient } = require('vscode-languageclient/node');

const INSTALL_URL = 'https://github.com/maxlestage/Rythme#install';

/** @type {LanguageClient | undefined} */
let client;
/** @type {vscode.OutputChannel} */
let output;
/** Changes of .ry files, also those made outside of the editor. */
let watcher;
/** Help pages shown by the commands, by address. */
const pages = new Map();
const pagesChanged = new vscode.EventEmitter();

/** English or French, like VS Code. */
function t(en, fr) {
  return vscode.env.language.toLowerCase().startsWith('fr') ? fr : en;
}

function settings() {
  return vscode.workspace.getConfiguration('rythme');
}

function isFile(p) {
  try {
    return fs.statSync(p).isFile();
  } catch {
    return false;
  }
}

/** Finds a program in the folders of PATH. */
function onPath(name) {
  const exts = process.platform === 'win32' ? ['.exe', '.cmd', '.bat', ''] : [''];
  for (const dir of (process.env.PATH || '').split(path.delimiter)) {
    for (const ext of exts) {
      const candidate = path.join(dir, name + ext);
      if (dir && isFile(candidate)) {
        return candidate;
      }
    }
  }
  return undefined;
}

/**
 * The `rythme` program: the `rythme.path` setting, else the one on the PATH,
 * else the one installed by `cargo install` (often missing from the PATH of
 * programs started from the desktop).
 */
function program() {
  let configured = (settings().get('path') || '').trim() || 'rythme';
  if (configured.startsWith('~')) {
    configured = path.join(os.homedir(), configured.slice(1));
  }
  if (configured !== 'rythme') {
    // A relative path is relative to the first folder of the workspace.
    const folder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0];
    if (!path.isAbsolute(configured) && configured.includes('/') && folder) {
      return path.join(folder.uri.fsPath, configured);
    }
    return configured;
  }
  const exe = process.platform === 'win32' ? 'rythme.exe' : 'rythme';
  const cargo = path.join(os.homedir(), '.cargo', 'bin', exe);
  return onPath('rythme') || (isFile(cargo) ? cargo : 'rythme');
}

/** The language chosen in the settings: 'en', 'fr' or 'auto'. */
function languageSetting() {
  const v = settings().get('language');
  return v === 'en' || v === 'fr' ? v : 'auto';
}

/** `--lang` for the commands, so that they speak like the language server. */
function langArgs() {
  const chosen = languageSetting();
  if (chosen !== 'auto') {
    return ['--lang', chosen];
  }
  if (process.env.RYTHME_LANG) {
    return [];
  }
  const ui = vscode.env.language.toLowerCase();
  return ui.startsWith('fr') ? ['--lang', 'fr'] : ui.startsWith('en') ? ['--lang', 'en'] : [];
}

function notFound(command) {
  const install = t('How to install', 'Comment installer');
  const settingsButton = t('Settings', 'Réglages');
  vscode.window
    .showErrorMessage(
      t(
        `Rythme: the program \`${command}\` was not found. Install Rythme, or give its path in the setting \`rythme.path\`. Colors and snippets work without it.`,
        `Rythme : le programme \`${command}\` est introuvable. Installez Rythme, ou indiquez son chemin dans le réglage \`rythme.path\`. Les couleurs et les extraits de code fonctionnent sans lui.`
      ),
      install,
      settingsButton
    )
    .then((choice) => {
      if (choice === install) {
        vscode.env.openExternal(vscode.Uri.parse(INSTALL_URL));
      } else if (choice === settingsButton) {
        vscode.commands.executeCommand('workbench.action.openSettings', 'rythme.path');
      }
    });
}

/** Does the program run? `false` (with a message) when it cannot be found. */
function programRuns(command) {
  return new Promise((resolve) => {
    cp.execFile(command, ['version'], { timeout: 10000 }, (err) => {
      if (err && err.code === 'ENOENT') {
        notFound(command);
        resolve(false);
      } else {
        resolve(true);
      }
    });
  });
}

async function startServer() {
  const command = program();
  if (!(await programRuns(command))) {
    return;
  }
  const serverOptions = { command, args: ['lsp'] };
  const clientOptions = {
    documentSelector: [{ language: 'rythme' }],
    initializationOptions: { language: languageSetting() },
    // Settings changes reach the server (`rythme.language`); so do the
    // changes of .ry files outside of the editor (for the files they `use`).
    synchronize: {
      configurationSection: 'rythme',
      fileEvents: watcher,
    },
    outputChannel: output,
  };
  client = new LanguageClient('rythme', 'Rythme', serverOptions, clientOptions);
  try {
    await client.start();
  } catch (e) {
    client = undefined;
    const reason = e && e.message ? e.message : String(e);
    output.appendLine(reason);
    vscode.window.showErrorMessage(
      t(
        `Rythme: the language server did not start (${reason}). It needs a recent version of Rythme, which has the command \`rythme lsp\`. More in the Rythme output.`,
        `Rythme : le serveur de langage n'a pas démarré (${reason}). Il faut une version récente de Rythme, qui a la commande \`rythme lsp\`. Plus de détails dans la sortie Rythme.`
      )
    );
  }
}

async function stopServer() {
  const running = client;
  client = undefined;
  if (running) {
    try {
      await running.stop();
    } catch (e) {
      output.appendLine(String(e));
    }
  }
}

let restarting = Promise.resolve();

function restartServer() {
  restarting = restarting.then(async () => {
    await stopServer();
    await startServer();
    if (client) {
      vscode.window.setStatusBarMessage(t('Rythme: language server restarted', 'Rythme : serveur de langage redémarré'), 3000);
    }
  });
  return restarting;
}

/** Quotes a word for the shell; backslashes always (Git Bash would remove them). */
function quote(arg) {
  return /^[\w./:-]+$/.test(arg) ? arg : `"${arg.replace(/"/g, '\\"')}"`;
}

/** Runs `rythme [subcommand] file` in a terminal, after saving the file. */
async function runInTerminal(subcommand) {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'rythme') {
    vscode.window.showInformationMessage(t('Open a Rythme file (.ry) first.', "Ouvrez d'abord un fichier Rythme (.ry)."));
    return;
  }
  const doc = editor.document;
  if ((doc.isUntitled || doc.isDirty) && !(await doc.save())) {
    return;
  }
  const file = doc.uri.fsPath;
  const args = [...langArgs(), ...(subcommand ? [subcommand] : []), file];
  let terminal = vscode.window.terminals.find((term) => term.name === 'Rythme');
  if (!terminal) {
    terminal = vscode.window.createTerminal({ name: 'Rythme', cwd: path.dirname(file) });
  }
  terminal.show(true);
  // PowerShell runs a quoted program with `&`.
  const shell = (vscode.env.shell || '').toLowerCase();
  const call = shell.includes('powershell') || shell.includes('pwsh') ? '& ' : '';
  terminal.sendText(call + [program(), ...args].map(quote).join(' '));
}

/** Shows what `rythme args...` prints, in a read-only page beside the code. */
function showCommandOutput(args, title) {
  const command = program();
  return new Promise((resolve) => {
    const env = { ...process.env, NO_COLOR: '1' };
    cp.execFile(command, [...langArgs(), ...args], { timeout: 10000, env }, async (err, stdout, stderr) => {
      if (err && err.code === 'ENOENT') {
        notFound(command);
        resolve();
        return;
      }
      const uri = vscode.Uri.from({ scheme: 'rythme-help', path: `/${title}.txt` });
      pages.set(uri.toString(), `${stdout || ''}${stderr || ''}`);
      pagesChanged.fire(uri);
      const page = await vscode.workspace.openTextDocument(uri);
      await vscode.window.showTextDocument(page, { viewColumn: vscode.ViewColumn.Beside, preview: true, preserveFocus: true });
      resolve();
    });
  });
}

async function explainError() {
  // The code of the problem under the cursor, if there is one.
  let code = '';
  const editor = vscode.window.activeTextEditor;
  if (editor) {
    const here = vscode.languages
      .getDiagnostics(editor.document.uri)
      .find((d) => d.source === 'rythme' && d.code && d.range.contains(editor.selection.active));
    if (here) {
      code = String(typeof here.code === 'object' ? here.code.value : here.code);
    }
  }
  const asked = await vscode.window.showInputBox({
    prompt: t('Error code to explain', "Code d'erreur à expliquer"),
    placeHolder: 'E003',
    value: code,
  });
  if (asked && asked.trim()) {
    await showCommandOutput(['explain', asked.trim()], `rythme explain ${asked.trim().toUpperCase()}`);
  }
}

async function showHelp() {
  const asked = await vscode.window.showInputBox({
    prompt: t('A topic or a function (empty: the list of topics)', 'Un sujet ou une fonction (vide : la liste des sujets)'),
    placeHolder: 'map, lists, match, errors…',
  });
  if (asked === undefined) {
    return;
  }
  const topic = asked.trim();
  await showCommandOutput(topic ? ['help', topic] : ['help'], topic ? `rythme help ${topic}` : 'rythme help');
}

async function activate(context) {
  output = vscode.window.createOutputChannel('Rythme');
  watcher = vscode.workspace.createFileSystemWatcher('**/*.ry');
  context.subscriptions.push(
    output,
    watcher,
    pagesChanged,
    vscode.workspace.registerTextDocumentContentProvider('rythme-help', {
      onDidChange: pagesChanged.event,
      provideTextDocumentContent: (uri) => pages.get(uri.toString()) || '',
    }),
    vscode.commands.registerCommand('rythme.restartServer', restartServer),
    vscode.commands.registerCommand('rythme.runFile', () => runInTerminal('')),
    vscode.commands.registerCommand('rythme.testFile', () => runInTerminal('test')),
    vscode.commands.registerCommand('rythme.explainError', explainError),
    vscode.commands.registerCommand('rythme.showHelp', showHelp),
    vscode.workspace.onDidChangeConfiguration((e) => {
      // The language is sent to the running server; another program needs a new one.
      if (e.affectsConfiguration('rythme.path')) {
        restartServer();
      }
    })
  );
  await startServer();
}

function deactivate() {
  return stopServer();
}

module.exports = { activate, deactivate };
