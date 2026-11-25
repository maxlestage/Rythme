//! A program that embeds Rythme adds its own functions: as globals, or as
//! the fields of a record (`db.all(q)`). The checker knows both.

use std::path::Path;
use std::rc::Rc;

use rythme::builtins::{Builtin, Category};
use rythme::diagnostic::Fallible;
use rythme::driver::Driver;
use rythme::interp::{Interp, Mode};
use rythme::source::Span;
use rythme::symbol::Sym;
use rythme::value::{Record, Value};

fn rows(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    Ok(Value::list(vec![a[0].clone(), a[0].clone()]))
}

static ALL: Builtin = Builtin {
    name: "all",
    sig: "db.all(query)",
    min: 1,
    max: 1,
    cat: Category::Basics,
    en: "Rows.",
    fr: "Des lignes.",
    example: "",
    func: rows,
};
static INSERT: Builtin = Builtin {
    name: "insert",
    sig: "db.insert(table, row)",
    min: 2,
    max: 2,
    cat: Category::Basics,
    en: "Adds a row.",
    fr: "Ajoute une ligne.",
    example: "",
    func: rows,
};

/// Runs `code` with a global record `db`; returns the messages and the last value.
fn run(code: &str) -> (String, Option<String>) {
    rythme::io::capture();
    let mut d = Driver::new();
    d.color = false;
    let mut db = Record::new();
    db.set(Rc::from("all"), Value::Builtin(&ALL));
    db.set(Rc::from("insert"), Value::Builtin(&INSERT));
    d.interp.globals.insert(Sym::new("db"), Value::Record(Rc::new(db)));
    let value = d.load_source("host.ry", code, None, Path::new(".")).and_then(|m| d.execute(&m, Mode::Run).ok().flatten());
    (rythme::io::take_captured(), value.map(|v| v.repr()))
}

#[test]
fn fields_of_a_host_record_are_known() {
    // `all` is also a function of the language, with two arguments: here it is the field.
    let (messages, value) = run("db.all(:tasks) |> map(fn(x) = x)");
    assert_eq!(messages, "");
    assert_eq!(value.as_deref(), Some("[:tasks, :tasks]"));
    let (messages, _) = run("db.insert(:tasks, {title: \"a\"})");
    assert_eq!(messages, "");
}

#[test]
fn a_misspelled_field_of_a_host_record_is_reported() {
    let (messages, _) = run("db.instert(:tasks, {})");
    assert!(messages.contains("W005") || messages.contains("E0"), "{messages}");
    assert!(messages.contains("insert"), "a suggestion is expected: {messages}");
}

#[test]
fn the_number_of_arguments_of_a_host_function_is_checked() {
    let (messages, _) = run("db.insert(:tasks)");
    assert!(messages.contains("E004"), "{messages}");
}

static TAG: Builtin = Builtin {
    name: "header",
    sig: "header(children)",
    min: 0,
    max: 1,
    cat: Category::Basics,
    en: "An HTML element.",
    fr: "Un élément HTML.",
    example: "",
    func: rows,
};

#[test]
fn a_host_function_may_be_defined_again() {
    rythme::io::capture();
    let mut d = Driver::new();
    d.color = false;
    d.interp.globals.insert(Sym::new("header"), Value::Builtin(&TAG));
    let m = d.load_source("host.ry", "fn header(title) = \"# {title}\"\nheader(\"Accueil\")", None, Path::new("."));
    let value = m.and_then(|m| d.execute(&m, Mode::Run).ok().flatten()).map(|v| v.repr());
    let messages = rythme::io::take_captured();
    assert_eq!(messages, "", "no W003 for a function of the host");
    assert_eq!(value.as_deref(), Some("\"# Accueil\""));
    // The language's own functions still warn.
    rythme::io::capture();
    let mut d = Driver::new();
    d.color = false;
    let _ = d.load_source("host.ry", "fn len(x) = 0\nlen(1)", None, Path::new("."));
    assert!(rythme::io::take_captured().contains("W003"));
}
