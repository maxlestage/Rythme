//! Rythme: a simple, purely functional programming language, with help
//! everywhere. This crate contains the whole interpreter.
//!
//! The pipeline: [`lexer`] → [`parser`] → [`check`] → [`compile`] →
//! [`interp`], driven by [`driver`]. Every message exists in English and
//! French ([`lang`]).

#[macro_use]
pub mod lang;
pub mod ast;
pub mod builtins;
pub mod check;
pub mod compile;
pub mod diagnostic;
pub mod doc;
pub mod driver;
pub mod exercise;
pub mod format;
pub mod help;
pub mod interp;
pub mod io;
pub mod lexer;
// The language server talks on the standard input and output; the web playground has none.
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
pub mod modules;
pub mod parser;
#[cfg(not(target_arch = "wasm32"))]
pub mod repl;
pub mod source;
pub mod suggest;
pub mod symbol;
pub mod types;
pub mod value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
