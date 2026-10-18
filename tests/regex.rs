//! Text tools: regexes (`matches`, `find_all`, `split_by`, `replace_all`...),
//! `format_number`, `words`, `trim_start`, `trim_end`, `equals_ignore_case`.
//!
//! The regex engine is checked against a simple reference: a matcher that
//! tries one way after the other (slow, but obviously right), on thousands of
//! random regexes and texts.

mod common;

use std::time::{Duration, Instant};

use common::{fails, ok, rythme};
use rythme::builtins::regex::{Captures, Regex};

// ----- the reference matcher -------------------------------------------------

#[derive(Clone, Debug)]
enum R {
    Lit(char),
    Dot,
    Set(bool, Vec<char>),
    Start,
    End,
    Boundary(bool),
    Group(Option<usize>, Box<R>),
    Cat(Vec<R>),
    Alt(Vec<R>),
    Rep(Box<R>, u32, Option<u32>, bool),
}

fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

type K<'k> = &'k mut dyn FnMut(&mut Ref, usize) -> bool;

/// Tries the ways in order of preference, as Perl does; a `*` stops when a
/// turn reads nothing.
struct Ref {
    text: Vec<char>,
    caps: Vec<Option<usize>>,
}

impl Ref {
    fn m(&mut self, r: &R, p: usize, k: K) -> bool {
        let c = self.text.get(p).copied();
        match r {
            R::Lit(x) => c == Some(*x) && k(self, p + 1),
            R::Dot => c.is_some_and(|c| c != '\n') && k(self, p + 1),
            R::Set(neg, set) => c.is_some_and(|c| set.contains(&c) != *neg) && k(self, p + 1),
            R::Start => p == 0 && k(self, p),
            R::End => p == self.text.len() && k(self, p),
            R::Boundary(yes) => {
                let before = p > 0 && word(self.text[p - 1]);
                let after = c.is_some_and(word);
                ((before != after) == *yes) && k(self, p)
            }
            R::Group(None, inner) => self.m(inner, p, k),
            R::Group(Some(i), inner) => {
                let (s, e) = (2 * i, 2 * i + 1);
                let old = (self.caps[s], self.caps[e]);
                self.caps[s] = Some(p);
                let found = self.m(inner, p, &mut |me: &mut Ref, q| {
                    let before = me.caps[e];
                    me.caps[e] = Some(q);
                    if k(me, q) {
                        return true;
                    }
                    me.caps[e] = before;
                    false
                });
                if !found {
                    (self.caps[s], self.caps[e]) = old;
                }
                found
            }
            R::Cat(items) => self.cat(items, p, k),
            R::Alt(alts) => alts.iter().any(|a| self.m(a, p, k)),
            R::Rep(node, min, max, greedy) => self.rep(node, *min, *max, *greedy, 0, p, k),
        }
    }

    fn cat(&mut self, items: &[R], p: usize, k: K) -> bool {
        match items.split_first() {
            None => k(self, p),
            Some((first, rest)) => self.m(first, p, &mut |me: &mut Ref, q| me.cat(rest, q, k)),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rep(&mut self, node: &R, min: u32, max: Option<u32>, greedy: bool, done: u32, p: usize, k: K) -> bool {
        if done < min {
            return self.m(node, p, &mut |me: &mut Ref, q| me.rep(node, min, max, greedy, done + 1, q, k));
        }
        let again = match max {
            Some(max) => done < max,
            None => true,
        };
        let more = |me: &mut Ref, k: K| {
            again
                && me.m(node, p, &mut |me: &mut Ref, q| {
                    // A `*` stops when a turn reads nothing.
                    (max.is_some() || q != p) && me.rep(node, min, max, greedy, done + 1, q, k)
                })
        };
        // The longest first (one more turn), or the shortest first (stop here).
        let order = if greedy { [true, false] } else { [false, true] };
        order.into_iter().any(|another| if another { more(self, k) } else { k(self, p) })
    }

    fn run(r: &R, groups: usize, text: &str, start: usize, whole: bool) -> Option<Captures> {
        let mut me = Ref { text: text.chars().collect(), caps: vec![None; 2 * groups] };
        let len = me.text.len();
        let starts = if whole { start..=start } else { start..=len };
        for s in starts {
            me.caps.iter_mut().for_each(|c| *c = None);
            let mut found = None;
            let root = R::Group(Some(0), Box::new(r.clone()));
            if me.m(&root, s, &mut |me: &mut Ref, q| {
                if whole && q != len {
                    return false;
                }
                found = Some(me.caps.clone());
                true
            }) {
                let caps = found.unwrap();
                return Some(caps.chunks(2).map(|g| g[0].zip(g[1])).collect());
            }
        }
        None
    }
}

// ----- random regexes ------------------------------------------------------------

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
}

const ALPHABET: [char; 4] = ['a', 'b', 'c', ' '];

struct Gen {
    rng: Rng,
    groups: usize,
}

impl Gen {
    /// A regex and its meaning.
    fn regex(&mut self, depth: usize) -> (String, R) {
        let choice = if depth == 0 { self.rng.below(5) } else { self.rng.below(12) };
        match choice {
            0 | 1 => {
                let c = ALPHABET[self.rng.below(3)];
                (c.to_string(), R::Lit(c))
            }
            2 => (".".into(), R::Dot),
            3 => {
                let neg = self.rng.below(3) == 0;
                let mut set: Vec<char> = ALPHABET.iter().copied().filter(|_| self.rng.below(2) == 0).collect();
                if set.is_empty() {
                    set.push('a');
                }
                let shown: String = set.iter().collect();
                (format!("[{}{shown}]", if neg { "^" } else { "" }), R::Set(neg, set))
            }
            4 => match self.rng.below(6) {
                0 => ("^".into(), R::Start),
                1 => ("$".into(), R::End),
                2 => ("\\b".into(), R::Boundary(true)),
                3 => ("\\B".into(), R::Boundary(false)),
                4 => ("\\w".into(), R::Set(false, vec!['a', 'b', 'c'])),
                _ => ("\\s".into(), R::Set(false, vec![' ', '\n'])),
            },
            5 => {
                let i = self.groups;
                self.groups += 1;
                let (p, r) = self.regex(depth - 1);
                (format!("({p})"), R::Group(Some(i), Box::new(r)))
            }
            6 | 7 => {
                let n = 2 + self.rng.below(2);
                let parts: Vec<(String, R)> = (0..n).map(|_| self.regex(depth - 1)).collect();
                let shown: Vec<String> = parts.iter().map(|(p, _)| format!("(?:{p})")).collect();
                (shown.concat(), R::Cat(parts.into_iter().map(|(_, r)| r).collect()))
            }
            8 => {
                let n = 2 + self.rng.below(2);
                let parts: Vec<(String, R)> = (0..n).map(|_| self.regex(depth - 1)).collect();
                let shown: Vec<&str> = parts.iter().map(|(p, _)| p.as_str()).collect();
                (format!("(?:{})", shown.join("|")), R::Alt(parts.into_iter().map(|(_, r)| r).collect()))
            }
            _ => {
                let (p, r) = self.regex(depth - 1);
                let (q, min, max) = match self.rng.below(6) {
                    0 => ("*".to_string(), 0, None),
                    1 => ("+".to_string(), 1, None),
                    2 => ("?".to_string(), 0, Some(1)),
                    3 => {
                        let n = self.rng.below(3) as u32;
                        (format!("{{{n}}}"), n, Some(n))
                    }
                    4 => {
                        let n = self.rng.below(3) as u32;
                        (format!("{{{n},}}"), n, None)
                    }
                    _ => {
                        let n = self.rng.below(3) as u32;
                        let m = n + self.rng.below(3) as u32;
                        (format!("{{{n},{m}}}"), n, Some(m))
                    }
                };
                let greedy = self.rng.below(3) != 0;
                let lazy = if greedy { "" } else { "?" };
                (format!("(?:{p}){q}{lazy}"), R::Rep(Box::new(r), min, max, greedy))
            }
        }
    }
}

fn random_text(rng: &mut Rng) -> String {
    let len = rng.below(8);
    (0..len).map(|_| if rng.below(20) == 0 { '\n' } else { ALPHABET[rng.below(4)] }).collect()
}

/// Can it match an empty text?
fn nullable(r: &R) -> bool {
    match r {
        R::Lit(_) | R::Dot | R::Set(..) => false,
        R::Start | R::End | R::Boundary(_) => true,
        R::Group(_, inner) => nullable(inner),
        R::Cat(items) => items.iter().all(nullable),
        R::Alt(alts) => alts.iter().any(nullable),
        R::Rep(node, min, _, _) => *min == 0 || nullable(node),
    }
}

/// Does it repeat without limit a part that can match an empty text, like
/// `(a*)*`? There, a way that reaches a step already reached at the same
/// place is dropped, while the reference tries it again: the match found
/// starts at the same place, but it may end elsewhere.
fn loops_on_empty(r: &R) -> bool {
    match r {
        R::Group(_, inner) => loops_on_empty(inner),
        R::Cat(items) | R::Alt(items) => items.iter().any(loops_on_empty),
        R::Rep(node, _, max, _) => (max.is_none() && nullable(node)) || loops_on_empty(node),
        _ => false,
    }
}

fn start_of(m: &Option<Captures>) -> Option<usize> {
    m.as_ref().map(|m| m[0].unwrap().0)
}

/// The engine and the reference agree on every random regex: whether the
/// whole text matches, and where the leftmost match and its groups are, from
/// every starting position. For a regex like `(a*)*`, they agree on whether
/// there is a match and where it starts.
#[test]
fn the_regex_engine_agrees_with_a_simple_reference() {
    let cases: usize = std::env::var("RYTHME_REGEX_CASES").ok().and_then(|n| n.parse().ok()).unwrap_or(3000);
    let mut rng = Rng(0x5eed_1234_abcd_ef01);
    let (mut exact, mut found) = (0, 0);
    for _ in 0..cases {
        let mut g = Gen { rng: Rng(rng.next() | 1), groups: 1 };
        let (pattern, meaning) = g.regex(4);
        let re = Regex::new(&pattern).unwrap_or_else(|e| panic!("{pattern}: {}", e.message));
        assert_eq!(re.groups(), g.groups, "{pattern}");
        let strict = !loops_on_empty(&meaning);
        for _ in 0..6 {
            let text = random_text(&mut rng);
            let expected = Ref::run(&meaning, g.groups, &text, 0, true);
            let got = re.whole(&text);
            assert_eq!(got.is_some(), expected.is_some(), "matches({text:?}, {pattern:?})");
            if strict {
                assert_eq!(got, expected, "matches({text:?}, {pattern:?})");
            }
            for start in 0..=text.len() {
                let expected = Ref::run(&meaning, g.groups, &text, start, false);
                let got = re.find_at(&text, start);
                assert_eq!(start_of(&got), start_of(&expected), "search in {text:?} from {start} for {pattern:?}");
                if strict {
                    assert_eq!(got, expected, "search in {text:?} from {start} for {pattern:?}");
                }
                found += usize::from(expected.is_some());
            }
            exact += usize::from(strict);
        }
    }
    // Enough cases are compared exactly, and enough of them match.
    assert!(exact >= cases * 3 && found >= cases * 6, "{exact} exact, {found} found");
}

/// A search takes a time proportional to the length of the text, even for
/// the regexes that make engines that try one way after the other take
/// forever.
#[test]
fn no_regex_takes_forever() {
    let text = "a".repeat(20_000);
    let start = Instant::now();
    for pattern in ["(a*)*b", "(a|a)*b", "(a+)+b", "(?:a|aa)*c", "(a*)*(a*)*(a*)*$x", "(.*a){12}b"] {
        let re = Regex::new(pattern).unwrap();
        assert_eq!(re.whole(&text), None, "{pattern}");
        assert_eq!(re.find_at(&text, 0), None, "{pattern}");
    }
    assert!(start.elapsed() < Duration::from_secs(60), "took {:?}", start.elapsed());
    ok("print(matches(repeat(\"a\", 5000), \"(a*)*b\"), len(find_all(repeat(\"ab\", 5000), \"(a|b)+?\")))", "false 10000");
}

// ----- the regexes in Rythme --------------------------------------------------------

#[test]
fn searching_with_regexes() {
    ok(
        "print(matches(\"2024\", \"[0-9]+\"), matches(\"2024!\", \"[0-9]+\"), matches(\"ab\", \"a|ab\"), matches(\"\", \"x*\"))",
        "true false true true",
    );
    ok("print(find_first(\"tel: 06 12\", \"[0-9]+\"), find_first(\"none\", \"[0-9]\"))", "Some(\"06\") None");
    ok(
        "print(find_all(\"3 cats, 12 dogs\", \"[0-9]+\"), find_all(\"\", \"a\"), find_all(\"baaa\", \"a*\"))",
        "[\"3\", \"12\"] [] [\"\", \"aaa\"]",
    );
    ok("print(find_all(\"x<b>y</b>\", \"<.+?>\"), find_all(\"x<b>y</b>\", \"<.+>\"))", "[\"<b>\", \"</b>\"] [\"<b>y</b>\"]");
    // Unicode: letters with accents are letters, and the case is ignored with (?i).
    ok(
        "print(find_all(\"Été, déjà-vu 42\", \"\\\\w+\"), matches(\"ÉTÉ\", \"(?i)été\"), find_all(\"Ab aB\", \"(?i)ab\"))",
        "[\"Été\", \"déjà\", \"vu\", \"42\"] true [\"Ab\", \"aB\"]",
    );
    ok(
        "print(find_all(\"😀a😀\", \".\"), find_all(\"a\\nb\", \".\"), find_all(\"a b\\tc\", \"\\\\S\"))",
        "[\"😀\", \"a\", \"😀\"] [\"a\", \"b\"] [\"a\", \"b\", \"c\"]",
    );
    ok(
        "print(find_all(\"a.b.c\", \"[.]\"), find_all(\"a.b\", \"\\\\.\"), find_all(\"(x)\", \"[()]\"), find_all(\"1\\{2\\}\", \"[\\{]\"))",
        "[\".\", \".\"] [\".\"] [\"(\", \")\"] [\"\\{\"]",
    );
    ok(
        "print(find_all(\"cat concat cats\", \"\\\\bcat\\\\b\"), matches(\"aaa\", \"a\\{3\\}\"), matches(\"aaaa\", \"a\\{2,3\\}\"))",
        "[\"cat\"] true false",
    );
    ok("print(find_all(\"a]b-c\", \"[]-]\"), find_all(\"x^y\", \"[x^]\"))", "[\"]\", \"-\"] [\"x\", \"^\"]");
}

#[test]
fn groups_and_captures() {
    ok("print(captures(\"2024-02-29\", \"([0-9]+)-([0-9]+)\"))", "Some([\"2024-02\", \"2024\", \"02\"])");
    ok("print(captures(\"ab\", \"(x)|(a)b\"), captures(\"zz\", \"(a)\"))", "Some([\"ab\", \"\", \"a\"]) None");
    ok("print(captures_all(\"a=1, b=2\", \"(\\\\w)=(\\\\d)\"))", "[[\"a=1\", \"a\", \"1\"], [\"b=2\", \"b\", \"2\"]]");
    ok("print(named_captures(\"on 2024-02\", \"(?<year>[0-9]+)-(?<month>[0-9]+)\"))", "Some({year: \"2024\", month: \"02\"})");
    ok("print(named_captures(\"x\", \"(?P<n>[0-9])\"), named_captures(\"1\", \"(?:a)|(?<n>[0-9])\"))", "None Some({n: \"1\"})");
    fails("named_captures(\"x\", \"([0-9])\")", "E023");
    // The checker knows the fields.
    ok("d = unwrap(named_captures(\"1-2\", \"(?<a>.)-(?<b>.)\"))\nprint(d.a + d.b)", "12");
}

#[test]
fn splitting_and_replacing() {
    ok(
        "print(split_by(\"a, b;c\", \"[,;] *\"), split_by(\"abc\", \"x*\"), split_by(\"\", \",\"), split_by(\",a,\", \",\"))",
        "[\"a\", \"b\", \"c\"] [\"abc\"] [\"\"] [\"\", \"a\", \"\"]",
    );
    ok("print(replace_all(\"29/02/2024\", \"(\\\\d+)/(\\\\d+)/(\\\\d+)\", \"$3-$2-$1\"))", "2024-02-29");
    ok("print(replace_all(\"a cat\", \"\\\\b\\\\w\", upper), replace_all(\"abc\", \"x*\", \"-\"), replace_all(\"a1\", \"[0-9]\", \"$$$0\"))", "A Cat -a-b-c- a$1");
    ok(
        "print(replace_all(\"2024-02\", \"(?<y>\\\\d+)-(?<m>\\\\d+)\", \"$<m>/$<y>\"), replace_all(\"ab\", \"(x)?b\", \"[$1]\"))",
        "02/2024 a[]",
    );
    ok("print(replace_all(\"1 2 3\", \"[0-9]\", fn(n) = text(int(n) * 2)))", "2 4 6");
    let r = fails("replace_all(\"1 2\", \"[0-9]\", fn(n) = int(n))", "E005");
    assert!(r.stderr.contains("must return a text"), "{}", r.stderr);
    fails("replace_all(\"a\", \"a\", 5)", "E005");
}

/// A mistake in a regex shows where it is: in the code when the regex is
/// written there (found by the checker), in the regex itself otherwise.
#[test]
fn mistakes_in_regexes_are_shown() {
    for (pattern, message, at, len) in [
        ("a(b", "this `(` is never closed", 1, 1),
        ("a)b", "this `)` closes nothing", 1, 1),
        ("[a-z", "this `[` is never closed", 0, 1),
        ("*a", "`*` has nothing to repeat", 0, 1),
        ("a|+", "`+` has nothing to repeat", 2, 1),
        ("^*", "`*` has nothing to repeat", 1, 1),
        ("a**", "a repetition cannot be repeated again", 2, 1),
        ("a{2", "`{` must start a count", 1, 1),
        ("{2}", "`{2}` has nothing to repeat", 0, 3),
        ("a{5,2}", "the count goes backwards", 1, 5),
        ("a{1001}", "a count can be at most 1000", 1, 6),
        ("[z-a]", "the range `z-a` goes backwards", 1, 3),
        ("\\q", "`\\q` means nothing in a regex", 0, 2),
        ("[\\b]", "`\\b` means nothing inside `[...]`", 1, 2),
        ("ab\\", "the regex ends with a lone `\\`", 2, 1),
        ("a(?i)", "`(?i)` must be at the very start", 1, 4),
        ("(?x)", "unknown kind of group", 0, 3),
        ("(?<1a>x)", "must be followed by a name", 0, 6),
        ("(?<n>a)(?<n>b)", "two groups are named `n`", 10, 1),
        ("(a{1000}){1000}", "too large", 0, 15),
    ] {
        rythme::lang::set(rythme::lang::Lang::En);
        let e = Regex::new(pattern).expect_err(pattern);
        assert!(e.message.contains(message), "{pattern}: {}", e.message);
        assert_eq!((e.at, e.len), (at, len), "{pattern}: {}", e.message);
    }
    // Before running, at its place in the code.
    let r = fails("print(matches(\"AB-12\", \"[A-Z]+-([0-9]+\"))", "E026");
    assert!(r.stderr.contains("1:32") && r.stderr.contains("^ opened here"), "{}", r.stderr);
    let r = fails("print(replace_all(\"a\", \"(a)\", \"$2\"))", "E026");
    assert!(r.stderr.contains("there is no group 2: the regex has 1 group") && r.stderr.contains("^^ no such group"), "{}", r.stderr);
    // An escape before it: the place is still right.
    let r = fails("print(matches(\"a\", \"\\\\d\\\\q\"))", "E026");
    assert!(r.stderr.contains("1:24") && r.stderr.contains("^^^ unknown"), "{}", r.stderr);
    // While running: in the regex.
    let r = fails("p = \"a[b\" + \"\"\nprint(find_all(\"x\", p))", "E026");
    assert!(r.stderr.contains("here in the regex:\n        a[b\n         ^"), "{}", r.stderr);
    let r = fails("r = \"$x\" + \"\"\nprint(replace_all(\"x\", \"x\", r))", "E026");
    assert!(r.stderr.contains("here in the replacement:"), "{}", r.stderr);
    // `\d` written with one backslash.
    let r = fails("find_all(\"a1\", \"\\d\")", "E018");
    assert!(r.stderr.contains("in a regex, write the backslash twice"), "{}", r.stderr);
    // In French.
    let r = common::rythme_lang("fr", &["-e", "matches(\"a\", \"(a\")"]);
    assert!(r.stderr.contains("regex invalide : cette `(` n'est jamais fermée"), "{}", r.stderr);
}

// ----- the other text tools ---------------------------------------------------------

#[test]
fn numbers_written_for_people() {
    ok(
        "print(format_number(1234567), format_number(-1234567), format_number(999), format_number(1000), format_number(0))",
        "1,234,567 -1,234,567 999 1,000 0",
    );
    ok(
        "print(format_number(1234.5, 2), format_number(1234.5678, 2, \"fr\"), format_number(1234567.891, 1, \"de\"), format_number(5, 2, \"fr-CA\"))",
        "1,234.50 1 234,57 1.234.567,9 5,00",
    );
    ok(
        "print(format_number(1234.5), format_number(-0.001, 2), format_number(1e300), format_number(-9223372036854775807 - 1))",
        "1,234.5 0.00 1e300 -9,223,372,036,854,775,808",
    );
    ok("print(pad_left(format_number(1234.5, 2), 10))", "  1,234.50");
    // Whole numbers beyond 64 bits, exactly.
    ok("print(format_number(10 ** 20), format_number(-(2 ** 64), 1, \"fr\"))", "100,000,000,000,000,000,000 -18 446 744 073 709 551 616,0");
    // The Ruby names, and the text tools as methods with blocks.
    ok(
        "print(\"  hi  \".lstrip + \"|\", \"  hi  \".rstrip + \"|\", \"Été\".casecmp?(\"éTÉ\"), \"a cat\".replace_all(\"\\\\w+\") { |w| w.upcase })",
        "hi  |   hi| true A CAT",
    );
    fails("format_number(\"12\")", "E005");
    fails("format_number(12, 1.5)", "E005");
    fails("format_number(12, -1)", "E023");
    fails("format_number(12, 101)", "E023");
    let r = fails("format_number(12, 0, \"xx\")", "E023");
    assert!(r.stderr.contains("\"fr\" (1 234,5)"), "{}", r.stderr);
}

#[test]
fn trimming_words_and_case() {
    ok("print(\"[{trim_start(\"  a b \\n\")}]\", \"[{trim_end(\"\\t a b  \")}]\", \"[{trim_start(\"\")}]\")", "[a b \n] [\t a b] []");
    ok(
        "print(words(\"Hello, world! It's me — aujourd'hui, well-known 42.\"), words(\"\"), words(\" -- \"), words(\"e\\u{301}t\\u{e9}\"))",
        "[\"Hello\", \"world\", \"It's\", \"me\", \"aujourd'hui\", \"well-known\", \"42\"] [] [] [\"e\u{301}té\"]",
    );
    ok("print(words(\"a- b 'c' d’e\"))", "[\"a\", \"b\", \"c\", \"d’e\"]");
    ok(
        "print(equals_ignore_case(\"Été\", \"éTÉ\"), equals_ignore_case(\"Straße\", \"STRASSE\"), equals_ignore_case(\"a\", \"b\"), equals_ignore_case(\"\", \"\"))",
        "true true false true",
    );
    fails("equals_ignore_case(\"a\", 1)", "E005");
    fails("words(5)", "E005");
}

#[test]
fn the_checker_knows_the_text_tools() {
    let dir = std::env::temp_dir().join(format!("rythme-regex-check-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for code in
        ["trim_start(5)", "matches(\"a\", 1)", "find_all([1], \"a\")", "replace_all(\"a\", \"a\", 5)", "format_number(\"1\")", "words(1)"]
    {
        let file = dir.join("check.ry");
        std::fs::write(&file, format!("fn f() = {code}\n")).unwrap();
        let r = rythme(&["check", file.to_str().unwrap()]);
        assert!(r.code != 0 && r.stderr.contains("E005"), "{code}: {}{}", r.stdout, r.stderr);
    }
    let _ = std::fs::remove_dir_all(&dir);
    let r = fails("find_first([1, 2], \"a\")", "E005");
    assert!(r.stderr.contains("for a list, use `find(list, f)`"), "{}", r.stderr);
}
