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
    let mut date = Record::new();
    for (k, v) in [("year", 2024), ("month", 2), ("day", 29)] {
        date.set(Rc::from(k), int(v));
    }
    let leaf = Rc::new(CtorInfo { type_name: Sym::new("T"), name: Sym::new("A"), fields: vec![Sym::new("x")], index: 0, doc: None });
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
        Value::Record(Rc::new(date)),
        Value::dict(Vec::new()),
        Value::dict(vec![(int(1), text("a")), (int(2), text("b"))]),
        Value::dict(vec![(text("a"), int(1))]),
        Value::set(Vec::new()),
        Value::set(vec![int(1), int(2)]),
        Value::set(vec![text("x")]),
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
const NOT_CALLED: &[&str] = &["input", "exit", "write_file", "append_file", "help", "print", "debug"];

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
    /// The functions of the program, and their number of parameters.
    fns: Vec<(String, usize)>,
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
    ("gcd", 2),
    ("capitalize", 1),
    ("pad_left", 2),
    ("char_code", 1),
    ("is_digit", 1),
    ("chunks", 2),
    ("windows", 2),
    ("step_by", 2),
    ("set_at", 3),
    ("remove_at", 2),
    ("from_entries", 1),
    ("dict", 1),
    ("set", 1),
    ("union", 2),
    ("intersection", 2),
    ("difference", 2),
    ("is_subset", 2),
    ("ok_or", 2),
    ("parse_json", 1),
    ("to_json", 1),
    ("assert", 1),
    ("assert_eq", 2),
    ("print", 1),
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
const WITH_FUNCTION: &[(&str, usize)] = &[
    ("map", 1),
    ("filter", 1),
    ("each", 1),
    ("any", 1),
    ("find", 1),
    ("count", 1),
    ("sort_by", 1),
    ("flat_map", 1),
    ("fold", 2),
    ("scan", 2),
    ("take_while", 1),
    ("min_by", 1),
    ("map_values", 1),
    ("and_then", 1),
    ("map_err", 1),
];

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
        let mut choice = self.below(31);
        if self.straight && matches!(choice, 19..=22 | 25 | 29) || (choice == 30 && self.fns.is_empty()) {
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
                if n == 2 {
                    format!("{f}({list}, {}, {lambda})", self.expr(d, locals))
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
            25 => format!("({}) {} ({})", self.expr(d, locals), self.pick(&["and", "or"]), self.expr(d, locals)),
            26 => format!("do\n  fn g(p) = {}\n  g({})\nend", self.expr(d, &[locals, &["p"][..]].concat()), self.expr(d, locals)),
            27 => format!("({{f: fn(p) = {}, a: 1}}).f({})", self.expr(d, &[locals, &["p"][..]].concat()), self.expr(d, locals)),
            28 => match self.below(2) {
                0 => format!("({})[\"{}\"]", self.expr(d, locals), self.pick(&["a", "b", "z"])),
                _ => format!("get({}, \"{}\")", self.expr(d, locals), self.pick(&["a", "b", "z"])),
            },
            29 => {
                let pats = ["Some([h, ..t])", "Pair(A(h), t)", "{a: [h, ..t]}", "Ok(Some(h))", "[Some(h), ..t]", "Err(h)", "[[h], t]"];
                let p1 = self.pick(&pats);
                let p2 = self.pick(&pats);
                let names = ["h", "t"];
                let s = self.expr(d, locals);
                let b1 = self.expr(d, &[locals, &names[..]].concat());
                let b2 = self.expr(d, &[locals, &names[..]].concat());
                format!("match {s}\n  {p1} -> {b1}\n  {p2} -> {b2}\n  other -> {}\nend", self.expr(d, &[locals, &["other"][..]].concat()))
            }
            _ => {
                let i = self.below(self.fns.len());
                let (f, n) = self.fns[i].clone();
                let args: Vec<String> = (0..n).map(|_| self.expr(d, locals)).collect();
                format!("{f}({})", args.join(", "))
            }
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

/// Runs a test on a thread with a large stack, like the `rythme` command
/// does: generated programs may recurse deeply.
fn on_large_stack(test: fn()) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(test).unwrap().join().unwrap();
}

/// Thousands of generated programs: every value computed by a line has the
/// type the checker inferred for it, and, in code that runs every time, an
/// error reported by the checker is an error of the program.
#[test]
fn generated_programs_have_the_types_the_checker_infers() {
    on_large_stack(generated_programs);
}

fn generated_programs() {
    rythme::lang::set(rythme::lang::Lang::En);
    let know = Knowledge::from_interp(&Interp::new());
    // `RYTHME_TYPE_PROGRAMS=100000 cargo test --release --test types` tries more.
    let programs: u64 = std::env::var("RYTHME_TYPE_PROGRAMS").ok().and_then(|n| n.parse().ok()).unwrap_or(400);
    let header_lines = HEADER.lines().count();
    let mut typed = 0;
    let mut known = 0;
    let mut parsed = 0;
    let mut found = 0;
    for seed in 1..=programs {
        let mut g = Gen { state: seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1, vars: Vec::new(), straight: seed % 3 == 0, fns: Vec::new() };
        let mut code = HEADER.to_string();
        // The program may define a type of its own (named like `Result`, a
        // type of Rythme, it would be an error).
        if seed % 4 == 1 {
            code.push_str("type Outcome = Win | Lose(why)\n");
            g.vars.push("Win".into());
        }
        // Functions with any body, recursive or not (their code does not run every time).
        if !g.straight {
            let body = g.expr(2, &["p", "q"]);
            code.push_str(&format!("fn h0(p, q) = {body}\n"));
            let body = g.expr(2, &["p"]);
            code.push_str(&format!("fn h1(p)\n  w = ({body})?\n  Ok(w)\nend\n"));
            let body = g.expr(2, &["p"]);
            code.push_str(&format!("fn h2(0) = []\nfn h2(p) = [{body}, ..h2(p - 1)]\n"));
            g.fns = vec![("h0".into(), 2), ("h1".into(), 1), ("h2".into(), 1)];
        }
        // Functions whose whole body runs every time they are called: the
        // lines where each one is.
        let mut bodies: Vec<(String, std::ops::RangeInclusive<usize>)> = Vec::new();
        if g.straight {
            let first = code.lines().count() + 1;
            let body = g.expr(2, &["p", "q"]);
            code.push_str(&format!("fn s0(p, q) = {body}\n"));
            bodies.push(("s0(".into(), first..=code.lines().count()));
            let first = code.lines().count() + 1;
            let (a, b) = (g.expr(2, &["p"]), g.expr(2, &["p", "w"]));
            code.push_str(&format!("fn s1(p)\n  w = {a}\n  {b}\nend\n"));
            bodies.push(("s1(".into(), first..=code.lines().count()));
            g.fns = vec![("s0".into(), 2), ("s1".into(), 1)];
        }
        // Each definition is kept if the program still runs to its end, so
        // that most of them run; the last one may fail.
        let mut names = Vec::new();
        let mut texts = Vec::new();
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
                    texts.push(line);
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
                // In a function whose whole body ran: called by a line that ran.
                if let Some((call, _)) = bodies.iter().find(|(_, lines)| lines.contains(&line)) {
                    let called = texts[..ran].iter().any(|t| t.contains(call.as_str()));
                    assert!(!called, "program {seed}: line {line} runs, but the checker reports: {}\n{code}", d.message);
                    found += 1;
                    continue;
                }
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

/// The warnings say the truth: a comparison said to be always false (or
/// true) is false (or true) when it runs, and a case said never to be
/// reached is never the one taken.
#[test]
fn warnings_about_comparisons_and_cases_are_true() {
    on_large_stack(warnings_are_true);
}

fn warnings_are_true() {
    rythme::lang::set(rythme::lang::Lang::En);
    let know = Knowledge::from_interp(&Interp::new());
    let header_lines = HEADER.lines().count();
    let programs: u64 = std::env::var("RYTHME_TYPE_PROGRAMS").ok().and_then(|n| n.parse().ok()).unwrap_or(400) * 2;
    let pats = ["[h, ..t]", "[]", "Some(h)", "None", "Ok(h)", "A(h)", "B", "{a}", "1", "\"a\"", "true", "Pair(h, t)", "h", "()"];
    let (mut comparisons, mut cases) = (0, 0);
    for seed in 1..=programs {
        let mut g = Gen { state: seed.wrapping_mul(0x2545_F491_4F6C_DD1D) | 1, vars: Vec::new(), straight: true, fns: Vec::new() };
        let (left, right) = (g.expr(2, &[]), g.expr(2, &[]));
        let op = g.pick(&["==", "!="]);
        let subject = g.expr(2, &[]);
        let arms: Vec<&str> = (0..3).map(|_| g.pick(&pats)).collect();
        let compare = format!("c = ({left}) {op} ({right})\n");
        let mut code = format!("{HEADER}{compare}m = match {subject}\n");
        let mut arm_lines = Vec::new();
        for (i, p) in arms.iter().enumerate() {
            arm_lines.push(code.lines().count() + 1);
            code.push_str(&format!("  {p} -> {i}\n"));
        }
        code.push_str("  _ -> 9\nend\n");
        let op_at = HEADER.len() + format!("c = ({left}) ").len();
        let Some(items) = parse(&format!("warn{seed}.ry"), &code) else { continue };
        rythme::io::capture();
        let values = run(&items);
        let diags = check::check(&items, &know, false);
        rythme::io::take_captured();
        for d in diags.iter().filter(|d| d.severity == Severity::Warning) {
            let Some(span) = d.span else { continue };
            if d.code == "W006" && span.start as usize == op_at {
                if let Some(v) = values.get(&Sym::new("c")) {
                    assert!(matches!(v, Value::Bool(b) if *b == (op == "!=")), "program {seed}: `{op}` gave {}\n{code}", v.repr());
                    comparisons += 1;
                }
            }
            let line = source::get(span.file).unwrap().line_col(span.start as usize).0;
            if d.code == "W004" && line > header_lines {
                if let (Some(i), Some(Value::Int(taken))) = (arm_lines.iter().position(|&l| l == line), values.get(&Sym::new("m"))) {
                    assert_ne!(*taken as usize, i, "program {seed}: the case said never reached was taken\n{code}");
                    cases += 1;
                }
            }
        }
    }
    eprintln!("{comparisons} comparisons and {cases} cases checked");
    assert!(comparisons * 50 > programs && cases * 50 > programs, "only {comparisons} comparisons and {cases} cases checked");
}

/// For mistakes written with values in the code, the checker reports the
/// error the program would stop with: same code, same message, and every
/// piece of help the run time gives (so that `rythme explain` applies).
#[test]
fn errors_found_before_running_are_those_of_the_run_time() {
    rythme::lang::set(rythme::lang::Lang::En);
    let know = Knowledge::from_interp(&Interp::new());
    let cases = [
        "x = \"age: \" + 36",
        "x = 1 + [2]",
        "x = [1] + 2",
        "x = [1] - 2",
        "x = \"a\" * 3",
        "x = true < 1",
        "x = 1 < \"a\"",
        "x = {a: 1} < {a: 2}",
        "x = -\"a\"",
        "x = not 1",
        "x = if 1 then 2 else 3",
        "x = 1.5..3",
        "x = [1, 2][\"a\"]",
        "x = 5[0]",
        "x = {a: 1}.b",
        "x = {name: 1}.nmae",
        "x = 5.a",
        "x = len(5)",
        "x = upper(3)",
        "x = map(5, text)",
        "x = split(\"a\", 1)",
        "x = [..5]",
        "x = None < Ok(1)",
        "x = map(\"abc\", upper)",
        "x = filter({a: 1}, fn(v) = true)",
        "x = put([1, 2], 0, 5)",
        "x = remove([1, 2], 0)",
        "x = flat_map({a: 1}, fn(v) = [v])",
        "x = gcd(12, 1.5)",
        "x = pad_left(\"7\", \"3\")",
        "x = text(\"a\", 2)",
        "x = map_err([1], upper)",
        "x = ok_or(\"a\", \"e\")",
        "x = [1] != {a: 1} and len([1, 2]) < [1]",
        "x = has([1, 2], \"a\")",
        "x = min_by(\"ab\", len)",
        "x = set_at({a: 1}, 0, 1)",
        "x = and_then([1], fn(v) = Some(v))",
        "x = take_while(5, is_odd)",
        "x = pad_left(1, 2, 3)",
        "x = to_json(1, \"a\")",
        "x = fold([1], 0, fn(a) = a)",
        "x = map([1], fn(a, b) = a)",
        "type T = A(v) | B\nx = A(1).w",
        "fn f() = 5?\nx = f()",
        "x = match \"abc\"\n  [h, ..t] -> 1\nend",
        "[a, b] = 5",
        "fn f(0) = 1\nx = f(\"a\")",
        "fn f(a) = a\nx = f(1, 2)",
    ];
    for code in cases {
        let items = parse("same.ry", code).unwrap();
        let found = check::check(&items, &know, false).into_iter().find(|d| d.is_error());
        let found = found.unwrap_or_else(|| panic!("not found before running:\n{code}"));
        rythme::io::capture();
        let stopped = Interp::new().exec_items(&items, Mode::Run).err();
        rythme::io::take_captured();
        let stopped = stopped.unwrap_or_else(|| panic!("runs without error:\n{code}"));
        assert_eq!((found.code, &found.message), (stopped.code, &stopped.message), "{code}");
        for help in &stopped.helps {
            assert!(found.helps.contains(help), "{code}: the help `{help}` is missing before running");
        }
    }
}
