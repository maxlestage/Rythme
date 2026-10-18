//! Fuzzing: thousands of random programs go through the lexer, the parser, the
//! checker and the interpreter. None may make Rythme panic or hang, and every
//! message must point at a real place of the code and render without panicking.
//!
//! The programs are random tokens, programs built from the grammar, and
//! mutations of the examples and of the code in the tests. Everything is
//! deterministic (fixed seed). A longer run, in release mode:
//!
//! ```sh
//! RYTHME_FUZZ_CASES=1000000 RYTHME_FUZZ_SEED=7 cargo test --release --test fuzz -- --ignored
//! ```

use std::panic::AssertUnwindSafe;
use std::path::Path;
use std::rc::Rc;
use std::sync::OnceLock;

use rythme::ast::Item;
use rythme::builtins::{self, Builtin};
use rythme::check::{self, Knowledge};
use rythme::diagnostic::{Diagnostic, Fallible};
use rythme::interp::{Interp, Mode};
use rythme::lang::{self, Lang};
use rythme::source::{self, Span};
use rythme::symbol::Sym;
use rythme::value::{Record, Value};
use rythme::{help, io, lexer, parser, steps};

/// Calls allowed to each program: enough to do real work, few enough to
/// stop programs that never end in a fraction of a second.
const STEPS: u64 = 3000;

// ----- random numbers ----------------------------------------------------

struct Rng(u64);

impl Rng {
    /// splitmix64.
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[self.below(items.len())]
    }
}

// ----- random tokens -----------------------------------------------------

const KEYWORDS: &[&str] = &["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false"];

const WORDS: &[&str] = &[
    "x",
    "y",
    "n",
    "xs",
    "acc",
    "rest",
    "f",
    "g",
    "_",
    "a",
    "b",
    "print",
    "len",
    "map",
    "filter",
    "fold",
    "sum",
    "sort",
    "text",
    "help",
    "debug",
    "split",
    "join",
    "get",
    "first",
    "range",
    "int",
    "float",
    "round",
    "pow",
    "abs",
    "repeat",
    "unwrap",
    "Some",
    "None",
    "Ok",
    "Err",
    "Circle",
    "Leaf",
    "Node",
    "T",
    "for",
    "while",
    "return",
    "let",
    "null",
    "class",
    "def",
    "élément",
    "日本",
    "x_1",
    "_unused",
    "lenght",
];

const NUMBERS: &[&str] = &[
    "0",
    "1",
    "2",
    "3",
    "10",
    "-1",
    "0.5",
    "3.14",
    "1e3",
    "1e400",
    "0.0",
    "-0.0",
    "9223372036854775807",
    "9223372036854775808",
    "-9223372036854775808",
    "0x1f",
    "0xFFFFFFFFFFFFFFFF",
    "0b101",
    "0b2",
    "0x",
    "1_000",
    "1e",
    "1.",
    "007",
    "2.5e-3",
];

const TEXTS: &[&str] = &[
    "\"a\"",
    "\"\"",
    "\"{x}\"",
    "\"é\"",
    "\"\\n\"",
    "\"{\"",
    "\"}\"",
    "\"\\{\"",
    "\"{1 + 2}\"",
    "\"{\"in\"}\"",
    "\"unclosed",
    "\"{x",
    "\"\\q\"",
    "\"\\u{e9}\"",
    "\"\\u{110000}\"",
    "\"👍🏽\"",
    "\"a{}b\"",
    "\"{ # c }\"",
    "\"{\"{\"{1}\"}\"}\"",
    "\"line\nbreak\"",
];

const SYMBOLS: &[&str] = &[
    "+",
    "-",
    "*",
    "/",
    "//",
    "%",
    "**",
    "==",
    "!=",
    "<",
    "<=",
    ">",
    ">=",
    "|>",
    "->",
    "=>",
    "=",
    "..",
    "...",
    ".",
    ",",
    ":",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    "|",
    "?",
    "!",
    "&&",
    "||",
    "+=",
    "++",
    ";",
    "# comment",
    "## doc",
    "&",
    "@",
    "$",
    "^",
    "`",
    "'",
    "“",
];

const SPACES: &[&str] = &[" ", "", "\n", "  ", "\t", "\r\n", "\n\n", "\u{feff}", "\u{301}", "\0", "\r", "é", "👍"];

fn token(rng: &mut Rng) -> &'static str {
    match rng.below(10) {
        0..=1 => rng.pick(KEYWORDS),
        2..=4 => rng.pick(WORDS),
        5 => rng.pick(NUMBERS),
        6 => rng.pick(TEXTS),
        _ => rng.pick(SYMBOLS),
    }
}

fn token_soup(rng: &mut Rng) -> String {
    let mut s = String::new();
    for _ in 0..1 + rng.below(60) {
        s.push_str(token(rng));
        s.push_str(if rng.chance(70) { " " } else { rng.pick(SPACES) });
    }
    s
}

// ----- programs built from the grammar --------------------------------------

/// Builds random programs that mostly follow the grammar, so that many of
/// them pass the checker and run.
struct Gen<'a> {
    rng: &'a mut Rng,
    fns: Vec<(String, usize)>,
    fresh: usize,
}

const VALUES: &[&str] = &["x", "y", "n", "xs", "s", "r", "o"];
const PREAMBLE: &str = "x = 3\ny = -2.5\nn = 0\nxs = [1, 2, 3]\ns = \"héllo 👍\"\nr = {name: \"Ada\", age: 36}\no = Some(1)\n";

impl Gen<'_> {
    fn name(&mut self) -> String {
        self.fresh += 1;
        format!("v{}", self.fresh)
    }

    fn atom(&mut self) -> String {
        let r = &mut *self.rng;
        match r.below(14) {
            0 => format!("{}", r.below(12) as i64 - 3),
            1 => r.pick(&["0.5", "-1.5", "1e300", "2.0", "0.0", "float(\"nan\")", "1e-320"]).into(),
            2 => r.pick(&["9223372036854775807", "-9223372036854775808", "4611686018427387904"]).into(),
            3 => r.pick(&["\"\"", "\"a\"", "\"héllo\"", "\"👍🏽\"", "\"a,b,,c\"", "\"e\\u{301}\"", "\" x \"", "\"ß\""]).into(),
            4 => r.pick(&["true", "false", "()", "None", "[]", "{}"]).into(),
            5..=8 => r.pick(VALUES).into(),
            9 => r.pick(&["len", "print", "abs", "is_even", "upper", "sum", "fn(v) = v", "Some", "Ok"]).into(),
            10 => r.pick(&["A", "B(1)", "C(2, \"z\")", "Leaf", "Node(Leaf, 1, Leaf)"]).into(),
            _ => r.pick(&["1", "2", "\"b\"", "[1, \"a\"]", "-1"]).into(),
        }
    }

    fn args(&mut self, n: usize, depth: usize) -> String {
        (0..n).map(|_| self.expr(depth)).collect::<Vec<_>>().join(", ")
    }

    fn expr(&mut self, depth: usize) -> String {
        if depth == 0 || self.rng.chance(25) {
            return self.atom();
        }
        let d = depth - 1;
        match self.rng.below(22) {
            0..=2 => {
                let op = self.rng.pick(&["+", "-", "*", "/", "//", "%", "**", "==", "!=", "<", "<=", ">", ">=", "and", "or", "+"]);
                format!("{} {op} {}", self.expr(d), self.expr(d))
            }
            3 => format!("({})", self.expr(d)),
            4 => format!("{}{}", self.rng.pick(&["-", "not ", "- "]), self.expr(d)),
            5 | 6 => {
                let b = &builtins::BUILTINS[self.rng.below(builtins::BUILTINS.len())];
                let n = if self.rng.chance(85) { b.min + self.rng.below((b.max.min(b.min + 3) - b.min) + 1) } else { self.rng.below(4) };
                format!("{}({})", b.name, self.args(n, d))
            }
            7 => match self.fns.len() {
                0 => self.atom(),
                k => {
                    let (name, arity) = self.fns[self.rng.below(k)].clone();
                    let n = if self.rng.chance(90) { arity } else { self.rng.below(4) };
                    format!("{name}({})", self.args(n, d))
                }
            },
            8 => format!("fn({}) = {}", self.rng.pick(&["v", "a, b", "", "[h, ..t]", "_"]), self.expr(d)),
            9 => format!("if {} then {} else {}", self.expr(d), self.expr(d), self.expr(d)),
            10 => {
                let mut s = format!("match {}\n", self.expr(d));
                for _ in 0..1 + self.rng.below(4) {
                    let guard = if self.rng.chance(20) { format!(" if {}", self.expr(d)) } else { String::new() };
                    s.push_str(&format!("  {}{guard} -> {}\n", self.pattern(2), self.expr(d)));
                }
                if self.rng.chance(60) {
                    s.push_str(&format!("  _ -> {}\n", self.expr(d)));
                }
                s.push_str("end");
                s
            }
            11 => {
                let n = self.rng.below(4);
                let spread = if self.rng.chance(20) { format!(", ..{}", self.expr(d)) } else { String::new() };
                format!("[{}{spread}]", self.args(n, d))
            }
            12 => {
                let k = self.rng.pick(&["name", "age", "a", "type", "\"my key\""]);
                let spread = if self.rng.chance(25) { format!(", ..{}", self.rng.pick(&["r", "{}", "xs"])) } else { String::new() };
                format!("{{{k}: {}{spread}}}", self.expr(d))
            }
            13 => format!("{}.{}", self.expr(d), self.rng.pick(&["name", "age", "x", "len()", "upper()", "map(fn(v) = v)", "value"])),
            14 => format!("{}[{}]", self.expr(d), self.expr(d)),
            15 => {
                let f = self.rng.pick(&["len", "sort", "reverse", "map(fn(v) = v)", "filter(is_even)", "text", "fn(v) = [v]", "first"]);
                format!("{} |> {f}", self.expr(d))
            }
            16 => format!("\"{}{{{}}}{}\"", self.rng.pick(&["", "a ", "é"]), self.expr(d), self.rng.pick(&["", " b", "\\{"])),
            17 => format!("{}..{}", self.expr(d), self.expr(d)),
            18 => format!("{}?", self.expr(d)),
            19 => {
                let v = self.name();
                format!("do\n  {v} = {}\n  {}\nend", self.expr(d), self.rng.pick(&[v.as_str(), "print(1)", "x"]))
            }
            _ => self.atom(),
        }
    }

    fn pattern(&mut self, depth: usize) -> String {
        let leaf = ["_", "p", "0", "-1", "1.5", "\"a\"", "true", "()", "None", "A", "-9223372036854775808"];
        if depth == 0 {
            return self.rng.pick(&leaf).into();
        }
        let d = depth - 1;
        match self.rng.below(10) {
            0..=2 => self.rng.pick(&leaf).into(),
            3 => format!("Some({})", self.pattern(d)),
            4 => format!("{}({})", self.rng.pick(&["Ok", "Err", "B"]), self.pattern(d)),
            5 => "[]".into(),
            6 => format!("[{}, ..{}]", self.pattern(d), self.rng.pick(&["t", "_", ""])),
            7 => format!("[{}, {}]", self.pattern(d), self.pattern(d)),
            8 => format!("{{{}}}", self.rng.pick(&["name", "age: q", "name, ..", "\"my key\": k"])),
            _ => format!("{} | {}", self.rng.pick(&leaf), self.rng.pick(&leaf)),
        }
    }

    fn item(&mut self) -> String {
        match self.rng.below(10) {
            0..=2 => {
                let name = format!("f{}", self.fns.len());
                let arity = self.rng.below(3);
                let params: Vec<String> = (0..arity).map(|i| ["p", "q", "w"][i].to_string()).collect();
                self.fns.push((name.clone(), arity));
                let body = self.expr(3);
                if arity == 1 && self.rng.chance(40) {
                    // One definition per case, then a recursive general case.
                    let base = self.pattern(1);
                    format!("fn {name}({base}) = {}\nfn {name}(p) = {name}(p - 1)", self.expr(2))
                } else if self.rng.chance(30) {
                    format!("fn {name}({})\n  {} = {}\n  {}\nend", params.join(", "), self.name(), self.expr(2), body)
                } else {
                    format!("fn {name}({}) = {body}", params.join(", "))
                }
            }
            3..=5 => format!("print({})", self.expr(3)),
            6 => format!("{} = {}", self.name(), self.expr(3)),
            7 => {
                let name = self.name();
                format!("test \"{name}\" = {}", self.expr(2))
            }
            8 => format!("[{}, {}] = {}", self.name(), self.name(), self.expr(2)),
            _ => self.expr(3),
        }
    }

    fn program(&mut self) -> String {
        let mut s = String::from(PREAMBLE);
        s.push_str("type T = A | B(v) | C(v, w)\ntype Tree = Leaf | Node(l, v, r)\n");
        for _ in 0..1 + self.rng.below(6) {
            s.push_str(&self.item());
            s.push('\n');
        }
        s
    }
}

// ----- built-in functions on edge cases -------------------------------------

/// Values at the limits: the smallest and largest whole numbers, numbers
/// that decimals cannot hold exactly, `nan`, empty and accented texts...
const EDGES: &[&str] = &[
    "0",
    "1",
    "-1",
    "2",
    "7",
    "-7",
    "9223372036854775807",
    "-9223372036854775808",
    "4611686018427387904",
    "9007199254740993",
    "2147483648",
    "0.0",
    "-0.0",
    "0.5",
    "-2.5",
    "1e300",
    "1e-320",
    "1e16",
    "9007199254740992.0",
    "float(\"nan\")",
    "1e400",
    "-1e400",
    "123.456",
    "\"\"",
    "\"a\"",
    "\"ß\"",
    "\"e\\u{301}\"",
    "\"👍🏽\"",
    "\"日本\"",
    "\" a b \"",
    "\"a,b\"",
    "\"42\"",
    "\"-1.5e3\"",
    "\"nan\"",
    "\"9223372036854775808\"",
    "\"\\n\"",
    "\"\\{\"",
    "[]",
    "[1, 2, 3]",
    "[3, 1, 2]",
    "[1, \"a\"]",
    "[[1], [2, 3]]",
    "[1.5, 1]",
    "[float(\"nan\"), 1.0]",
    "[\"b\", \"a\"]",
    "[None, Some(1)]",
    "1..5",
    "[[]]",
    "{}",
    "{a: 1}",
    "{\"my key\": [1]}",
    "{year: 2024, month: 2, day: 29}",
    "{year: 9999, month: 12, day: 31, hour: 23, minute: 59, second: 59, offset: -60}",
    "\"2024-02-29T12:00:00+01:00\"",
    "r",
    "None",
    "Some(0)",
    "Ok(\"x\")",
    "Err(1)",
    "()",
    "true",
    "false",
    "len",
    "fn(v) = v",
    "fn(a, b) = a",
    "is_even",
    "xs",
    "s",
];

/// An expression on edge cases. The checker knows the kinds of the values
/// written in the code, and stops most wrong calls with them before running.
/// When `hidden`, every value goes through `hide`, whose result the checker
/// does not know: these mistakes are found while running.
fn edge_expr(rng: &mut Rng, depth: usize, hidden: bool) -> String {
    if depth == 0 || rng.chance(35) {
        let edge = rng.pick(EDGES);
        return if hidden { format!("hide({edge})") } else { edge.to_string() };
    }
    let d = depth - 1;
    let value = match rng.below(10) {
        0..=5 => {
            let b = &builtins::BUILTINS[rng.below(builtins::BUILTINS.len())];
            let n = if rng.chance(90) { b.min + rng.below(b.max.min(b.min + 2) - b.min + 1) } else { rng.below(4) };
            let args: Vec<String> = (0..n).map(|_| edge_expr(rng, d, hidden)).collect();
            format!("{}({})", b.name, args.join(", "))
        }
        6 | 7 => {
            let op = rng.pick(&["+", "-", "*", "/", "//", "%", "**", "==", "!=", "<", "<=", ">", ">=", ".."]);
            format!("({} {op} {})", edge_expr(rng, d, hidden), edge_expr(rng, d, hidden))
        }
        8 => format!("-({})", edge_expr(rng, d, hidden)),
        _ => format!("{}[{}]", edge_expr(rng, d, hidden), edge_expr(rng, d, hidden)),
    };
    if hidden {
        format!("hide({value})")
    } else {
        value
    }
}

/// Half of these programs are checked with their values in plain sight, the
/// other half run with their values hidden from the checker.
fn edge_program(rng: &mut Rng) -> String {
    let hidden = rng.chance(50);
    format!("{PREAMBLE}fn hide(v) = v\nprint({})\n", edge_expr(rng, 3, hidden))
}

// ----- mutations of existing code -------------------------------------------

fn seeds() -> &'static Vec<String> {
    static SEEDS: OnceLock<Vec<String>> = OnceLock::new();
    SEEDS.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut out = Vec::new();
        for dir in ["examples", "examples/modules"] {
            let mut files: Vec<_> = std::fs::read_dir(root.join(dir)).unwrap().flatten().map(|e| e.path()).collect();
            files.sort();
            for f in files.into_iter().filter(|f| f.extension().is_some_and(|e| e == "ry")) {
                out.push(std::fs::read_to_string(f).unwrap());
            }
        }
        let mut tests: Vec<_> = std::fs::read_dir(root.join("tests")).unwrap().flatten().map(|e| e.path()).collect();
        tests.sort();
        for f in tests.into_iter().filter(|f| f.extension().is_some_and(|e| e == "rs")) {
            out.extend(code_in_tests(&std::fs::read_to_string(f).unwrap()));
        }
        assert!(out.len() > 50, "too few seeds: {}", out.len());
        out
    })
}

/// The Rythme code given to `ok(...)`, `fails(...)`, `warns(...)` and `-e` in the tests.
fn code_in_tests(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for marker in ["ok(\"", "fails(\"", "warns(\"", "\"-e\", \""] {
        let mut rest = src;
        while let Some(i) = rest.find(marker) {
            rest = &rest[i + marker.len()..];
            let mut code = String::new();
            let mut chars = rest.chars();
            while let Some(c) = chars.next() {
                match c {
                    '"' => break,
                    '\\' => match chars.next() {
                        Some('n') => code.push('\n'),
                        Some('t') => code.push('\t'),
                        Some('r') => code.push('\r'),
                        Some('0') => code.push('\0'),
                        Some('u') => {
                            let hex: String = chars.by_ref().skip(1).take_while(|&c| c != '}').collect();
                            code.extend(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32));
                        }
                        Some(other) => code.push(other),
                        None => break,
                    },
                    c => code.push(c),
                }
            }
            if !code.is_empty() {
                out.push(code);
            }
        }
    }
    out
}

/// The start of the character at byte `i` of `s`.
fn floor(s: &str, mut i: usize) -> usize {
    while !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// A random position in `s` where a character starts.
fn boundary(rng: &mut Rng, s: &str) -> usize {
    floor(s, rng.below(s.len() + 1))
}

fn mutate(rng: &mut Rng, src: &str) -> String {
    let mut s = src.to_string();
    for _ in 0..1 + rng.below(4) {
        let a = boundary(rng, &s);
        let b = boundary(rng, &s);
        let (lo, hi) = (a.min(b), a.max(b));
        // Short ranges: mutations stay local.
        let hi = floor(&s, hi.min(lo + 1 + rng.below(40)));
        match rng.below(9) {
            0 => s.replace_range(lo..hi, ""),
            1 => {
                let part = s[lo..hi].to_string();
                s.insert_str(hi, &part);
            }
            2 | 3 => s.insert_str(a, token(rng)),
            4 => s.replace_range(lo..hi, token(rng)),
            5 => s.insert_str(a, rng.pick(SPACES)),
            6 => s.truncate(a),
            7 => {
                let other = &seeds()[rng.below(seeds().len())];
                let x = boundary(rng, other);
                let y = boundary(rng, other);
                s.insert_str(a, &other[x.min(y)..x.max(y)]);
            }
            _ => {
                // Swap two lines.
                let mut lines: Vec<&str> = s.split('\n').collect();
                if lines.len() > 1 {
                    let i = rng.below(lines.len());
                    let j = rng.below(lines.len());
                    lines.swap(i, j);
                }
                s = lines.join("\n");
            }
        }
    }
    s
}

/// Mutations may create huge numbers: a range or a `repeat` of billions of
/// elements is refused with an error, but tens of millions are allowed and
/// would make the fuzzing slow. Whole numbers between 10 000 and 10^15 are
/// cut down.
fn tame(src: String) -> String {
    let mut out = String::with_capacity(src.len());
    let mut digits = String::new();
    let flush = |digits: &mut String, out: &mut String| {
        if digits.len() > 4 && digits.len() < 16 {
            out.push_str(&digits[..2]);
        } else {
            out.push_str(digits);
        }
        digits.clear();
    };
    for c in src.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            flush(&mut digits, &mut out);
            out.push(c);
        }
    }
    flush(&mut digits, &mut out);
    out
}

// ----- running a case --------------------------------------------------------

/// Built-in functions that touch the world outside of the program are
/// replaced: no files, no keyboard, no `exit`, no output, no clock (`time`,
/// `now` and `today` give fixed values), no environment variables.
fn sandbox() -> &'static Vec<&'static Builtin> {
    static SANDBOX: OnceLock<Vec<&'static Builtin>> = OnceLock::new();
    fn quiet(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
        for v in a {
            let _ = v.display();
        }
        Ok(Value::Unit)
    }
    fn same(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
        let _ = a[0].repr();
        Ok(a[0].clone())
    }
    fn empty_text(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(Value::text(""))
    }
    fn no_file(it: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(it.err(Value::text("no files while fuzzing")))
    }
    fn number(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(Value::Int(4))
    }
    fn no_variable(it: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(it.none())
    }
    /// The clock always says 29 February 2024, 12:34:56, one hour ahead of UTC.
    fn moment(fields: &[(&str, i64)]) -> Value {
        let mut r = Record::new();
        for &(k, v) in fields {
            r.set(Rc::from(k), Value::Int(v));
        }
        Value::Record(Rc::new(r))
    }
    fn fixed_now(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(moment(&[("year", 2024), ("month", 2), ("day", 29), ("hour", 12), ("minute", 34), ("second", 56), ("offset", 60)]))
    }
    fn fixed_today(_: &mut Interp, _: &[Value], _: Span) -> Fallible<Value> {
        Ok(moment(&[("year", 2024), ("month", 2), ("day", 29)]))
    }
    let fakes: [(&str, builtins::BuiltinFn); 13] = [
        ("print", quiet),
        ("debug", same),
        ("input", empty_text),
        ("exit", quiet),
        ("time", number),
        ("random", number),
        ("env", no_variable),
        ("read_file", no_file),
        ("write_file", no_file),
        ("append_file", no_file),
        ("file_exists", number),
        ("now", fixed_now),
        ("today", fixed_today),
    ];
    SANDBOX.get_or_init(|| {
        fakes
            .iter()
            .map(|&(name, func)| {
                let real = builtins::lookup(name).unwrap_or_else(|| panic!("no built-in `{name}`"));
                &*Box::leak(Box::new(Builtin { func, ..*real }))
            })
            .collect()
    })
}

fn sandboxed_interp() -> Interp {
    let mut it = Interp::new();
    for b in sandbox() {
        it.globals.insert(Sym::new(b.name), Value::Builtin(b));
    }
    it.set_step_limit(STEPS);
    it
}

fn check_span(span: Span, file: u32, src: &str, what: &str) -> Result<(), String> {
    let (start, end) = (span.start as usize, span.end as usize);
    if span.file != file || start > end || end > src.len() || !src.is_char_boundary(start) || !src.is_char_boundary(end) {
        return Err(format!("{what} span {start}..{end} (file {}) is not inside the code ({} bytes)", span.file, src.len()));
    }
    Ok(())
}

fn check_diag(d: &Diagnostic, file: u32, src: &str) -> Result<(), String> {
    let context = || format!("[{}] {}", d.code, d.message);
    if d.message.trim().is_empty() {
        return Err(format!("empty message {}", context()));
    }
    if !d.code.is_empty() && !help::CODES.contains(&d.code) {
        return Err(format!("unknown code {}", context()));
    }
    if let Some(span) = d.span {
        check_span(span, file, src, "main").map_err(|e| format!("{e}: {}", context()))?;
    }
    for (span, _) in &d.secondary {
        check_span(*span, file, src, "secondary").map_err(|e| format!("{e}: {}", context()))?;
    }
    for (_, span) in &d.trace {
        check_span(*span, file, src, "trace").map_err(|e| format!("{e}: {}", context()))?;
    }
    let plain = d.render(false);
    let _ = d.render(true);
    if !plain.contains(d.message.lines().next().unwrap_or("")) {
        return Err(format!("the rendered message lost its text: {}", context()));
    }
    Ok(())
}

/// How far a program went.
#[derive(Clone, Copy)]
enum Stage {
    Lexer,
    Parser,
    Checker,
    RunError,
    Ran,
}

/// Lexes, parses, checks and runs `src` the way `rythme` does.
fn pipeline(src: &str) -> Result<Stage, String> {
    let file = source::add("fuzz.ry", src);
    let lexed = match lexer::lex(src, file) {
        Ok(l) => l,
        Err(d) => return check_diag(&d, file, src).map(|_| Stage::Lexer),
    };
    let items = match parser::parse(lexed) {
        Ok(items) => items,
        Err(d) => return check_diag(&d, file, src).map(|_| Stage::Parser),
    };
    let mut interp = sandboxed_interp();
    // One program in four is checked and run as in the REPL (names can be defined again).
    let repl = src.len().is_multiple_of(4);
    let diags = check::check(&items, &Knowledge::from_interp(&interp), repl);
    for d in &diags {
        check_diag(d, file, src)?;
    }
    if diags.iter().any(|d| d.is_error()) {
        return Ok(Stage::Checker);
    }
    let mut stage = Stage::Ran;
    let _ = io::take_captured();
    io::capture();
    let ran = interp.exec_items(&items, if repl { Mode::Repl } else { Mode::Run });
    let printed = io::take_captured();
    io::capture();
    match &ran {
        Ok(Some(v)) => {
            let _ = (v.repr(), v.describe());
        }
        Ok(None) => {}
        Err(d) => {
            check_diag(d, file, src)?;
            stage = Stage::RunError;
        }
    }
    if !repl {
        same_as_the_stepper(&items, &printed, ran.as_ref().err().map(|d| &**d), file, src)?;
    }
    if items.iter().any(|it| matches!(it, Item::Test(..))) {
        let mut interp = sandboxed_interp();
        match interp.exec_items(&items, Mode::Test) {
            Err(d) => check_diag(&d, file, src)?,
            Ok(_) => {
                for test in std::mem::take(&mut interp.tests) {
                    if let Err(d) = interp.eval_test(&test) {
                        check_diag(&d, file, src)?;
                    }
                }
            }
        }
    }
    Ok(stage)
}

/// `rythme steps` has an evaluator of its own: it must print the same and stop
/// with the same error as the interpreter, unless one of them reached its
/// limits (number of calls or steps, depth).
fn same_as_the_stepper(items: &[Item], printed: &str, error: Option<&Diagnostic>, file: u32, src: &str) -> Result<(), String> {
    let limited = |d: &Diagnostic| {
        d.code == "E014"
            || ["was stopped", "a été arrêté", "too deep to be shown", "trop profond pour être montré", "stopped after", "arrêtée après"]
                .iter()
                .any(|w| d.message.contains(w))
    };
    // What `help` prints is in the language of the messages, which the threads
    // of a long session change while others run.
    if error.is_some_and(limited) || src.contains("help") {
        return Ok(());
    }
    let mut interp = sandboxed_interp();
    let opts = steps::Options { show: false, limit: 100_000, ..steps::Options::default() };
    let stepped = steps::run_items(&mut interp, items, opts, 0, false);
    let stepped_printed = io::take_captured();
    io::capture();
    if let Err(d) = &stepped {
        check_diag(d, file, src)?;
        if limited(d) {
            return Ok(());
        }
    }
    // The code and the place: the language of the messages may change while the threads of a long session run.
    let message = |e: Option<&Diagnostic>| e.map(|d| format!("[{}] {:?}", d.code, d.span));
    if stepped_printed != printed || message(stepped.as_ref().err().map(|d| &**d)) != message(error) {
        return Err(format!(
            "the stepper differs from the interpreter: it printed {stepped_printed:?} and ended with {:?}, instead of {printed:?} and {:?}",
            message(stepped.as_ref().err().map(|d| &**d)),
            message(error)
        ));
    }
    Ok(())
}

/// Runs one program; `Err` describes what went wrong.
fn run_case(src: &str) -> Result<Stage, String> {
    io::capture();
    let r = std::panic::catch_unwind(AssertUnwindSafe(|| pipeline(src)));
    let _ = io::take_captured();
    match r {
        Ok(r) => r,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            Err(format!("panic: {msg}"))
        }
    }
}

/// The program of case number `i` for `seed`.
fn case(seed: u64, i: u64) -> String {
    let mut rng = Rng(seed ^ i.wrapping_mul(0x2545_F491_4F6C_DD1D));
    match rng.below(12) {
        0..=1 => token_soup(&mut rng),
        2..=4 => Gen { rng: &mut rng, fns: Vec::new(), fresh: 0 }.program(),
        5..=7 => edge_program(&mut rng),
        8 => {
            let program = Gen { rng: &mut rng, fns: Vec::new(), fresh: 0 }.program();
            tame(mutate(&mut rng, &program))
        }
        _ => {
            let seed = seeds()[rng.below(seeds().len())].clone();
            tame(mutate(&mut rng, &seed))
        }
    }
}

type Failure = (u64, String, String);

/// Runs `cases` programs from `first` on a thread with a large stack (deep
/// recursion is part of the game). Returns the failures, and how many
/// programs stopped at each stage.
fn fuzz(seed: u64, first: u64, cases: u64) -> (Vec<Failure>, [u64; 5]) {
    std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(move || {
            rythme::interp::set_stack_budget(768 << 20);
            let mut failures = Vec::new();
            let mut stages = [0; 5];
            for i in first..first + cases {
                let src = case(seed, i);
                lang::set(if i % 3 == 0 { Lang::Fr } else { Lang::En });
                match run_case(&src) {
                    Ok(stage) => stages[stage as usize] += 1,
                    Err(e) => failures.push((i, src, e)),
                }
            }
            (failures, stages)
        })
        .unwrap()
        .join()
        .unwrap()
}

fn report(seed: u64, failures: &[Failure]) {
    if failures.is_empty() {
        return;
    }
    let mut msg = format!("{} failing case(s) (seed {seed}); the shortest of each kind:\n", failures.len());
    let mut kinds: Vec<&Failure> = Vec::new();
    for f in failures {
        match kinds.iter_mut().find(|k| k.2 == f.2) {
            Some(k) if k.1.len() > f.1.len() => *k = f,
            Some(_) => {}
            None => kinds.push(f),
        }
    }
    for (i, src, e) in kinds.into_iter().take(20) {
        msg.push_str(&format!("\n--- case {i}: {e}\n{src:?}\n"));
    }
    panic!("{msg}");
}

#[test]
fn random_programs_never_crash_or_hang() {
    let seed = 0x0052_5954_4D45;
    let (failures, stages) = fuzz(seed, 0, 4000);
    report(seed, &failures);
    // The generators must keep reaching every stage.
    assert!(stages.iter().all(|&n| n > 100), "programs stopped by the lexer, parser, checker, at run time, or ran: {stages:?}");
}

/// A long session; the seed and the number of cases come from the environment.
#[test]
#[ignore]
fn long_fuzzing_session() {
    let seed = std::env::var("RYTHME_FUZZ_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let cases: u64 = std::env::var("RYTHME_FUZZ_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(200_000);
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get()) as u64;
    // Fresh threads every 20 000 cases: each keeps the files and names it saw.
    let batch = 20_000.min(cases);
    let mut failures = Vec::new();
    let mut stages = [0; 5];
    let mut next = 0;
    while next < cases {
        let handles: Vec<_> = (0..threads)
            .map(|t| next + t * batch)
            .filter(|&start| start < cases)
            .map(|start| std::thread::spawn(move || fuzz(seed, start, batch.min(cases - start))))
            .collect();
        next += threads * batch;
        for h in handles {
            let (f, s) = h.join().unwrap();
            failures.extend(f);
            stages.iter_mut().zip(s).for_each(|(a, b)| *a += b);
        }
        eprintln!("{} cases, {} failures; lexer/parser/checker/run error/ran: {stages:?}", next.min(cases), failures.len());
    }
    report(seed, &failures);
}

// ----- inputs that made Rythme fail, found by fuzzing -------------------------

#[track_caller]
fn survives(src: &str) {
    let failures = std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn({
            let src = src.to_string();
            move || {
                rythme::interp::set_stack_budget(768 << 20);
                [Lang::En, Lang::Fr]
                    .into_iter()
                    .filter_map(|l| {
                        lang::set(l);
                        run_case(&src).err()
                    })
                    .collect::<Vec<_>>()
            }
        })
        .unwrap()
        .join()
        .unwrap();
    assert!(failures.is_empty(), "{src:?}: {failures:?}");
}

#[test]
fn inputs_found_by_fuzzing() {
    for src in [
        // Long chains of operators and calls (the tree was too deep for the stack).
        &format!("x = {}", ["1"; 5000].join(" + ")),
        &format!("x = 1{}", "?".repeat(5000)),
        &format!("x = abs{}", "(1)".repeat(5000)),
        &format!("x = 1{}", ".abs()".repeat(5000)),
        &format!("x = {}", ["2"; 5000].join(" ** ")),
        &format!("x = {}1{}", "abs(".repeat(5000), ")".repeat(5000)),
        &format!("x = {}", "fn f()\n".repeat(3000)),
        // `fail` with an empty message gave an error without any text.
        "fail(\"\")",
        "fail(\"\\n\")",
    ] {
        survives(src);
    }
}

// ----- properties ------------------------------------------------------------

/// Whole numbers worth trying: the limits and their neighbours, powers of
/// two (where decimal numbers stop being exact), and random ones.
fn some_int(rng: &mut Rng) -> i64 {
    match rng.below(6) {
        0 => [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX, 2, -2, 3, -3][rng.below(11)],
        1 => {
            let p = 1i64 << rng.below(63);
            [p, p - 1, p + 1, -p, -p + 1][rng.below(5)]
        }
        2 => (rng.next() % 2000) as i64 - 1000,
        _ => rng.next() as i64,
    }
}

fn some_float(rng: &mut Rng) -> f64 {
    match rng.below(5) {
        0 => [0.0, -0.0, 0.5, -0.5, 1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 9.223372036854776e18, -9.223372036854776e18]
            [rng.below(10)],
        1 => some_int(rng) as f64,
        2 => some_int(rng) as f64 + 0.5,
        3 => f64::from_bits(rng.next()),
        _ => (rng.next() % 20000) as f64 / 8.0 - 1000.0,
    }
}

#[test]
fn whole_number_arithmetic_is_exact_or_an_error() {
    use rythme::ast::BinOp;
    let mut rng = Rng(11);
    let span = Span::default();
    let show = |r: Fallible<Value>| r.map(|v| v.repr()).map_err(|d| d.message);
    for _ in 0..200_000 {
        let (a, b) = (some_int(&mut rng), some_int(&mut rng));
        let (wa, wb) = (a as i128, b as i128);
        // Division rounded down, and the remainder that goes with it (same sign as b).
        let quotient = (wb != 0).then(|| if wa % wb != 0 && ((wa < 0) != (wb < 0)) { wa / wb - 1 } else { wa / wb });
        for (op, expected) in [
            (BinOp::Add, Some(wa + wb)),
            (BinOp::Sub, Some(wa - wb)),
            (BinOp::Mul, Some(wa * wb)),
            (BinOp::IntDiv, quotient),
            (BinOp::Mod, quotient.map(|q| wa - wb * q)),
        ] {
            let got = rythme::interp::binop(op, &Value::Int(a), &Value::Int(b), span);
            let sym = op.symbol();
            match (expected, got) {
                (None, Err(d)) => assert_eq!(d.code, "E008", "{a} {sym} {b}"),
                (Some(e), Ok(Value::Int(r))) => assert_eq!(e, r as i128, "{a} {sym} {b}"),
                (Some(e), Err(d)) => assert!(i64::try_from(e).is_err() && d.code == "E012", "{a} {sym} {b}: {}", d.message),
                (e, got) => panic!("{a} {sym} {b}: expected {e:?}, got {:?}", show(got)),
            }
        }
        // Powers: exact, or an error when too large.
        let n = (rng.next() % 70) as u32;
        match (a.checked_pow(n), rythme::interp::binop(BinOp::Pow, &Value::Int(a), &Value::Int(n as i64), span)) {
            (Some(e), Ok(Value::Int(r))) => assert_eq!(e, r, "{a} ** {n}"),
            (None, Err(d)) => assert_eq!(d.code, "E012", "{a} ** {n}"),
            (e, got) => panic!("{a} ** {n}: expected {e:?}, got {:?}", show(got)),
        }
        let lt = rythme::interp::binop(BinOp::Lt, &Value::Int(a), &Value::Int(b), span);
        assert_eq!(show(lt), Ok((a < b).to_string()), "{a} < {b}");
    }
}

#[test]
fn whole_and_decimal_numbers_compare_exactly() {
    use std::cmp::Ordering;
    let mut rng = Rng(12);
    for _ in 0..200_000 {
        let (i, f) = (some_int(&mut rng), some_float(&mut rng));
        // The exact answer, with the whole part of the decimal number in an i128.
        let expected = if f.is_nan() {
            None
        } else if f >= 2f64.powi(64) {
            Some(Ordering::Less)
        } else if f <= -(2f64.powi(64)) {
            Some(Ordering::Greater)
        } else {
            let whole = f.floor() as i128;
            Some((i as i128).cmp(&whole).then(if f.fract() == 0.0 { Ordering::Equal } else { Ordering::Less }))
        };
        let (vi, vf) = (Value::Int(i), Value::Float(f));
        assert_eq!(vi.compare(&vf), expected, "{i} vs {f:?}");
        assert_eq!(vf.compare(&vi), expected.map(Ordering::reverse), "{f:?} vs {i}");
        assert_eq!(vi.equals(&vf), expected == Some(Ordering::Equal), "{i} == {f:?}");
    }
}

#[test]
fn sorting_uses_a_consistent_order() {
    use std::cmp::Ordering;
    let mut rng = Rng(13);
    let it = Interp::new();
    let mut pool: Vec<Value> = Vec::new();
    for _ in 0..60 {
        pool.push(Value::Int(some_int(&mut rng)));
        pool.push(Value::Float(some_float(&mut rng)));
    }
    pool.extend([Value::text(""), Value::text("a"), Value::text("é"), Value::Unit, Value::Bool(true), it.none(), it.some(Value::Int(1))]);
    pool.push(Value::list(vec![Value::Int(1), Value::Float(f64::NAN)]));
    pool.push(Value::list(vec![Value::Float(1.0)]));
    for _ in 0..300_000 {
        let a = &pool[rng.below(pool.len())];
        let b = &pool[rng.below(pool.len())];
        let c = &pool[rng.below(pool.len())];
        let (ab, bc, ac) = (a.total_cmp(b), b.total_cmp(c), a.total_cmp(c));
        assert_eq!(ab, b.total_cmp(a).reverse(), "{} vs {}", a.repr(), b.repr());
        if ab != Ordering::Greater && bc != Ordering::Greater {
            assert_ne!(ac, Ordering::Greater, "{} <= {} <= {}", a.repr(), b.repr(), c.repr());
        }
        if let Some(o) = a.compare(b) {
            if o != Ordering::Equal {
                assert_eq!(o, ab, "{} vs {}", a.repr(), b.repr());
            }
        }
    }
    // Rust's sort stops with a panic when the order is not consistent.
    let sort = builtins::lookup("sort").unwrap();
    let mut it = Interp::new();
    for _ in 0..2000 {
        let n = rng.below(40);
        let list = Value::list((0..n).map(|_| pool[rng.below(pool.len())].clone()).collect());
        let _ = (sort.func)(&mut it, &[list], Span::default());
    }
}

/// A random value whose text (`repr`) is Rythme code.
fn some_value(rng: &mut Rng, it: &Interp, depth: usize) -> Value {
    const CHARS: &[char] = &['a', 'Z', ' ', '"', '\\', '{', '}', '\n', '\t', '\r', '\0', '\u{1}', 'é', '\u{301}', '👍', '#', '\u{feff}'];
    let text = |rng: &mut Rng| -> String { (0..rng.below(6)).map(|_| CHARS[rng.below(CHARS.len())]).collect() };
    match rng.below(if depth == 0 { 6 } else { 10 }) {
        0 => Value::Int(some_int(rng)),
        1 => Value::Float(Some(some_float(rng)).filter(|f| f.is_finite()).unwrap_or(0.25)),
        2 => Value::Text(text(rng).into()),
        3 => Value::Bool(rng.chance(50)),
        4 => Value::Unit,
        5 => it.none(),
        6 => Value::list((0..rng.below(4)).map(|_| some_value(rng, it, depth - 1)).collect()),
        7 => {
            let mut r = rythme::value::Record::new();
            for _ in 0..rng.below(4) {
                let key = match rng.below(3) {
                    0 => rng.pick(&["name", "type", "for", "_", "Ab", "1a", "é", "true", "x y", ""]).to_string(),
                    _ => text(rng),
                };
                r.set(key.into(), some_value(rng, it, depth - 1));
            }
            Value::Record(std::rc::Rc::new(r))
        }
        8 => it.some(some_value(rng, it, depth - 1)),
        _ => {
            let inner = some_value(rng, it, depth - 1);
            if rng.chance(50) {
                it.ok(inner)
            } else {
                it.err(inner)
            }
        }
    }
}

#[test]
fn values_read_back_from_their_text_are_the_same() {
    let mut rng = Rng(14);
    for _ in 0..20_000 {
        let v = some_value(&mut rng, &Interp::new(), 3);
        let code = format!("value = {}\n", v.repr());
        let file = source::add("repr.ry", &code);
        let items = lexer::lex(&code, file).and_then(parser::parse).unwrap_or_else(|d| panic!("{code:?}: {}", d.render(false)));
        let mut it = Interp::new();
        it.exec_items(&items, Mode::Run).unwrap_or_else(|d| panic!("{code:?}: {}", d.render(false)));
        let back = it.globals.get(&Sym::new("value")).unwrap();
        assert_eq!(back.repr(), v.repr(), "{code:?}");
        assert!(back.equals(&v), "{code:?}");
    }
}
