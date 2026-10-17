//! The types inferred by the checker are right: every value that a program
//! computes has the type inferred for it, and the checker only reports an
//! error when the program really fails there. Both are tested on the
//! built-in functions with many kinds of values, and on thousands of
//! generated programs.

use std::collections::HashMap;
use std::rc::Rc;

use rythme::builtins::{lookup, BUILTINS};
use rythme::check::{self, Knowledge};
use rythme::diagnostic::Severity;
use rythme::interp::{Interp, Mode};
use rythme::source::{self, Span};
use rythme::symbol::Sym;
use rythme::types::{builtin_call, builtin_value, Outcome, Ty};
use rythme::value::{CtorInfo, Record, Value, Variant};
use rythme::{lexer, parser};

/// Values of every kind, to call the built-in functions with.
fn samples(it: &Interp) -> Vec<Value> {
    let int = Value::Int;
    let text = Value::text;
    let mut one_field = Record::new();
    one_field.set(Rc::from("a"), int(1));
    let leaf = Rc::new(CtorInfo { type_name: Sym::new("T"), name: Sym::new("A"), fields: vec![Sym::new("x")] });
    vec![
        Value::Unit,
        Value::Bool(true),
        Value::Bool(false),
        int(0),
        int(3),
        int(-2),
        Value::Float(1.5),
        Value::Float(-0.5),
        Value::Float(2.0),
        text(""),
        text("a b"),
        text("42"),
        Value::list(Vec::new()),
        Value::list(vec![int(1), int(2)]),
        Value::list(vec![text("x"), text("y")]),
        Value::list(vec![Value::list(vec![int(1)])]),
        Value::list(vec![int(1), Value::Float(0.5)]),
        Value::Record(Rc::new(Record::new())),
        Value::Record(Rc::new(one_field)),
        it.some(int(1)),
        it.none(),
        it.ok(text("x")),
        it.err(text("e")),
        Value::Variant(Rc::new(Variant { ctor: leaf, fields: vec![int(1)] })),
        Value::Builtin(lookup("is_even").unwrap()),
        Value::Builtin(lookup("text").unwrap()),
        Value::Builtin(lookup("pow").unwrap()),
    ]
}

/// Built-in functions that read the keyboard, stop the program or write files.
const NOT_CALLED: &[&str] = &["input", "exit", "write_file", "help", "print", "debug"];

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = vec![Vec::new()];
    for _ in 0..n {
        out = out.into_iter().flat_map(|c| (0..k).map(move |i| [c.clone(), vec![i]].concat())).collect();
    }
    out
}

/// For every built-in function the checker describes, with arguments of
/// every kind: when the checker says an argument is never accepted, the
/// function fails with the same message; when it gives a type, the value
/// returned has this type. The same holds when one argument is unknown.
#[test]
fn built_in_functions_do_what_the_checker_says() {
    rythme::lang::set(rythme::lang::Lang::En);
    rythme::io::capture();
    let mut it = Interp::new();
    let values = samples(&it);
    let mut checked = 0;
    for b in BUILTINS {
        if NOT_CALLED.contains(&b.name) {
            continue;
        }
        for n in b.min..=b.max.min(3) {
            for combo in combinations(n, values.len()) {
                let args: Vec<Value> = combo.iter().map(|&i| values[i].clone()).collect();
                let tys: Vec<Ty> = args.iter().map(Ty::of_value).collect();
                let Some(outcome) = builtin_call(b.name, &tys) else { continue };
                let got = (b.func)(&mut it, &args, Span::default());
                let shown: Vec<String> = args.iter().map(|a| a.repr()).collect();
                let call = format!("{}({})", b.name, shown.join(", "));
                match &outcome {
                    Outcome::Wrong { index, expected } => {
                        let err = got.as_ref().err().unwrap_or_else(|| panic!("{call} works, but the checker says it never does"));
                        assert!(matches!(err.code, "E005" | "E023"), "{call}: {}", err.message);
                        if err.code == "E005" {
                            let message =
                                format!("`{}` expects {expected} as argument {}, but got {}", b.name, index + 1, args[*index].describe());
                            assert_eq!(err.message, message, "{call}");
                        }
                    }
                    Outcome::Gives(t) => {
                        if let Ok(v) = &got {
                            assert!(t.contains(v), "{call} gave {}, not of the type {t:?}", v.repr());
                        }
                    }
                }
                // With one argument unknown, the checker knows less, but never wrongly.
                for j in 0..n {
                    let mut partial = tys.clone();
                    partial[j] = Ty::Any;
                    match builtin_call(b.name, &partial) {
                        Some(Outcome::Wrong { .. }) => {
                            assert!(matches!(outcome, Outcome::Wrong { .. }), "{call}: an unknown argument {j} makes it wrong")
                        }
                        Some(Outcome::Gives(t)) => {
                            if let Ok(v) = &got {
                                assert!(t.contains(v), "{call} with argument {j} unknown: {} is not of the type {t:?}", v.repr());
                            }
                        }
                        None => panic!("{}: the checker forgets it", b.name),
                    }
                }
                checked += 1;
            }
        }
        // Used as a value (`map(xs, len)`), the function gives what it gives when called.
        let value = builtin_value(b);
        if (b.min..=b.max).contains(&1) {
            for v in &values {
                if let Ok(r) = (b.func)(&mut it, std::slice::from_ref(v), Span::default()) {
                    assert!(value.ret().contains(&r), "{}({}) as a value", b.name, v.repr());
                }
            }
        }
    }
    rythme::io::take_captured();
    assert!(checked > 10_000, "only {checked} calls checked");
}

// ----- generated programs -------------------------------------------------

/// A small random generator (xorshift), the same on every run.
struct Gen {
    state: u64,
    /// The names defined so far, usable in the next lines.
    vars: Vec<String>,
    /// Only code that runs every time: no `if`, `match`, `and`, `or`, nor functions.
    straight: bool,
}

const HEADER: &str = "type T = A(x) | B
type Pair = Pair(left, right)
fn id(x) = x
fn twice(x) = x + x
fn wrap(x) = [x]
fn pick(c, a, b) = if c then a else b
fn first_or(xs, d) = unwrap_or(first(xs), d)
fn size([]) = 0
fn size([_, ..rest]) = 1 + size(rest)
fn parsed(t)
  n = parse_int(t)?
  Ok(n + 1)
end
fn kind(A(_)) = \"a\"
fn kind(B) = \"b\"
fn fact(0) = 1
fn fact(n) = n * fact(n - 1)
fn swap(Pair(l, r)) = Pair(r, l)
";

/// Built-in functions to call, with their number of arguments.
const CALLS: &[(&str, usize)] = &[
    ("len", 1),
    ("text", 1),
    ("type_of", 1),
    ("upper", 1),
    ("trim", 1),
    ("split", 1),
    ("split", 2),
    ("join", 2),
    ("contains", 2),
    ("starts_with", 2),
    ("replace", 3),
    ("chars", 1),
    ("parse_int", 1),
    ("parse_float", 1),
    ("int", 1),
    ("float", 1),
    ("abs", 1),
    ("min", 2),
    ("max", 1),
    ("sqrt", 1),
    ("floor", 1),
    ("round", 1),
    ("round", 2),
    ("is_even", 1),
    ("sum", 1),
    ("product", 1),
    ("first", 1),
    ("last", 1),
    ("rest", 1),
    ("get", 2),
    ("push", 2),
    ("reverse", 1),
    ("sort", 1),
    ("unique", 1),
    ("slice", 3),
    ("take", 2),
    ("zip", 2),
    ("enumerate", 1),
    ("index_of", 2),
    ("flatten", 1),
    ("is_empty", 1),
    ("keys", 1),
    ("values", 1),
    ("entries", 1),
    ("has", 2),
    ("put", 3),
    ("remove", 2),
    ("unwrap", 1),
    ("unwrap_or", 2),
    ("is_some", 1),
    ("is_ok", 1),
    ("pow", 2),
    ("assert", 1),
    ("fail", 1),
    ("id", 1),
    ("twice", 1),
    ("wrap", 1),
    ("first_or", 2),
    ("size", 1),
    ("parsed", 1),
    ("kind", 1),
    ("fact", 1),
    ("swap", 1),
];

/// Built-in functions that take a function, and how many arguments it receives.
const WITH_FUNCTION: &[(&str, usize)] =
    &[("map", 1), ("filter", 1), ("each", 1), ("any", 1), ("find", 1), ("count", 1), ("sort_by", 1), ("flat_map", 1), ("fold", 2)];

const OPS: &[&str] = &["+", "-", "*", "/", "//", "%", "==", "!=", "<", ">=", "and", "or"];

impl Gen {
    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[self.below(items.len())]
    }

    fn literal(&mut self) -> String {
        self.pick(&["0", "1", "3", "-2", "10", "1.5", "-0.5", "2.0", "\"\"", "\"a\"", "\"42\"", "\"a b\"", "true", "false", "()"])
            .to_string()
    }

    fn expr(&mut self, depth: usize, locals: &[&str]) -> String {
        if depth == 0 {
            return match self.below(4) {
                0 if !locals.is_empty() => locals[self.below(locals.len())].to_string(),
                1 if !self.vars.is_empty() => {
                    let i = self.below(self.vars.len());
                    self.vars[i].clone()
                }
                _ => self.literal(),
            };
        }
        let d = depth - 1;
        let mut choice = self.below(26);
        if self.straight && matches!(choice, 19..=22 | 25) {
            choice = 9;
        }
        match choice {
            0 => self.literal(),
            1 => format!("[{}, {}]", self.expr(d, locals), self.expr(d, locals)),
            2 => format!("[..({}), {}]", self.expr(d, locals), self.expr(d, locals)),
            3 => format!("{{a: {}, b: {}}}", self.expr(d, locals), self.expr(d, locals)),
            4 => format!("{{..({}), a: {}}}", self.expr(d, locals), self.expr(d, locals)),
            5 => format!("({}).{}", self.expr(d, locals), self.pick(&["a", "b", "x", "value", "left"])),
            6 => format!("({})[{}]", self.expr(d, locals), self.expr(d, locals)),
            7 => format!("{}..{}", self.below(4), self.below(6)),
            8 => format!("{}({})", self.pick(&["-", "not "]), self.expr(d, locals)),
            9 | 10 => {
                let op = if self.straight { self.pick(&OPS[..10]) } else { self.pick(OPS) };
                format!("({}) {op} ({})", self.expr(d, locals), self.expr(d, locals))
            }
            11..=13 => {
                let (f, n) = CALLS[self.below(CALLS.len())];
                let args: Vec<String> = (0..n).map(|_| self.expr(d, locals)).collect();
                format!("{f}({})", args.join(", "))
            }
            14 => format!("{}({})", self.pick(&["Some", "Ok", "Err", "A", "wrap"]), self.expr(d, locals)),
            15 => format!("Pair({}, {})", self.expr(d, locals), self.expr(d, locals)),
            16 => self.pick(&["None", "B", "[]", "{}", "pi"]).to_string(),
            17 => format!("\"t{{{}}}\"", self.expr(d, locals)),
            18 => format!("({}).{}()", self.expr(d, locals), self.pick(&["len", "upper", "first", "reverse", "is_empty"])),
            19 => format!("if {} then {} else {}", self.expr(d, locals), self.expr(d, locals), self.expr(d, locals)),
            20 | 21 => {
                let (f, n) = WITH_FUNCTION[self.below(WITH_FUNCTION.len())];
                let list = self.expr(d, locals);
                let params = ["p", "q"];
                let body = self.expr(d, &[locals, &params[..n]].concat());
                let lambda = format!("fn({}) = {body}", params[..n].join(", "));
                if f == "fold" {
                    format!("fold({list}, {}, {lambda})", self.expr(d, locals))
                } else {
                    format!("{f}({list}, {lambda})")
                }
            }
            22 => {
                let pats = ["[h, ..t]", "[]", "Some(h)", "None", "Ok(h)", "A(h)", "B", "{a}", "1", "\"a\"", "true", "Pair(h, t)"];
                let p1 = self.pick(&pats);
                let p2 = self.pick(&pats);
                let names = ["h", "t", "a"];
                let s = self.expr(d, locals);
                let b1 = self.expr(d, &[locals, &names[..]].concat());
                let b2 = self.expr(d, &[locals, &names[..]].concat());
                let b3 = self.expr(d, locals);
                format!("match {s}\n  {p1} -> {b1}\n  {p2} -> {b2}\n  _ -> {b3}\nend")
            }
            23 => format!("({}) |> fn(p) = {}", self.expr(d, locals), self.expr(d, &[locals, &["p"][..]].concat())),
            24 => format!("do\n  w = {}\n  {}\nend", self.expr(d, locals), self.expr(d, &[locals, &["w"][..]].concat())),
            _ => format!("({}) {} ({})", self.expr(d, locals), self.pick(&["and", "or"]), self.expr(d, locals)),
        }
    }
}

/// Runs a generated program until it stops; returns the values of its names.
fn run(items: &[rythme::ast::Item]) -> HashMap<Sym, Value> {
    let mut it = Interp::new();
    rythme::interp::set_max_depth(200);
    let _ = it.exec_items(items, Mode::Run);
    it.globals.into_iter().collect()
}

fn parse(name: &str, code: &str) -> Option<Vec<rythme::ast::Item>> {
    let file = source::add(name, code);
    parser::parse(lexer::lex(code, file).ok()?).ok()
}

/// Thousands of generated programs: every value computed by a line has the
/// type the checker inferred for it, and, in code that runs every time, an
/// error reported by the checker is an error of the program.
#[test]
fn generated_programs_have_the_types_the_checker_infers() {
    rythme::lang::set(rythme::lang::Lang::En);
    let know = Knowledge::from_interp(&Interp::new());
    // `RYTHME_TYPE_PROGRAMS=100000 cargo test --release --test types` tries more.
    let programs: u64 = std::env::var("RYTHME_TYPE_PROGRAMS").ok().and_then(|n| n.parse().ok()).unwrap_or(600);
    let header_lines = HEADER.lines().count();
    let mut typed = 0;
    let mut known = 0;
    let mut parsed = 0;
    let mut found = 0;
    for seed in 1..=programs {
        let mut g = Gen { state: seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1, vars: Vec::new(), straight: seed % 3 == 0 };
        // Each definition is kept if the program still runs to its end, so
        // that most of them run; the last one may fail.
        let mut code = HEADER.to_string();
        let mut names = Vec::new();
        // The line where each definition starts.
        let mut starts = Vec::new();
        for i in 0..8 {
            let name = format!("v{i}");
            for attempt in 0..6 {
                let depth = 1 + g.below(3);
                let line = format!("{name} = {}\n", g.expr(depth, &[]));
                let candidate = format!("{code}{line}");
                let Some(items) = parse("candidate.ry", &candidate) else { continue };
                rythme::io::capture();
                let runs = run(&items).contains_key(&Sym::new(&name));
                rythme::io::take_captured();
                if runs || attempt == 5 {
                    starts.push(code.lines().count() + 1);
                    code = candidate;
                    names.push(name.clone());
                    g.vars.push(name);
                    break;
                }
            }
        }
        let Some(items) = parse(&format!("gen{seed}.ry"), &code) else { continue };
        parsed += 1;
        let types = check::top_level_types(&items, &know);
        rythme::io::capture();
        let values = run(&items);
        let diags = check::check(&items, &know, false);
        rythme::io::take_captured();
        // The lines that ran: their names have a value.
        let ran = names.iter().take_while(|n| values.contains_key(&Sym::new(n))).count();
        for n in &names[..ran] {
            let s = Sym::new(n);
            let (t, v) = (&types[&s], &values[&s]);
            assert!(t.contains(v), "program {seed}: `{n}` is {}, not of the type {t:?}\n{code}", v.repr());
            typed += 1;
            if *t != Ty::Any {
                known += 1;
            }
        }
        if g.straight {
            for d in diags.iter().filter(|d| d.severity == Severity::Error) {
                let Some(span) = d.span else { continue };
                let line = source::get(span.file).unwrap().line_col(span.start as usize).0;
                // The header has no mistakes.
                assert!(line > header_lines, "program {seed}: error in the header: {}\n{code}", d.message);
                let index = starts.iter().rposition(|&s| s <= line).unwrap_or(0);
                assert!(index >= ran, "program {seed}: line {line} runs, but the checker reports: {}\n{code}", d.message);
                found += 1;
            }
        }
    }
    eprintln!("{typed} values checked, {known} with a known type, {found} errors found before running");
    assert!(parsed * 100 > programs * 95, "only {parsed} programs could be read");
    assert!(typed > programs * 4, "only {typed} values checked");
    assert!(known * 2 > typed, "only {known} of {typed} values have a known type");
    assert!(found * 30 > programs, "only {found} errors found before running");
}
