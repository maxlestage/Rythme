//! Rythme: a simple, purely functional programming language, with help
//! everywhere. This crate contains the whole interpreter.
//!
//! The pipeline: [`lexer`] → [`parser`] → [`check`] → [`interp`], driven by
//! [`driver`]. Every message exists in English and French ([`lang`]).

#[macro_use]
pub mod lang;
pub mod ast;
pub mod builtins;
pub mod check;
pub mod diagnostic;
pub mod driver;
pub mod help;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod source;
pub mod suggest;
pub mod symbol;
pub mod value;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
