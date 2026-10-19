// The Rythme extension for VS Code: colors and snippets (declared in
// package.json), and the language server `rythme lsp` for help everywhere:
// problems as you type, documentation on hover, completion, the parameters of
// a call, go to definition, references, renaming, an outline, quick fixes,
// the types of names, "Run" and "Run test" above the code (the server asks
// for the commands `rythme.runFile`, `rythme.testFile` and `rythme.runTest`,
// carried out here), the names of the workspace, folding. And the debugger:
// `rythme dap` for the debug sessions of type `rythme`.

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

/** The text in the language of VS Code: English, French, Spanish, German, Portuguese or Chinese. */
function t(en, fr, es, de, pt, zh) {
  const texts = { fr, es, de, pt, zh };
  return texts[vscode.env.language.toLowerCase().slice(0, 2)] || en;
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

/** The languages of the messages of Rythme. */
const LANGUAGES = ['en', 'fr', 'es', 'de', 'pt', 'zh'];

/** The language chosen in the settings: one of `LANGUAGES`, or 'auto'. */
function languageSetting() {
  const v = settings().get('language');
  return LANGUAGES.includes(v) ? v : 'auto';
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
  const ui = vscode.env.language.toLowerCase().slice(0, 2);
  return LANGUAGES.includes(ui) ? ['--lang', ui] : [];
}

function notFound(command) {
  const install = t('How to install', 'Comment installer', 'Cómo instalarlo', 'Installationsanleitung', 'Como instalar', '如何安装');
  const settingsButton = t('Settings', 'Réglages', 'Ajustes', 'Einstellungen', 'Configurações', '设置');
  vscode.window
    .showErrorMessage(
      t(
        `Rythme: the program \`${command}\` was not found. Install Rythme, or give its path in the setting \`rythme.path\`. Colors and snippets work without it.`,
        `Rythme : le programme \`${command}\` est introuvable. Installez Rythme, ou indiquez son chemin dans le réglage \`rythme.path\`. Les couleurs et les extraits de code fonctionnent sans lui.`,
        `Rythme: no se encontró el programa \`${command}\`. Instala Rythme, o indica su ruta en el ajuste \`rythme.path\`. Los colores y los fragmentos funcionan sin él.`,
        `Rythme: das Programm \`${command}\` wurde nicht gefunden. Installiere Rythme, oder gib seinen Pfad in der Einstellung \`rythme.path\` an. Farben und Snippets funktionieren auch ohne es.`,
        `Rythme: o programa \`${command}\` não foi encontrado. Instale o Rythme, ou informe o caminho dele na configuração \`rythme.path\`. As cores e os snippets funcionam sem ele.`,
        `Rythme：找不到程序 \`${command}\`。请安装 Rythme，或者在设置 \`rythme.path\` 中给出它的路径。没有它，着色和代码片段也能使用。`
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
        `Rythme : le serveur de langage n'a pas démarré (${reason}). Il faut une version récente de Rythme, qui a la commande \`rythme lsp\`. Plus de détails dans la sortie Rythme.`,
        `Rythme: el servidor de lenguaje no arrancó (${reason}). Hace falta una versión reciente de Rythme, que tenga el comando \`rythme lsp\`. Más detalles en la salida Rythme.`,
        `Rythme: der Sprachserver ist nicht gestartet (${reason}). Er braucht eine aktuelle Version von Rythme, die den Befehl \`rythme lsp\` hat. Mehr in der Ausgabe Rythme.`,
        `Rythme: o servidor de linguagem não iniciou (${reason}). É preciso uma versão recente do Rythme, que tenha o comando \`rythme lsp\`. Mais detalhes na saída Rythme.`,
        `Rythme：语言服务器没有启动（${reason}）。它需要较新版本的 Rythme，带有命令 \`rythme lsp\`。更多信息见 Rythme 输出。`
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
      vscode.window.setStatusBarMessage(t(
          'Rythme: language server restarted',
          'Rythme : serveur de langage redémarré',
          'Rythme: servidor de lenguaje reiniciado',
          'Rythme: Sprachserver neu gestartet',
          'Rythme: servidor de linguagem reiniciado',
          'Rythme：语言服务器已重启'
        ), 3000);
    }
  });
  return restarting;
}

/**
 * Quotes a word for the shell, so that it reaches the program as it is
 * (backslashes, `$`, quotes): single quotes for POSIX shells and PowerShell
 * (Git Bash keeps the backslashes of Windows paths), double quotes for cmd.
 */
function quote(arg, shell) {
  if (/^[\w./:-]+$/.test(arg)) {
    return arg;
  }
  if (shell === 'cmd') {
    return `"${arg.replace(/"/g, '""')}"`;
  }
  if (shell === 'powershell') {
    return `'${arg.replace(/'/g, "''")}'`;
  }
  return `'${arg.replace(/'/g, "'\\''")}'`;
}

/** The kind of shell of the terminals: 'powershell', 'cmd' or 'posix'. */
function shellKind() {
  const shell = (vscode.env.shell || '').toLowerCase();
  if (shell.includes('powershell') || shell.includes('pwsh')) {
    return 'powershell';
  }
  return shell.endsWith('cmd.exe') ? 'cmd' : 'posix';
}

/**
 * The document to run: the one named by `target` (an address, from a code
 * lens of the language server, or a Uri, from the buttons of the editor),
 * else the active one.
 */
async function documentToRun(target) {
  if (target) {
    const uri = typeof target === 'string' ? vscode.Uri.parse(target) : target;
    const open = vscode.workspace.textDocuments.find((d) => d.uri.toString() === uri.toString());
    return open || vscode.workspace.openTextDocument(uri);
  }
  const editor = vscode.window.activeTextEditor;
  return editor && editor.document.languageId === 'rythme' ? editor.document : undefined;
}

/**
 * Runs `rythme [subcommand] file [extra...]` in a terminal, after saving the
 * file. `target` chooses the file (see `documentToRun`).
 */
async function runInTerminal(subcommand, target, extra = []) {
  const doc = await documentToRun(target);
  if (!doc) {
    vscode.window.showInformationMessage(t(
        'Open a Rythme file (.ry) first.',
        "Ouvrez d'abord un fichier Rythme (.ry).",
        'Abre antes un archivo de Rythme (.ry).',
        'Öffne zuerst eine Rythme-Datei (.ry).',
        'Abra antes um arquivo do Rythme (.ry).',
        '请先打开一个 Rythme 文件（.ry）。'
      ));
    return;
  }
  if ((doc.isUntitled || doc.isDirty) && !(await doc.save())) {
    return;
  }
  const file = doc.uri.fsPath;
  const args = [...langArgs(), ...(subcommand ? [subcommand] : []), file, ...extra];
  let terminal = vscode.window.terminals.find((term) => term.name === 'Rythme');
  if (!terminal) {
    terminal = vscode.window.createTerminal({ name: 'Rythme', cwd: path.dirname(file) });
  }
  terminal.show(true);
  // PowerShell runs a quoted program with `&`.
  const shell = shellKind();
  const call = shell === 'powershell' ? '& ' : '';
  terminal.sendText(call + [program(), ...args].map((a) => quote(a, shell)).join(' '));
}

/** The arguments of `rythme test file ...` that run only the test `name`. */
function testFilter(name) {
  return ['--filter', name];
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
    prompt: t('Error code to explain', "Code d'erreur à expliquer", 'Código de error que explicar', 'Zu erklärender Fehlercode', 'Código de erro a explicar', '要解释的错误代码'),
    placeHolder: 'E003',
    value: code,
  });
  if (asked && asked.trim()) {
    await showCommandOutput(['explain', asked.trim()], `rythme explain ${asked.trim().toUpperCase()}`);
  }
}

async function showHelp() {
  const asked = await vscode.window.showInputBox({
    prompt: t(
      'A topic or a function (empty: the list of topics)',
      'Un sujet ou une fonction (vide : la liste des sujets)',
      'Un tema o una función (vacío: la lista de temas)',
      'Ein Thema oder eine Funktion (leer: die Liste der Themen)',
      'Um tema ou uma função (vazio: a lista de temas)',
      '一个主题或一个函数（留空：主题列表）'
    ),
    placeHolder: 'map, lists, match, errors…',
  });
  if (asked === undefined) {
    return;
  }
  const topic = asked.trim();
  await showCommandOutput(topic ? ['help', topic] : ['help'], topic ? `rythme help ${topic}` : 'rythme help');
}

/**
 * The debugger: VS Code starts `rythme dap` (the Debug Adapter Protocol on
 * its standard input and output) for the debug sessions of type `rythme`.
 */
const debugAdapterFactory = {
  createDebugAdapterDescriptor() {
    return new vscode.DebugAdapterExecutable(program(), [...langArgs(), 'dap']);
  },
};

/** The name of the configuration made when there is no launch.json. */
function debugName() {
  return t(
    'Debug this Rythme file',
    'Déboguer ce fichier Rythme',
    'Depurar este archivo Rythme',
    'Diese Rythme-Datei debuggen',
    'Depurar este arquivo Rythme',
    '调试此 Rythme 文件'
  );
}

/**
 * Completes a debug configuration: F5 without launch.json debugs the open
 * .ry file; a configuration needs a program.
 */
const debugConfigurationProvider = {
  resolveDebugConfiguration(folder, config) {
    if (!config.type && !config.request && !config.name) {
      const editor = vscode.window.activeTextEditor;
      if (editor && editor.document.languageId === 'rythme') {
        config = { type: 'rythme', request: 'launch', name: debugName(), program: editor.document.uri.fsPath };
      }
    }
    if (!config.program) {
      vscode.window.showInformationMessage(t(
          'Open a Rythme file (.ry) first.',
          "Ouvrez d'abord un fichier Rythme (.ry).",
          'Abre antes un archivo de Rythme (.ry).',
          'Öffne zuerst eine Rythme-Datei (.ry).',
          'Abra antes um arquivo do Rythme (.ry).',
          '请先打开一个 Rythme 文件（.ry）。'
        ));
      return undefined;
    }
    if (!config.cwd) {
      config.cwd = path.dirname(config.program);
    }
    return config;
  },
};

/** "Debug this file": saves the file and starts the debugger on it. */
async function debugFile(target) {
  const doc = await documentToRun(target);
  if (!doc) {
    return debugConfigurationProvider.resolveDebugConfiguration(undefined, { type: 'rythme' });
  }
  if ((doc.isUntitled || doc.isDirty) && !(await doc.save())) {
    return undefined;
  }
  const file = doc.uri.fsPath;
  return vscode.debug.startDebugging(undefined, { type: 'rythme', request: 'launch', name: debugName(), program: file, cwd: path.dirname(file) });
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
    vscode.commands.registerCommand('rythme.runFile', (target) => runInTerminal('', target)),
    vscode.commands.registerCommand('rythme.testFile', (target) => runInTerminal('test', target)),
    // From the "Run test" code lens: the address of the file and the name of the test.
    vscode.commands.registerCommand('rythme.runTest', (target, name) =>
      runInTerminal('test', target, typeof name === 'string' ? testFilter(name) : [])
    ),
    vscode.commands.registerCommand('rythme.explainError', explainError),
    vscode.commands.registerCommand('rythme.showHelp', showHelp),
    vscode.commands.registerCommand('rythme.debugFile', debugFile),
    vscode.debug.registerDebugAdapterDescriptorFactory('rythme', debugAdapterFactory),
    vscode.debug.registerDebugConfigurationProvider('rythme', debugConfigurationProvider),
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

module.exports = { activate, deactivate, quote, testFilter, debugAdapterFactory, debugConfigurationProvider };
