//! The website of Rythme, written with Paquets: the home page, the tour of the
//! language, the examples, the playground (the interpreter compiled to WebAssembly)
//! and the guide for teachers, in six languages; installable, and offline once
//! visited. Made to be read by AI assistants too: `/llms.txt`, every page as
//! Markdown, structured data, and a page that tells how to give them Rythme.

pub mod ai;
pub mod client;
pub mod examples;
pub mod export;
pub mod highlight;
pub mod i18n;
pub mod json;
pub mod markdown;
pub mod pages;
pub mod pwa;
pub mod routes;
pub mod server;
pub mod structured;
pub mod teach;
pub mod themes;
