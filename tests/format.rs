//! `rythme fmt`: the formatter never changes what a program means, never loses
//! a comment, gives the same result when run twice, and does not depend on how
//! the code was spaced. Checked on every piece of Rythme code of the
//! repository, on mutations of it, and on generated programs.

mod common;

use std::path::{Path, PathBuf};

use common::{rythme, rythme_lang};
use rythme::format::format;
use rythme::lexer::{self, Tok};
use rythme::parser;

// ----- what a program means, and its comments ---------------------------------

/// The syntax tree of a program without positions, or `None` if it does not parse.
fn tree(src: &str) -> Option<String> {
    let items = lexer::lex(src, 7).and_then(parser::parse).ok()?;
    let debug = format!("{items:?}");
    let mut out = String::new();
    let mut rest = debug.as_str();
    while let Some(i) = rest.find("Span {") {
        out.push_str(&rest[..i]);
        let end = rest[i..].find('}').unwrap() + i + 1;
        rest = &rest[end..];
    }
    out.push_str(rest);
    Some(out)
}

/// The comments of a program, read independently of the formatter: texts can
/// contain `#`, and `{...}` inside texts can contain other texts.
fn comments(src: &str) -> Vec<String> {
    enum Mode {
        Code(usize),
        Text,
    }
    let mut stack = vec![Mode::Code(0)];
    let mut found = Vec::new();
    let mut chars = src.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let nested = stack.len() > 1;
        match stack.last_mut().unwrap() {
            Mode::Code(depth) => match c {
                '#' => {
                    let end = src[i..].find('\n').map_or(src.len(), |e| i + e);
                    found.push(src[i..end].trim_end().to_string());
                    while chars.peek().is_some_and(|&(j, _)| j < end) {
                        chars.next();
                    }
                }
                '"' => stack.push(Mode::Text),
                '{' => *depth += 1,
                '}' if *depth == 0 && nested => {
                    stack.pop();
                }
                '}' => *depth -= 1,
                _ => {}
            },
            Mode::Text => match c {
                '\\' => {
                    chars.next();
                }
                '"' => {
                    stack.pop();
                }
                '{' => stack.push(Mode::Code(0)),
                _ => {}
            },
        }
    }
    found
}

/// Formats a program that parses and checks every promise of the formatter.
#[track_caller]
fn check(src: &str, origin: &str) -> String {
    let before = tree(src).expect("the program parses");
    let out = match format(src) {
        Ok(out) => out,
        Err(d) => panic!("{origin}: could not format:\n{src}\n---\n{}", d[0].render(false)),
    };
    assert_eq!(tree(&out).as_ref(), Some(&before), "{origin}: the meaning changed:\n{src}\n--- became ---\n{out}");
    assert_eq!(comments(&out), comments(src), "{origin}: comments changed:\n{src}\n--- became ---\n{out}");
    let again = format(&out).unwrap_or_else(|d| panic!("{origin}: the result does not format:\n{out}\n{}", d[0].render(false)));
    assert_eq!(again, out, "{origin}: formatting twice gives something else:\n{src}\n--- first ---\n{out}");
    // Outside texts written on several lines (kept as they are): no tabs, no
    // spaces at the end of lines, two spaces per level.
    let texts: Vec<(usize, usize)> = lexer::lex(&out, 7)
        .unwrap()
        .tokens
        .iter()
        .filter(|t| matches!(t.tok, Tok::Str(_)))
        .map(|t| (t.span.start as usize, t.span.end as usize))
        .collect();
    let in_text = |at: usize| texts.iter().any(|&(start, end)| start < at && at < end);
    let mut at = 0;
    for line in out.split_inclusive('\n') {
        let code = line.trim_end_matches('\n').trim_end_matches('\r');
        let indent = code.len() - code.trim_start().len();
        if !in_text(at) {
            assert!(indent % 2 == 0 && !code[..indent].contains('\t'), "{origin}: odd indentation:\n{out}");
        }
        if !in_text(at + code.len()) {
            assert!(!code.ends_with([' ', '\t']), "{origin}: spaces at the end of a line:\n{out}");
        }
        at += line.len();
    }
    if !out.is_empty() {
        assert!(out.ends_with('\n') && !out.ends_with("\n\n"), "{origin}: wrong end of file:\n{out:?}");
    }
    assert!(!out.contains("\n\n\n"), "{origin}: two blank lines in a row:\n{out}");
    out
}

// ----- every piece of Rythme code of the repository --------------------------------

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn files(dir: &Path, ext: &str, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let path = e.path();
        if path.is_dir() && !e.file_name().to_string_lossy().starts_with('.') && e.file_name() != "target" {
            files(&path, ext, found);
        } else if path.extension().is_some_and(|x| x == ext) {
            found.push(path);
        }
    }
}

/// The string literals of a Rust file, unescaped.
fn rust_strings(src: &str) -> Vec<String> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => i = src[i..].find('\n').map_or(b.len(), |e| i + e),
            b'\'' => {
                // A character like '"' or '\\', or a lifetime.
                if b.get(i + 2) == Some(&b'\'') {
                    i += 3;
                } else if b.get(i + 1) == Some(&b'\\') {
                    i = src[i + 2..].find('\'').map_or(b.len(), |e| i + 2 + e + 1);
                } else {
                    i += 1;
                }
            }
            b'r' if (b.get(i + 1) == Some(&b'"') || b.get(i + 1) == Some(&b'#')) && (i == 0 || !b[i - 1].is_ascii_alphanumeric()) => {
                let hashes = b[i + 1..].iter().take_while(|&&c| c == b'#').count();
                let start = i + 1 + hashes + 1;
                let close = format!("\"{}", "#".repeat(hashes));
                let Some(end) = src[start..].find(&close) else { break };
                out.push(src[start..start + end].to_string());
                i = start + end + close.len();
            }
            b'"' => {
                let mut s = String::new();
                let mut j = i + 1;
                let chars: Vec<char> = src[j..].chars().take(200_000).collect();
                let mut k = 0;
                while k < chars.len() && chars[k] != '"' {
                    if chars[k] == '\\' {
                        k += 1;
                        match chars.get(k) {
                            Some('n') => s.push('\n'),
                            Some('t') => s.push('\t'),
                            Some('r') => s.push('\r'),
                            Some('0') => s.push('\0'),
                            Some('u') => {
                                let close = chars[k..].iter().position(|&c| c == '}').unwrap_or(0);
                                let hex: String = chars[k + 2..k + close].iter().collect();
                                s.extend(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32));
                                k += close;
                            }
                            Some('\n') => {
                                while chars.get(k + 1).is_some_and(|c| c.is_whitespace()) {
                                    k += 1;
                                }
                            }
                            Some(&c) => s.push(c),
                            None => {}
                        }
                    } else {
                        s.push(chars[k]);
                    }
                    k += 1;
                }
                j += chars[..k].iter().map(|c| c.len_utf8()).sum::<usize>();
                out.push(s);
                i = j + 1;
            }
            _ => i += 1,
        }
    }
    out
}

/// The ```rythme blocks of a Markdown file.
fn markdown_code(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(i) = rest.find("```rythme\n") {
        rest = &rest[i + 10..];
        let end = rest.find("```").unwrap_or(rest.len());
        out.push(rest[..end].to_string());
        rest = &rest[end..];
    }
    out
}

/// Every piece of Rythme code of the repository that parses, with where it comes from.
fn corpus() -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut ry = Vec::new();
    files(&root().join("examples"), "ry", &mut ry);
    files(&root().join("bench"), "ry", &mut ry);
    files(&root().join("docs/exercises"), "ry", &mut ry);
    files(&root().join("docs/exercises/.solutions"), "ry", &mut ry);
    for p in ry {
        out.push((p.display().to_string(), std::fs::read_to_string(&p).unwrap()));
    }
    let mut rs = Vec::new();
    files(&root().join("tests"), "rs", &mut rs);
    files(&root().join("site/src"), "rs", &mut rs);
    files(&root().join("src"), "rs", &mut rs);
    for p in rs {
        for (k, s) in rust_strings(&std::fs::read_to_string(&p).unwrap()).into_iter().enumerate() {
            out.push((format!("{} (string {k})", p.display()), s));
        }
    }
    let mut md = Vec::new();
    files(&root(), "md", &mut md);
    for p in md {
        for (k, s) in markdown_code(&std::fs::read_to_string(&p).unwrap()).into_iter().enumerate() {
            out.push((format!("{} (block {k})", p.display()), s));
        }
    }
    out.retain(|(_, s)| s.len() < 100_000 && tree(s).is_some());
    out
}

#[test]
fn every_program_of_the_repository_keeps_its_meaning() {
    let corpus = corpus();
    assert!(corpus.len() > 300, "only {} programs found", corpus.len());
    for (origin, src) in &corpus {
        check(src, origin);
    }
}

#[test]
fn the_examples_are_formatted() {
    let mut ry = Vec::new();
    files(&root().join("examples"), "ry", &mut ry);
    assert!(ry.len() >= 10);
    for p in ry {
        let src = std::fs::read_to_string(&p).unwrap();
        assert_eq!(format(&src).unwrap(), src, "`rythme fmt {}` changes it", p.display());
    }
}

// ----- mutations --------------------------------------------------------------

/// A small deterministic random number generator.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

/// The text between the tokens of a program: `(start, end)` of each gap,
/// and whether spaces may be added there when there are none.
fn gaps(src: &str) -> Vec<(usize, usize, bool)> {
    let tokens = lexer::lex(src, 7).unwrap().tokens;
    let mut out = Vec::new();
    let mut end = if src.starts_with('\u{FEFF}') { 3 } else { 0 };
    let mut before: [Option<&Tok>; 2] = [None, None];
    for t in tokens.iter().filter(|t| t.tok != Tok::Newline) {
        // `even?` and `even?(4)` are names, `unless(x)` a call: these
        // tokens are written next to each other on purpose.
        let joined = match (before, &t.tok) {
            ([_, Some(Tok::Ident(_))], Tok::Question) => true,
            ([Some(Tok::Ident(_)), Some(Tok::Question)], Tok::LParen) => true,
            ([_, Some(Tok::Ident(s))], Tok::LParen | Tok::LBracket) if &*s.name() == "unless" => true,
            // A space before `[` or `-` after a name, and none after that `-`,
            // may make a call without parentheses: `puts [1]`, `puts -1`.
            ([_, Some(Tok::Ident(_) | Tok::Question)], Tok::LBracket | Tok::Minus) => true,
            ([Some(Tok::Ident(_) | Tok::Question), Some(Tok::Minus)], _) => true,
            _ => false,
        };
        out.push((end, t.span.start as usize, !joined));
        end = t.span.end as usize;
        before = [before[1], Some(&t.tok)];
    }
    out
}

/// Changes the spaces of a program, but not its tokens, line breaks or blank
/// lines: indentation, spaces between tokens, spaces before comments.
fn respace(src: &str, rng: &mut Rng) -> String {
    let mut out = String::new();
    let mut last = 0;
    for (start, end, may_space) in gaps(src) {
        out.push_str(&src[last..start]);
        let gap = &src[start..end];
        let mut rest = gap;
        let first = start == 0;
        while !rest.is_empty() {
            if rest.starts_with('#') {
                let e = rest.find('\n').unwrap_or(rest.len());
                out.push_str(&rest[..e]);
                rest = &rest[e..];
            } else if rest.starts_with([' ', '\t']) {
                let e = rest.find(|c| c != ' ' && c != '\t').unwrap_or(rest.len());
                rest = &rest[e..];
                out.push_str(&random_spaces(rng, 1));
            } else {
                let c = rest.chars().next().unwrap();
                out.push(c);
                rest = &rest[c.len_utf8()..];
                // Indentation, or more blank lines where there was one.
                if c == '\n' && !rest.starts_with([' ', '\t']) && rng.chance(30) {
                    out.push_str(&random_spaces(rng, 0));
                }
                if c == '\n' && rest.starts_with('\n') && rng.chance(50) {
                    out.push('\n');
                }
            }
        }
        if gap.is_empty() && may_space && !first && end > 0 && rng.chance(30) {
            out.push_str(&random_spaces(rng, 1));
        }
        last = end;
    }
    out.push_str(&src[last..]);
    out
}

fn random_spaces(rng: &mut Rng, min: usize) -> String {
    (0..min + rng.below(4)).map(|_| if rng.chance(15) { '\t' } else { ' ' }).collect()
}

#[test]
fn spaces_do_not_change_the_result() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for (origin, src) in corpus() {
        let expected = check(&src, &origin);
        for _ in 0..3 {
            let mutated = respace(&src, &mut rng);
            assert_eq!(tree(&mutated), tree(&src), "{origin}: the mutation changed the program:\n{mutated}");
            assert_eq!(check(&mutated, &origin), expected, "{origin}: spaces changed the result, from:\n{mutated}");
        }
    }
}

/// Adds line breaks, `;` and comments where the language allows them: a
/// mutation is kept only when the program still means the same thing.
fn rebreak(src: &str, rng: &mut Rng) -> Option<String> {
    let gaps = gaps(src);
    let (start, end, _) = gaps[1 + rng.below(gaps.len().checked_sub(1).filter(|&n| n > 0)?)];
    let insert = match rng.below(6) {
        0 => "\n".to_string(),
        1 => "\n\n".to_string(),
        2 => format!("  # note {}\n", rng.below(100)),
        3 => format!("\n# note {}\n", rng.below(100)),
        4 => ";".to_string(),
        _ => format!(" # note {}\n   ", rng.below(100)),
    };
    let at = if rng.chance(50) { start } else { end };
    let mutated = format!("{}{insert}{}", &src[..at], &src[at..]);
    (tree(&mutated) == tree(src)).then_some(mutated)
}

#[test]
fn line_breaks_and_comments_anywhere_are_handled() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let mut tried = 0;
    for (origin, src) in corpus() {
        let mut current = src.clone();
        for _ in 0..12 {
            if let Some(m) = rebreak(&current, &mut rng) {
                check(&m, &origin);
                current = m;
                tried += 1;
            }
        }
    }
    assert!(tried > 1000, "only {tried} mutations");
}

// ----- generated programs ---------------------------------------------------------

/// Generates random programs with random layouts.
struct Gen {
    rng: Rng,
    depth: usize,
}

impl Gen {
    fn pick<'a>(&mut self, xs: &[&'a str]) -> &'a str {
        xs[self.rng.below(xs.len())]
    }

    /// A line break or a space, where both are allowed.
    fn sp(&mut self) -> &'static str {
        match self.rng.below(20) {
            0 | 1 => "\n  ",
            2 => "  # why\n      ",
            3 => "\n\n",
            4 => "\n  # step\n  ",
            _ => " ",
        }
    }

    fn atom(&mut self) -> String {
        match self.rng.below(9) {
            0 => self.rng.below(100).to_string(),
            1 => format!("{}.5", self.rng.below(10)),
            2 => "\"text {x}\"".to_string(),
            3 => self.pick(&["x", "y", "xs", "total", "é"]).to_string(),
            4 => self.pick(&["None", "Some(1)", "Ok(x)", "true"]).to_string(),
            5 => format!("[{}]", self.list(&["1", "x", "..xs"])),
            6 => format!("{{{}}}", self.list(&["a: 1", "b: x", "..r", "c"])),
            7 => format!("-{}", self.pick(&["x", "1", "(x + 1)"])),
            _ => format!("f({})", self.list(&["1", "x", "y"])),
        }
    }

    fn list(&mut self, items: &[&str]) -> String {
        let n = self.rng.below(4);
        let mut out = String::new();
        for i in 0..n {
            if i > 0 {
                out.push_str(if self.rng.chance(20) { ",\n" } else { ", " });
            }
            out.push_str(self.pick(items));
        }
        if n > 0 && self.rng.chance(15) {
            out.push(',');
        }
        out
    }

    fn expr(&mut self) -> String {
        if self.depth > 3 {
            return self.atom();
        }
        self.depth += 1;
        let e = match self.rng.below(14) {
            0 | 1 => self.atom(),
            2 => {
                let op = self.pick(&["+", "-", "*", "==", "<", "and", "or", "..", "**", "//"]);
                let (a, b, sp) = (self.atom(), self.atom(), self.sp());
                // `-` cannot start a line.
                if op == "-" {
                    format!("{a} -{sp}{b}")
                } else {
                    format!("{a}{sp}{op} {b}")
                }
            }
            3 => {
                let mut e = self.atom();
                for _ in 0..1 + self.rng.below(3) {
                    let step = self.pick(&["map(f)", "filter(fn(v) = v > 1)", "sum", "len", "sort_by(fn(v) = -v)"]);
                    let sp = self.sp();
                    e = format!("{e}{sp}|> {step}");
                }
                e
            }
            4 => {
                let (c, a, b) = (self.expr(), self.expr(), self.expr());
                let (s1, s2) = (self.sp(), self.sp());
                format!("if {c} then{s1}{a}{s2}else {b}")
            }
            5 => {
                let (s, a, b) = (self.atom(), self.expr(), self.expr());
                format!("match {s}\n  0 -> {a}\n  [p, ..rest] if p > 0 -> p\n  Some(v) | Ok(v) ->\n    {b}\n  _ -> 0\nend")
            }
            6 => {
                let body = self.expr();
                let sp = self.sp();
                format!("fn(v) ={sp}{body}")
            }
            7 => {
                let (a, b) = (self.expr(), self.expr());
                format!("do\n  v = {a}\n  {b}\nend")
            }
            8 => format!("xs.map(f).len(){}", self.pick(&["", "?", "[0]"])),
            9 => {
                let b = self.expr();
                format!("each(xs, fn(v)\n  print(v)\n  {b}\nend)")
            }
            // Ruby style: blocks, `unless`, symbols, names ending with `?`, chains.
            11 => {
                let (a, b) = (self.expr(), self.expr());
                match self.rng.below(4) {
                    0 => format!("xs.map do |v| {a} end"),
                    1 => format!("xs.each do |v, w|\n  print(v)\n  {b}\nend"),
                    2 => format!("xs.select {{ |v| {a} }}.sum"),
                    _ => format!("map(xs) do\n  {a}\nend"),
                }
            }
            12 => {
                let (c, a, b) = (self.expr(), self.expr(), self.expr());
                let (s1, s2) = (self.sp(), self.sp());
                format!("unless {c} then{s1}{a}{s2}else {b}")
            }
            13 => {
                let sp = self.sp();
                let chain = format!("xs\n  .select(even?){sp}.map {{ |v| v * 2 }}\n  .sum");
                self.pick(&[":ok", "xs.sum", "x.even?", "even?(2)", "\"#{x} {y}\"", "x?", "[:a, :b]"]).to_string()
                    + if self.rng.chance(30) { "" } else { " == x" }
                    + &if self.rng.chance(20) { format!(" + {chain}") } else { String::new() }
            }
            _ => {
                let (a, b) = (self.expr(), self.expr());
                format!("({a}) + (\n  {b}\n)")
            }
        };
        self.depth -= 1;
        e
    }

    fn comment(&mut self) -> &'static str {
        if self.rng.chance(20) {
            "  # a comment"
        } else {
            ""
        }
    }

    fn program(&mut self) -> String {
        let mut out = String::new();
        for _ in 0..1 + self.rng.below(5) {
            let item = match self.rng.below(12) {
                0 => {
                    let (e, c) = (self.expr(), self.comment());
                    format!("fn g(x, y) = {e}{c}")
                }
                1 => {
                    let (a, b) = (self.expr(), self.expr());
                    format!("fn h(x)\n  y = {a}\n  {b}\nend")
                }
                2 => {
                    let sep = self.pick(&[" ", "\n  "]);
                    format!("type T ={sep}A | B(x){sep}| C(x, y)")
                }
                3 => {
                    let e = self.expr();
                    format!("test \"t\" = {e}")
                }
                4 => {
                    let e = self.expr();
                    format!("## Docs.\nfn k(0) = 0\nfn k(n) =\n  {e}")
                }
                5 => {
                    let (a, b, sp) = (self.expr(), self.expr(), self.sp());
                    format!("test \"block\"\n  fn local([first, ..], {{name, type: t}}) if first > 0 ={sp}{a}\n  assert({b});  x = 1\nend")
                }
                6 => {
                    let (a, sp) = (self.expr(), self.sp());
                    format!("fn m(v) = match v\n  # first case\n  Some({{\"key\": k}}) | Ok([k]) ->{sp}{a}\n\n  -1 | 2.5 -> v.type\n  _ -> do v end\nend")
                }
                7 => {
                    let (a, b) = (self.expr(), self.expr());
                    format!("use \"other.ry\"\nz = [\n  {a},\n  # comment\n  {b}\n]")
                }
                // Calls without parentheses, and names followed by `-` or `[` that are not calls.
                8 => {
                    let (a, b, sp) = (self.expr(), self.expr(), self.sp());
                    match self.rng.below(6) {
                        0 => format!("print x,{sp}{b}\nputs {a}"),
                        1 => format!("puts -1\nputs [x, {a}]\nx -1\nxs [0]"),
                        2 => format!("xs.each do |v| puts v end\neach xs do |v|\n  print v, {a}\nend"),
                        3 => format!("assert_eq total, {b}{}", self.comment()),
                        4 => "fn f(v) = v\nf -x\nf [1]\nf(-x)\nf\n  -1".to_string(),
                        _ => format!("test \"t\"\n  assert not {a}\n  even? 4\nend"),
                    }
                }
                _ => {
                    let (e, c) = (self.expr(), self.comment());
                    format!("x = {e}{c}")
                }
            };
            out.push_str(&item);
            out.push_str(self.pick(&["\n", "\n\n", "\n# between\n"]));
        }
        out
    }
}

#[test]
fn generated_programs_keep_their_meaning() {
    let mut g = Gen { rng: Rng(0x1234_5678_9abc_def1), depth: 0 };
    let mut rng = Rng(0x0bad_cafe_dead_beef);
    let mut valid = 0;
    for i in 0..1200 {
        let src = g.program();
        if tree(&src).is_some() {
            let origin = format!("generated program {i}");
            let expected = check(&src, &origin);
            assert_eq!(check(&respace(&src, &mut rng), &origin), expected, "{origin}: spaces changed the result");
            valid += 1;
        }
    }
    assert!(valid > 1000, "only {valid} generated programs parse");
}

// ----- the command ----------------------------------------------------------------

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rythme-fmt-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn fmt_rewrites_files_and_reports_them() {
    let dir = temp_dir("write");
    let messy = dir.join("messy.ry");
    let clean = dir.join("clean.ry");
    std::fs::write(&messy, "x=1\nprint( x )").unwrap();
    std::fs::write(&clean, "x = 1\n").unwrap();
    let r = rythme(&["fmt", messy.to_str().unwrap(), clean.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert!(r.stdout.contains("formatted: ") && r.stdout.contains("messy.ry"), "{}", r.stdout);
    assert!(!r.stdout.contains("formatted: ") || !r.stdout.contains("clean.ry\n"), "{}", r.stdout);
    assert!(r.stdout.contains("1 file(s) formatted, 1 already formatted"), "{}", r.stdout);
    assert_eq!(std::fs::read_to_string(&messy).unwrap(), "x = 1\nprint(x)\n");
    assert_eq!(std::fs::read_to_string(&clean).unwrap(), "x = 1\n");
    // In French, and with the French name of the command.
    std::fs::write(&messy, "y=2\n").unwrap();
    let r = rythme_lang("fr", &["formater", messy.to_str().unwrap()]);
    assert_eq!(r.code, 0);
    assert!(r.stdout.contains("mis en forme : ") && r.stdout.contains("1 fichier(s) mis en forme"), "{}", r.stdout);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fmt_check_lists_what_needs_formatting() {
    let dir = temp_dir("check");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    std::fs::create_dir_all(dir.join(".hidden")).unwrap();
    std::fs::write(dir.join("good.ry"), "x = 1\n").unwrap();
    std::fs::write(dir.join("sub/bad.ry"), "x  = 1\n").unwrap();
    std::fs::write(dir.join(".hidden/bad.ry"), "x  = 1\n").unwrap();
    std::fs::write(dir.join("notes.txt"), "x  = 1\n").unwrap();
    let r = rythme(&["fmt", "--check", dir.to_str().unwrap()]);
    assert_eq!(r.code, 1, "{}{}", r.stdout, r.stderr);
    assert!(r.stdout.contains("needs formatting: ") && r.stdout.contains("bad.ry"), "{}", r.stdout);
    assert_eq!(r.stdout.matches("needs formatting").count(), 1, "hidden folders and other files are skipped:\n{}", r.stdout);
    assert_eq!(std::fs::read_to_string(dir.join("sub/bad.ry")).unwrap(), "x  = 1\n", "--check does not write");
    let r = rythme(&["fmt", dir.to_str().unwrap()]);
    assert_eq!(r.code, 0);
    let r = rythme(&["fmt", "--check", dir.to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stdout);
    assert!(r.stdout.contains("✓ 2 file(s) already formatted"), "{}", r.stdout);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fmt_refuses_code_with_syntax_errors() {
    let dir = temp_dir("errors");
    let bad = dir.join("bad.ry");
    std::fs::write(&bad, "fn f(x) =\nprint(\"hello)\n").unwrap();
    let r = rythme(&["fmt", bad.to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E002]") && r.stderr.contains("bad.ry:2:7"), "{}", r.stderr);
    assert!(r.stdout.contains("1 file(s) left as they are because of errors"), "{}", r.stdout);
    assert_eq!(std::fs::read_to_string(&bad).unwrap(), "fn f(x) =\nprint(\"hello)\n");
    let r = rythme(&["fmt", dir.join("bda.ry").to_str().unwrap()]);
    assert_eq!(r.code, 1);
    assert!(r.stderr.contains("error[E017]") && r.stderr.contains("bad.ry"), "a suggestion:\n{}", r.stderr);
    // Without `.ry`, like `rythme run hello`.
    std::fs::write(dir.join("good.ry"), "x  = 1\n").unwrap();
    let r = rythme(&["fmt", dir.join("good").to_str().unwrap()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(std::fs::read_to_string(dir.join("good.ry")).unwrap(), "x = 1\n");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fmt_reads_the_standard_input() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let run = |args: &[&str], input: &str| {
        let mut child = Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(args)
            .env("RYTHME_LANG", "en")
            .env("NO_COLOR", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
        let out = child.wait_with_output().unwrap();
        (
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stdout).into_owned(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
    };
    let (code, out, _) = run(&["fmt", "-"], "xs |> map(f)\n |> sum");
    assert_eq!((code, out.as_str()), (0, "xs\n  |> map(f)\n  |> sum\n"));
    let (code, out, _) = run(&["fmt", "--check", "-"], "x  = 1\n");
    assert_eq!(code, 1);
    assert!(out.contains("needs formatting: <stdin>"), "{out}");
    let (code, _, _) = run(&["fmt", "--check", "-"], "x = 1\n");
    assert_eq!(code, 0);
    let (code, _, err) = run(&["fmt", "-"], "x = [1, 2\n");
    assert_eq!(code, 1);
    assert!(err.contains("error[E0"), "{err}");
}

#[test]
fn fmt_usage_and_help() {
    let r = rythme(&["fmt"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("rythme fmt hello.ry"), "{}", r.stderr);
    let r = rythme(&["fmt", "--chek", "x.ry"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`--check`"), "{}", r.stderr);
    let r = rythme(&["fnt"]);
    assert!(r.stderr.contains("rythme fmt"), "{}", r.stderr);
    for lang in ["en", "fr"] {
        let r = rythme_lang(lang, &["help", "fmt"]);
        assert_eq!(r.code, 0);
        assert!(r.stdout.contains("--check") && r.stdout.len() > 500, "{}", r.stdout);
        let r = rythme_lang(lang, &["help"]);
        assert!(r.stdout.contains("rythme fmt"), "{}", r.stdout);
    }
}
