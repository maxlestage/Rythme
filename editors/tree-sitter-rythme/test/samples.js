// Parses every example and every sample of the manual with the grammar, and
// checks that the queries of the editors are valid.
//
// A sample that `rythme` itself refuses (the manual shows some errors on
// purpose) is skipped: `rythme fmt --check` tells which ones. Set RYTHME_BIN
// to the program (by default target/debug/rythme of the repository).
//
//   npm test            (after `cargo build` at the root)

'use strict';

const { execFileSync, spawnSync } = require('child_process');
const fs = require('fs');
const os = require('os');
const path = require('path');

const grammarDir = path.resolve(__dirname, '..');
const root = path.resolve(grammarDir, '..', '..');
const exe = process.platform === 'win32' ? '.exe' : '';
const rythme = process.env.RYTHME_BIN || path.join(root, 'target', 'debug', 'rythme' + exe);
const treeSitter = path.join(grammarDir, 'node_modules', '.bin', 'tree-sitter' + (process.platform === 'win32' ? '.cmd' : ''));

if (!fs.existsSync(rythme)) {
  console.error(`rythme not found at ${rythme}: run \`cargo build\` at the root, or set RYTHME_BIN`);
  process.exit(1);
}

/** Files of `dir` (recursively) whose name matches `re`. */
function files(dir, re) {
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap(e => {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) return e.name === 'node_modules' ? [] : files(p, re);
    return re.test(e.name) ? [p] : [];
  });
}

const samples = [];
for (const f of files(path.join(root, 'examples'), /\.ry$/)) {
  samples.push({ name: path.relative(root, f), code: fs.readFileSync(f, 'utf8') });
}
const docs = [
  ...files(path.join(root, 'docs'), /\.md$/),
  ...files(root, /^README.*\.md$/).filter(f => path.dirname(f) === root),
  ...files(path.join(root, 'editors'), /^README.*\.md$/).filter(f => path.dirname(path.dirname(f)) === root),
];
for (const f of docs) {
  const text = fs.readFileSync(f, 'utf8').replace(/\r\n/g, '\n');
  let i = 0;
  for (const m of text.matchAll(/^```rythme\n([\s\S]*?)^```/gm)) {
    samples.push({ name: `${path.relative(root, f)} (sample ${++i})`, code: m[1] });
  }
}

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'rythme-tree-sitter-'));
const kept = [];
let refused = 0;
samples.forEach((s, i) => {
  const file = path.join(tmp, `sample${i}.ry`);
  fs.writeFileSync(file, s.code);
  const r = spawnSync(rythme, ['--lang', 'en', 'fmt', '--check', file], { encoding: 'utf8' });
  if (r.error) throw r.error;
  if ((r.stdout + r.stderr).includes('error[')) {
    refused++;
  } else {
    kept.push({ ...s, file });
  }
});

let failed = 0;
for (const s of kept) {
  const r = spawnSync(treeSitter, ['parse', '-q', s.file], { cwd: grammarDir, encoding: 'utf8', shell: process.platform === 'win32' });
  if (r.status !== 0 || /ERROR|MISSING/.test(r.stdout)) {
    failed++;
    console.error(`✗ ${s.name}\n${r.stdout}${r.stderr}`);
    console.error(execFileSync(treeSitter, ['parse', s.file], { cwd: grammarDir, encoding: 'utf8' }).toString());
  }
}
console.log(`${kept.length} samples parsed (${refused} refused by rythme itself, skipped), ${failed} with errors`);

// The queries: each must be valid for the grammar.
const queries = [
  ...files(path.join(grammarDir, 'queries'), /\.scm$/),
  ...files(path.join(root, 'editors', 'zed', 'languages'), /\.scm$/),
  ...files(path.join(root, 'editors', 'helix'), /\.scm$/),
];
const example = path.join(root, 'examples', 'shapes.ry');
for (const q of queries) {
  const r = spawnSync(treeSitter, ['query', '-q', q, example], { cwd: grammarDir, encoding: 'utf8', shell: process.platform === 'win32' });
  if (r.status !== 0) {
    failed++;
    console.error(`✗ invalid query ${path.relative(root, q)}\n${r.stdout}${r.stderr}`);
  }
}
console.log(`${queries.length} query files checked`);

fs.rmSync(tmp, { recursive: true, force: true });
if (kept.length < 100 || failed > 0) {
  process.exit(1);
}
