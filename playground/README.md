# Rythme playground

Rythme in the browser: the real interpreter compiled to WebAssembly, with
the examples, the tests and the help of the `rythme` command.

## Build

```sh
rustup target add wasm32-unknown-unknown
playground/build.sh        # creates web/rythme.wasm and web/examples.json
```

## Try it locally

```sh
cd playground/web
{ echo '<!doctype html><meta charset="utf-8">'; cat index.html; } > local.html
python3 -m http.server 8000      # then open http://localhost:8000/local.html
```

## Publish

The `Playground` workflow (`.github/workflows/pages.yml`) builds it and
publishes it on GitHub Pages at every push on `master`, once Pages is enabled
in the repository settings (Settings → Pages → Source: GitHub Actions).

## How it works

- `src/lib.rs` exports `rythme_alloc`, `rythme_run` and `rythme_result_len`.
  The page writes the code in the WebAssembly memory, runs it, and reads the
  output back. Error messages are wrapped in the characters U+0001 … U+0002 so
  that the page can color them.
- Programs run in a Web Worker: a program that never stops can be stopped.
- The browser's call stack is small, so nested (non-tail) calls are limited
  to 600 here; tail calls have no limit. The installed `rythme` allows about
  200,000 nested calls.
- Files cannot be read in the browser, and `input()` returns an empty text.
