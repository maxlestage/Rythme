//! Rythme: a simple, purely functional programming language, with help
//! everywhere. This crate contains the whole interpreter.
//!
//! The pipeline: [`lexer`] → [`parser`] → [`check`] → [`compile`] →
//! [`interp`], driven by [`driver`]. Every message exists in English,
//! French and the languages of `src/i18n/` ([`lang`]).

#[macro_use]
pub mod lang;
pub mod ast;
pub mod bigint;
pub mod buffer;
pub mod builtins;
pub mod check;
pub mod compile;
pub mod debug;
pub mod diagnostic;
pub mod dict;
pub mod doc;
pub mod driver;
pub mod exercise;
pub mod format;
pub mod help;
pub mod interp;
pub mod io;
pub mod lexer;
pub mod manifest;
// The language server talks on the standard input and output; the web playground has none.
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
pub mod modules;
pub mod parser;
pub mod project;
#[cfg(not(target_arch = "wasm32"))]
pub mod repl;
pub mod source;
pub mod steps;
pub mod suggest;
pub mod symbol;
pub mod testing;
pub mod types;
pub mod value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
