//! Regular expressions ("regex"): a small language to describe pieces of
//! text, for `matches`, `find_all`, `split_by`, `replace_all`...
//!
//! Written by hand, without a library, with the method of Ken Thompson and
//! Rob Pike: the regex becomes a small program ([`Inst`]), and the text is
//! read once, from left to right, following every possible way through the
//! program at the same time. A way that reaches a step of the program that
//! another way already reached at the same place in the text is dropped: the
//! work for one search is at most (length of the text) × (size of the
//! program), and no regex can make a search take forever (no "catastrophic
//! backtracking", as in the engines that try one way after the other).
//!
//! The answers are those of the usual engines (Perl, Python, JavaScript,
//! Rust): the leftmost match wins; at the same place, the first alternative
//! of `a|b` and the longest repetition (the shortest for `*?`, `+?`, `??`).
//!
//! The subset: characters, `.`, `[a-z]` and `[^...]`, `\d \w \s` and their
//! opposites, `\b`, `^ $`, groups `(...)`, `(?:...)` and `(?<name>...)`,
//! `|`, `* + ?` and `{n}` `{n,}` `{n,m}` (with `?` after them for the
//! shortest), and `(?i)` at the start to ignore the case.

use std::cell::RefCell;

use super::*;

/// The largest count of `{n,m}`.
const MAX_COUNT: u32 = 1000;
/// The largest program: `(a{1000}){1000}` would be a million steps.
const MAX_INSTS: usize = 100_000;
/// Groups nested deeper than this are refused.
const MAX_DEPTH: usize = 100;

/// A mistake in a regex (or in the replacement of `replace_all`): where it
/// is, in characters, and what is wrong.
#[derive(Clone, Debug, PartialEq)]
pub struct RegexError {
    /// The first character of the mistake, counted from 0.
    pub at: usize,
    /// How many characters it takes (at least 1 when shown).
    pub len: usize,
    pub message: String,
    /// A short label, shown under the mistake.
    pub label: String,
    pub help: Option<String>,
}

fn error(at: usize, len: usize, message: String, label: String, help: Option<String>) -> RegexError {
    RegexError { at, len, message, label, help }
}

// ----- syntax --------------------------------------------------------------

/// One part of a class `[...]`, or `\d`, `\w`, `\s` (true: the opposite).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Item {
    Range(char, char),
    Digit(bool),
    Word(bool),
    Space(bool),
}

pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl Item {
    fn has(self, c: char) -> bool {
        match self {
            Item::Range(lo, hi) => lo <= c && c <= hi,
            Item::Digit(not) => c.is_ascii_digit() != not,
            Item::Word(not) => is_word_char(c) != not,
            Item::Space(not) => c.is_whitespace() != not,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Class {
    negated: bool,
    items: Vec<Item>,
    /// The answers of `matches` for the ASCII characters, one bit each:
    /// most texts are mostly ASCII, and a bit is faster than the items.
    ascii: u128,
}

impl Class {
    fn matches(&self, c: char, fold: bool) -> bool {
        if c.is_ascii() {
            return self.ascii & (1 << c as u32) != 0;
        }
        self.matches_slowly(c, fold)
    }

    fn matches_slowly(&self, c: char, fold: bool) -> bool {
        let has = |c: char| self.items.iter().any(|i| i.has(c));
        let found = has(c) || (fold && (has(lower(c)) || has(upper(c)) || has(fold_case(c))));
        found != self.negated
    }

    /// Writes down the answers for the ASCII characters (see `ascii`).
    fn learn_ascii(&mut self, fold: bool) {
        self.ascii = (0..128u8).filter(|&b| self.matches_slowly(char::from(b), fold)).fold(0, |bits, b| bits | (1 << b));
    }
}

/// The lowercase of a character, when it is one character.
fn lower(c: char) -> char {
    let mut l = c.to_lowercase();
    match (l.next(), l.next()) {
        (Some(x), None) => x,
        _ => c,
    }
}

fn upper(c: char) -> char {
    let mut u = c.to_uppercase();
    match (u.next(), u.next()) {
        (Some(x), None) => x,
        _ => c,
    }
}

/// The character as compared: its case folded when the case is ignored.
#[inline]
fn fold_if(c: char, fold: bool) -> char {
    if fold {
        fold_case(c)
    } else {
        c
    }
}

/// The same character for every case: `A`, `a` → `a`; `Σ`, `σ`, `ς` → `σ`.
fn fold_case(c: char) -> char {
    // The case of an ASCII character is an ASCII character: the same answer, faster.
    if c.is_ascii() {
        return c.to_ascii_lowercase();
    }
    lower(upper(c))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Assert {
    Start,
    End,
    Boundary,
    NotBoundary,
}

#[derive(Clone, Debug)]
enum Node {
    Empty,
    Char(char),
    Any,
    Class(usize),
    Assert(Assert),
    /// A group, with its number when it captures.
    Group(Option<usize>, Box<Node>),
    Concat(Vec<Node>),
    Alt(Vec<Node>),
    Repeat {
        node: Box<Node>,
        min: u32,
        max: Option<u32>,
        greedy: bool,
    },
}

/// What a `\` gives.
enum Escape {
    Char(char),
    Item(Item),
    Assert(Assert),
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
    /// The names of the groups, by number (0 is the whole match).
    names: Vec<Option<Rc<str>>>,
    classes: Vec<Class>,
    depth: usize,
}

fn sign_help(c: char) -> String {
    if c == '{' || c == '}' {
        // Braces are written `\{` and `\}` in a Rythme text.
        return t!(
            "for the sign `{c}` itself, write `[{c}]` (in a Rythme text: \"[\\{c}]\")",
            "pour le signe `{c}` lui-même, écrivez `[{c}]` (dans un texte Rythme : \"[\\{c}]\")"
        );
    }
    t!("for the sign `{c}` itself, write `[{c}]`", "pour le signe `{c}` lui-même, écrivez `[{c}]`")
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_at(&self, i: usize) -> Option<char> {
        self.chars.get(self.pos + i).copied()
    }

    fn alt(&mut self) -> Result<Node, RegexError> {
        let mut alts = vec![self.concat()?];
        while self.peek() == Some('|') {
            self.pos += 1;
            alts.push(self.concat()?);
        }
        Ok(if alts.len() == 1 { alts.pop().unwrap_or(Node::Empty) } else { Node::Alt(alts) })
    }

    fn concat(&mut self) -> Result<Node, RegexError> {
        let mut items = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            let atom = self.atom()?;
            items.push(self.repeated(atom)?);
        }
        Ok(match items.len() {
            0 => Node::Empty,
            1 => items.pop().unwrap_or(Node::Empty),
            _ => Node::Concat(items),
        })
    }

    fn nothing_to_repeat(&self, at: usize, len: usize) -> RegexError {
        let q: String = self.chars[at..at + len].iter().collect();
        let sign = self.chars[at];
        error(
            at,
            len,
            t!("`{q}` has nothing to repeat", "`{q}` n'a rien à répéter"),
            t!("nothing before it to repeat", "rien à répéter avant"),
            Some(t!(
                "put it after a character or a group, like `a{q}` or `(ab){q}`; {}",
                "mettez-le après un caractère ou un groupe, comme `a{q}` ou `(ab){q}` ; {}",
                sign_help(sign)
            )),
        )
    }

    fn atom(&mut self) -> Result<Node, RegexError> {
        let at = self.pos;
        let Some(c) = self.peek() else { return Ok(Node::Empty) };
        self.pos += 1;
        Ok(match c {
            '(' => self.group(at)?,
            '[' => self.class(at)?,
            '.' => Node::Any,
            '^' => Node::Assert(Assert::Start),
            '$' => Node::Assert(Assert::End),
            '\\' => match self.escape(at, false)? {
                Escape::Char(c) => Node::Char(c),
                Escape::Item(i) => {
                    self.classes.push(Class { negated: false, items: vec![i], ascii: 0 });
                    Node::Class(self.classes.len() - 1)
                }
                Escape::Assert(a) => Node::Assert(a),
            },
            '*' | '+' | '?' => return Err(self.nothing_to_repeat(at, 1)),
            '{' => {
                self.pos = at;
                let _ = self.count()?;
                return Err(self.nothing_to_repeat(at, self.pos - at));
            }
            c => Node::Char(c),
        })
    }

    /// `{n}`, `{n,}` or `{n,m}`, starting at the `{`.
    fn count(&mut self) -> Result<(u32, Option<u32>), RegexError> {
        let open = self.pos;
        let bad = |p: &Parser| {
            error(
                open,
                1,
                t!("`{{` must start a count, like {{3}} or {{2,5}}", "`{{` doit commencer un nombre de fois, comme {{3}} ou {{2,5}}"),
                t!("not a count", "pas un nombre de fois"),
                Some(t!(
                    "{}; in a Rythme text, braces are written `\\{{` and `\\}}`, like \"[0-9]\\{{4\\}}\"",
                    "{} ; dans un texte Rythme, les accolades s'écrivent `\\{{` et `\\}}`, comme \"[0-9]\\{{4\\}}\"",
                    sign_help(p.chars[open])
                )),
            )
        };
        self.pos += 1;
        let Some(min) = self.number() else {
            let e = bad(self);
            self.pos = open;
            return Err(e);
        };
        let max = if self.peek() == Some(',') {
            self.pos += 1;
            self.number()
        } else {
            Some(min)
        };
        if self.peek() != Some('}') {
            let e = bad(self);
            self.pos = open;
            return Err(e);
        }
        self.pos += 1;
        let len = self.pos - open;
        if min > u64::from(MAX_COUNT) || max.is_some_and(|m| m > u64::from(MAX_COUNT)) {
            return Err(error(
                open,
                len,
                t!("a count can be at most {MAX_COUNT}", "un nombre de fois peut valoir au plus {MAX_COUNT}"),
                t!("too large", "trop grand"),
                Some(t!("for more, use `*` or `+`, or {{n,}} without a maximum", "pour plus, utilisez `*` ou `+`, ou {{n,}} sans maximum")),
            ));
        }
        if let Some(max) = max.filter(|&m| m < min) {
            return Err(error(
                open,
                len,
                t!(
                    "the count goes backwards: {min} is more than {max}",
                    "le nombre de fois va à l'envers : {min} est plus grand que {max}"
                ),
                t!("smallest first", "le plus petit d'abord"),
                Some(t!("write the smallest number first: {{{max},{min}}}", "écrivez le plus petit nombre d'abord : {{{max},{min}}}")),
            ));
        }
        Ok((min as u32, max.map(|m| m as u32)))
    }

    fn number(&mut self) -> Option<u64> {
        let start = self.pos;
        let mut n: u64 = 0;
        while let Some(d) = self.peek().and_then(|c| c.to_digit(10)) {
            n = n.saturating_mul(10).saturating_add(u64::from(d));
            self.pos += 1;
        }
        (self.pos > start).then_some(n)
    }

    /// `*`, `+`, `?` or a count after an atom, then `?` for the shortest.
    fn repeated(&mut self, atom: Node) -> Result<Node, RegexError> {
        let at = self.pos;
        let (min, max) = match self.peek() {
            Some('*') => (0, None),
            Some('+') => (1, None),
            Some('?') => (0, Some(1)),
            Some('{') => self.count()?,
            _ => return Ok(atom),
        };
        if self.pos == at {
            self.pos += 1;
        }
        if matches!(atom, Node::Assert(_) | Node::Empty) {
            return Err(self.nothing_to_repeat(at, self.pos - at));
        }
        let greedy = if self.peek() == Some('?') {
            self.pos += 1;
            false
        } else {
            true
        };
        if let Some(c @ ('*' | '+' | '?' | '{')) = self.peek() {
            let here = self.pos;
            let len = if c == '{' && self.count().is_ok() { self.pos - here } else { 1 };
            return Err(error(
                here,
                len,
                t!("a repetition cannot be repeated again", "une répétition ne peut pas être répétée encore"),
                t!("repeats a repetition", "répète une répétition"),
                Some(t!(
                    "put the repeated part in parentheses, like `(a+)*`, or keep one sign, like `a*`; {}",
                    "mettez la partie répétée entre parenthèses, comme `(a+)*`, ou gardez un seul signe, comme `a*` ; {}",
                    sign_help(c)
                )),
            ));
        }
        Ok(Node::Repeat { node: Box::new(atom), min, max, greedy })
    }

    fn group(&mut self, at: usize) -> Result<Node, RegexError> {
        if self.depth >= MAX_DEPTH {
            return Err(error(
                at,
                1,
                t!(
                    "the groups are nested too deeply (more than {MAX_DEPTH})",
                    "les groupes sont imbriqués trop profondément (plus de {MAX_DEPTH})"
                ),
                t!("too deep", "trop profond"),
                None,
            ));
        }
        let mut number = None;
        if self.peek() == Some('?') {
            match (self.peek_at(1), self.peek_at(2)) {
                (Some(':'), _) => self.pos += 2,
                (Some('<'), _) | (Some('P'), Some('<')) => {
                    self.pos += if self.peek_at(1) == Some('P') { 3 } else { 2 };
                    let name_at = self.pos;
                    while self.peek().is_some_and(|c| c != '>' && c != ')') {
                        self.pos += 1;
                    }
                    let name: String = self.chars[name_at..self.pos].iter().collect();
                    let valid = name.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_')
                        && name.chars().all(|c| c.is_alphanumeric() || c == '_');
                    if self.peek() != Some('>') || !valid {
                        return Err(error(
                            at,
                            self.pos - at + usize::from(self.peek() == Some('>')),
                            t!("`(?<` must be followed by a name and `>`", "`(?<` doit être suivi d'un nom et de `>`"),
                            t!("not a group name", "pas un nom de groupe"),
                            Some(t!(
                                "a named group looks like `(?<year>[0-9]+)`: the name has letters, digits and `_`",
                                "un groupe nommé ressemble à `(?<annee>[0-9]+)` : le nom a des lettres, des chiffres et des `_`"
                            )),
                        ));
                    }
                    if self.names.iter().flatten().any(|n| **n == *name) {
                        return Err(error(
                            name_at,
                            self.pos - name_at,
                            t!("two groups are named `{name}`", "deux groupes s'appellent `{name}`"),
                            t!("already used", "déjà utilisé"),
                            Some(t!("give each group its own name", "donnez à chaque groupe son propre nom")),
                        ));
                    }
                    self.pos += 1;
                    self.names.push(Some(Rc::from(name)));
                    number = Some(self.names.len() - 1);
                }
                (Some('i'), Some(')')) => {
                    return Err(error(
                        at,
                        4,
                        t!("`(?i)` must be at the very start of the regex", "`(?i)` doit être tout au début de la regex"),
                        t!("not at the start", "pas au début"),
                        Some(t!("`(?i)` ignores the case in the whole regex: write it first", "`(?i)` ignore la casse dans toute la regex : écrivez-le en premier")),
                    ))
                }
                _ => {
                    return Err(error(
                        at,
                        2 + usize::from(self.peek_at(1).is_some()),
                        t!("unknown kind of group", "sorte de groupe inconnue"),
                        t!("unknown", "inconnu"),
                        Some(t!(
                            "the groups are `(...)`, `(?:...)` (a group that captures nothing) and `(?<name>...)` (a named group); `(?i)` at the start ignores the case",
                            "les groupes sont `(...)`, `(?:...)` (un groupe qui ne capture rien) et `(?<nom>...)` (un groupe nommé) ; `(?i)` au début ignore la casse"
                        )),
                    ))
                }
            }
        } else {
            self.names.push(None);
            number = Some(self.names.len() - 1);
        }
        self.depth += 1;
        let inner = self.alt()?;
        self.depth -= 1;
        if self.peek() != Some(')') {
            return Err(error(
                at,
                1,
                t!("this `(` is never closed", "cette `(` n'est jamais fermée"),
                t!("opened here", "ouverte ici"),
                Some(t!("add a `)` where the group ends; {}", "ajoutez une `)` là où le groupe finit ; {}", sign_help('('))),
            ));
        }
        self.pos += 1;
        Ok(Node::Group(number, Box::new(inner)))
    }

    /// After a `\` (at `at`).
    fn escape(&mut self, at: usize, in_class: bool) -> Result<Escape, RegexError> {
        let Some(c) = self.peek() else {
            return Err(error(
                at,
                1,
                t!("the regex ends with a lone `\\`", "la regex finit par un `\\` seul"),
                t!("followed by nothing", "suivi de rien"),
                Some(t!(
                    "a `\\` comes before a letter like d, w, s or b, or before a sign for the sign itself, like `\\.`",
                    "un `\\` se met devant une lettre comme d, w, s ou b, ou devant un signe pour le signe lui-même, comme `\\.`"
                )),
            ));
        };
        self.pos += 1;
        Ok(match c {
            'd' => Escape::Item(Item::Digit(false)),
            'D' => Escape::Item(Item::Digit(true)),
            'w' => Escape::Item(Item::Word(false)),
            'W' => Escape::Item(Item::Word(true)),
            's' => Escape::Item(Item::Space(false)),
            'S' => Escape::Item(Item::Space(true)),
            'b' | 'B' if in_class => {
                return Err(error(
                    at,
                    2,
                    t!("`\\{c}` means nothing inside `[...]`", "`\\{c}` ne veut rien dire dans `[...]`"),
                    t!("not a character", "pas un caractère"),
                    Some(t!("`\\b` is the edge of a word: write it outside of the brackets", "`\\b` est le bord d'un mot : écrivez-le hors des crochets")),
                ))
            }
            'b' => Escape::Assert(Assert::Boundary),
            'B' => Escape::Assert(Assert::NotBoundary),
            'n' => Escape::Char('\n'),
            't' => Escape::Char('\t'),
            'r' => Escape::Char('\r'),
            c if c.is_ascii_punctuation() => Escape::Char(c),
            c => {
                return Err(error(
                    at,
                    2,
                    t!("`\\{c}` means nothing in a regex", "`\\{c}` ne veut rien dire dans une regex"),
                    t!("unknown", "inconnu"),
                    Some(t!(
                        "the known ones: \\d (a digit), \\w (a letter, a digit or _), \\s (a space), \\b (the edge of a word),\ntheir opposites \\D, \\W, \\S, \\B, \\n, \\t, and `\\` before a sign for the sign itself, like \\. or \\(",
                        "ceux qui existent : \\d (un chiffre), \\w (une lettre, un chiffre ou _), \\s (un espace), \\b (le bord d'un mot),\nleurs contraires \\D, \\W, \\S, \\B, \\n, \\t, et `\\` devant un signe pour le signe lui-même, comme \\. ou \\("
                    )),
                ))
            }
        })
    }

    fn class(&mut self, at: usize) -> Result<Node, RegexError> {
        let negated = self.peek() == Some('^');
        if negated {
            self.pos += 1;
        }
        let mut items = Vec::new();
        let mut first = true;
        loop {
            let Some(c) = self.peek() else {
                return Err(error(
                    at,
                    1,
                    t!("this `[` is never closed", "ce `[` n'est jamais fermé"),
                    t!("opened here", "ouvert ici"),
                    Some(t!(
                        "add a `]` after the characters; for the sign `]` itself, put it first: `[]]`",
                        "ajoutez un `]` après les caractères ; pour le signe `]` lui-même, mettez-le en premier : `[]]`"
                    )),
                ));
            };
            let item_at = self.pos;
            self.pos += 1;
            if c == ']' && !first {
                break;
            }
            first = false;
            let lo = if c == '\\' {
                match self.escape(item_at, true)? {
                    Escape::Char(c) => c,
                    Escape::Item(i) => {
                        items.push(i);
                        continue;
                    }
                    Escape::Assert(_) => unreachable!("refused in a class"),
                }
            } else {
                c
            };
            if self.peek() == Some('-') && self.peek_at(1).is_some_and(|c| c != ']') {
                self.pos += 1;
                let hi_at = self.pos;
                let h = self.chars[hi_at];
                self.pos += 1;
                let hi = if h == '\\' {
                    match self.escape(hi_at, true)? {
                        Escape::Char(c) => c,
                        _ => {
                            return Err(error(
                                item_at,
                                self.pos - item_at,
                                t!("a range must end with one character", "un intervalle doit finir par un caractère"),
                                t!("not a range", "pas un intervalle"),
                                Some(t!(
                                    "for the sign `-` itself, put it last: `[a-]`",
                                    "pour le signe `-` lui-même, mettez-le en dernier : `[a-]`"
                                )),
                            ))
                        }
                    }
                } else {
                    h
                };
                if hi < lo {
                    return Err(error(
                        item_at,
                        self.pos - item_at,
                        t!("the range `{lo}-{hi}` goes backwards", "l'intervalle `{lo}-{hi}` va à l'envers"),
                        t!("backwards", "à l'envers"),
                        Some(t!("write the first character first: `{hi}-{lo}`", "écrivez le premier caractère d'abord : `{hi}-{lo}`")),
                    ));
                }
                items.push(Item::Range(lo, hi));
            } else {
                items.push(Item::Range(lo, lo));
            }
        }
        self.classes.push(Class { negated, items, ascii: 0 });
        Ok(Node::Class(self.classes.len() - 1))
    }
}

// ----- the program -----------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Inst {
    Char(char),
    /// Any character but a line break.
    Any,
    Class(usize),
    /// Two ways: the first is preferred.
    Split(usize, usize),
    Jmp(usize),
    /// Remembers the current position in a slot (the start and the end of each group).
    Save(usize),
    Assert(Assert),
    Match,
}

struct Compiler {
    prog: Vec<Inst>,
    fold: bool,
}

struct TooBig;

impl Compiler {
    fn emit(&mut self, i: Inst) -> Result<usize, TooBig> {
        if self.prog.len() >= MAX_INSTS {
            return Err(TooBig);
        }
        self.prog.push(i);
        Ok(self.prog.len() - 1)
    }

    fn node(&mut self, n: &Node) -> Result<(), TooBig> {
        match n {
            Node::Empty => {}
            Node::Char(c) => {
                self.emit(Inst::Char(if self.fold { fold_case(*c) } else { *c }))?;
            }
            Node::Any => {
                self.emit(Inst::Any)?;
            }
            Node::Class(k) => {
                self.emit(Inst::Class(*k))?;
            }
            Node::Assert(a) => {
                self.emit(Inst::Assert(*a))?;
            }
            Node::Group(Some(i), inner) => {
                self.emit(Inst::Save(2 * i))?;
                self.node(inner)?;
                self.emit(Inst::Save(2 * i + 1))?;
            }
            Node::Group(None, inner) => self.node(inner)?,
            Node::Concat(items) => {
                for i in items {
                    self.node(i)?;
                }
            }
            Node::Alt(alts) => {
                let mut jumps = Vec::new();
                for (k, a) in alts.iter().enumerate() {
                    if k + 1 < alts.len() {
                        let split = self.emit(Inst::Split(0, 0))?;
                        self.node(a)?;
                        jumps.push(self.emit(Inst::Jmp(0))?);
                        self.prog[split] = Inst::Split(split + 1, self.prog.len());
                    } else {
                        self.node(a)?;
                    }
                }
                let end = self.prog.len();
                for j in jumps {
                    self.prog[j] = Inst::Jmp(end);
                }
            }
            Node::Repeat { node, min, max, greedy } => {
                // `x{2,}` is `x x x*`, and `x{2,4}` is `x x (x (x)?)?`.
                for _ in 0..*min {
                    self.node(node)?;
                }
                let way = |at: usize, exit: usize| if *greedy { Inst::Split(at + 1, exit) } else { Inst::Split(exit, at + 1) };
                match max {
                    None => {
                        let l = self.emit(Inst::Split(0, 0))?;
                        self.node(node)?;
                        self.emit(Inst::Jmp(l))?;
                        self.prog[l] = way(l, self.prog.len());
                    }
                    Some(max) => {
                        let mut splits = Vec::new();
                        for _ in *min..*max {
                            splits.push(self.emit(Inst::Split(0, 0))?);
                            self.node(node)?;
                        }
                        let exit = self.prog.len();
                        for s in splits {
                            self.prog[s] = way(s, exit);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// A regex, ready to search.
#[derive(Debug)]
pub struct Regex {
    prog: Vec<Inst>,
    classes: Vec<Class>,
    /// Ignore the case: `(?i)` at the start.
    fold: bool,
    /// The names of the groups, by number; group 0 is the whole match.
    names: Vec<Option<Rc<str>>>,
    /// The characters a match can start with, when every match reads one.
    first: Option<First>,
    /// The memory of the last search, kept for the next one: `matches`
    /// called on each line of a long list would allocate it each time.
    spare: RefCell<Option<Vm>>,
}

/// The characters a match can start with (see [`Regex::skip`]).
#[derive(Debug)]
struct First {
    /// The steps that read the first character.
    steps: Vec<usize>,
    /// Whether one of them reads each ASCII character, one bit each.
    ascii: u128,
}

/// Where a match and each of its groups start and end (bytes), by group
/// number: `None` for a group that took no part in the match.
pub type Captures = Vec<Option<(usize, usize)>>;

const NONE: usize = usize::MAX;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    /// The leftmost match, anywhere from the start position.
    Search,
    /// The whole text, from the start position to the end.
    Whole,
}

/// The ways being followed at one position of the text, in order of preference.
#[derive(Debug)]
struct Threads {
    sparse: Vec<usize>,
    dense: Vec<usize>,
    len: usize,
    /// The slots of each way, by step of the program.
    slots: Vec<usize>,
}

impl Threads {
    fn new(n: usize, slots: usize) -> Threads {
        Threads { sparse: vec![0; n], dense: vec![0; n], len: 0, slots: vec![NONE; n * slots] }
    }

    fn contains(&self, pc: usize) -> bool {
        let i = self.sparse[pc];
        i < self.len && self.dense[i] == pc
    }

    fn insert(&mut self, pc: usize) {
        self.sparse[pc] = self.len;
        self.dense[self.len] = pc;
        self.len += 1;
    }
}

#[derive(Debug)]
enum Frame {
    Explore(usize),
    Restore(usize, usize),
}

/// The lists and the scratch space of a search, kept from one search to the
/// next by `find_all`.
#[derive(Debug)]
struct Vm {
    clist: Threads,
    nlist: Threads,
    stack: Vec<Frame>,
    scratch: Vec<usize>,
    /// The slots of the best match found so far.
    best: Vec<usize>,
    track: Track,
}

/// One thing to do while following the ways of a match (see [`Track`]).
#[derive(Clone, Copy, Debug)]
enum Job {
    /// Follow the way at a step and a place.
    Explore(usize, usize),
    /// Give back its value to a slot, when the ways after a `Save` failed.
    Restore(usize, usize),
}

/// The memory of the engine that tries one way after the other, in order of
/// preference, and stops at the first that reaches a match ("backtracking",
/// as Perl does): faster than following every way at the same time, and
/// with the same answers, because it remembers each step and place that it
/// reached. The first way to reach them is the preferred one (as when every
/// way is followed at the same time); a way that comes back to them later is
/// dropped, so no search reads the text more than (length of the text) ×
/// (size of the program) times. Those that failed stay failed for the next
/// searches of `find_all` (see [`Dead`]).
#[derive(Debug, Default)]
struct Track {
    /// One bit per step of the program and place in the text, from `base`.
    bits: Vec<u64>,
    steps: usize,
    base: usize,
    /// The last place reached: the bits after it are all zero.
    hi: usize,
    jobs: Vec<Job>,
}

/// The table of [`Track`] kept between searches (in 64 bits): 64 KB.
const KEEP_TRACK_WORDS: usize = 1 << 13;

impl Track {
    /// Ready to follow `places` places from `base`, with no bit set.
    fn begin(&mut self, steps: usize, base: usize, places: usize) {
        let words = (steps * places).div_ceil(64);
        if self.bits.len() < words {
            self.bits.resize(words, 0);
        }
        self.steps = steps;
        self.base = base;
        self.hi = base;
    }

    /// Was step `pc` at place `pos` reached already? Then it is now.
    #[inline]
    fn seen(&mut self, pc: usize, pos: usize) -> bool {
        let i = (pos - self.base) * self.steps + pc;
        let (word, bit) = (i / 64, 1u64 << (i % 64));
        if self.bits[word] & bit != 0 {
            return true;
        }
        // `hi` is moved by the caller, when it reads a character.
        self.bits[word] |= bit;
        false
    }

    /// The words holding the bits of the places `from..=to`.
    fn words(&self, from: usize, to: usize) -> std::ops::Range<usize> {
        let first = (from - self.base) * self.steps / 64;
        let last = ((to - self.base + 1) * self.steps).div_ceil(64).min(self.bits.len());
        first..last.max(first)
    }

    /// Forgets the steps reached at `pos` (and maybe some around it, which
    /// only costs time).
    fn forget(&mut self, pos: usize) {
        let words = self.words(pos, pos);
        self.bits[words].fill(0);
    }

    /// Sets every bit back to zero, for the next search.
    fn end(&mut self) {
        if self.bits.len() > KEEP_TRACK_WORDS {
            self.bits = Vec::new();
        } else {
            let words = self.words(self.base, self.hi);
            self.bits[words].fill(0);
        }
    }
}

/// The largest table of [`Dead`] (bits): 8 MB.
const MAX_DEAD_BITS: usize = 1 << 26;
/// The most ways [`Dead`] remembers during one search before writing them down.
const MAX_PENDING: usize = 1 << 21;

/// The ways that cannot lead to a match: a step of the program at a place in
/// the text from which no match can be reached, whatever came before. Found
/// by one search of `find_all` and skipped by the next ones, so that finding
/// every match reads the text about once instead of once per match: without
/// it, `a*b|a` on a text of `a`s follows `a*b` to the end of the text for
/// each `a` found. Dropping such a way changes no answer: it never matches,
/// and the ways it would have stopped (the same step at the same place, after
/// it) cannot match either.
///
/// After a search, the ways it followed after the last place where it
/// reached a match (or all of them, when it found none) were followed to
/// their end without reaching one: they are dead.
struct Dead {
    /// One bit per step of the program and place in the text.
    bits: Vec<u64>,
    steps: usize,
    /// The ways followed since the last match of the current search: (place, step).
    pending: Vec<(usize, usize)>,
}

impl Dead {
    fn new(steps: usize, text: &str) -> Option<Dead> {
        let n = steps.checked_mul(text.len() + 1)?;
        (n <= MAX_DEAD_BITS).then(|| Dead { bits: vec![0; n.div_ceil(64)], steps, pending: Vec::new() })
    }

    fn has(&self, pc: usize, pos: usize) -> bool {
        let i = pos * self.steps + pc;
        self.bits[i / 64] & (1 << (i % 64)) != 0
    }

    fn write_down(&mut self) {
        for (pos, pc) in self.pending.drain(..) {
            let i = pos * self.steps + pc;
            self.bits[i / 64] |= 1 << (i % 64);
        }
    }
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, RegexError> {
        let chars: Vec<char> = pattern.chars().collect();
        let fold = pattern.starts_with("(?i)");
        let mut p = Parser { chars, pos: if fold { 4 } else { 0 }, names: vec![None], classes: Vec::new(), depth: 0 };
        let node = p.alt()?;
        if p.pos < p.chars.len() {
            // `concat` and `alt` only stop early at a `)`.
            return Err(error(
                p.pos,
                1,
                t!("this `)` closes nothing", "cette `)` ne ferme rien"),
                t!("no `(` before it", "pas de `(` avant"),
                Some(sign_help(')')),
            ));
        }
        let mut c = Compiler { prog: Vec::new(), fold };
        let compiled = (|| {
            c.emit(Inst::Save(0))?;
            c.node(&node)?;
            c.emit(Inst::Save(1))?;
            c.emit(Inst::Match)
        })();
        if compiled.is_err() {
            return Err(error(
                0,
                p.chars.len(),
                t!(
                    "the regex is too large once its repetitions are written out",
                    "la regex est trop grande une fois ses répétitions écrites"
                ),
                t!("too large", "trop grande"),
                Some(t!(
                    "use smaller counts in {{n,m}}, or `*` and `+`, which cost nothing",
                    "utilisez des nombres plus petits dans {{n,m}}, ou `*` et `+`, qui ne coûtent rien"
                )),
            ));
        }
        let prog = c.prog;
        let mut classes = p.classes;
        for class in &mut classes {
            class.learn_ascii(fold);
        }
        let mut re = Regex { prog, classes, fold, names: p.names, first: None, spare: RefCell::new(None) };
        re.first = re.first_steps().map(|steps| {
            let ascii = (0..128u8).filter(|&b| steps.iter().any(|&pc| re.reads(pc, char::from(b)))).fold(0, |bits, b| bits | (1 << b));
            First { steps, ascii }
        });
        Ok(re)
    }

    /// The steps that can read the first character of a match, or `None`
    /// when a match can be empty: the steps reached from the start without
    /// reading (an assertion is passed as if it held: more steps, never fewer).
    fn first_steps(&self) -> Option<Vec<usize>> {
        let mut seen = vec![false; self.prog.len()];
        let mut todo = vec![0];
        let mut steps = Vec::new();
        while let Some(pc) = todo.pop() {
            if std::mem::replace(&mut seen[pc], true) {
                continue;
            }
            match self.prog[pc] {
                Inst::Jmp(to) => todo.push(to),
                Inst::Split(a, b) => todo.extend([a, b]),
                Inst::Save(_) | Inst::Assert(_) => todo.push(pc + 1),
                Inst::Char(_) | Inst::Any | Inst::Class(_) => steps.push(pc),
                Inst::Match => return None,
            }
        }
        Some(steps)
    }

    /// Does step `pc` read the character `c`?
    #[inline]
    fn reads(&self, pc: usize, c: char) -> bool {
        match self.prog[pc] {
            Inst::Char(x) => fold_if(c, self.fold) == x,
            Inst::Any => c != '\n',
            Inst::Class(k) => self.classes[k].matches(c, self.fold),
            _ => false,
        }
    }

    /// The first place at `pos` or after where a match can start: the next
    /// character that a first step reads (the end of the text if none).
    /// Searching for `word[0-9]+` then only stops at the `w`s.
    fn skip(&self, first: &First, text: &str, mut pos: usize) -> usize {
        let bytes = text.as_bytes();
        while let Some(&b) = bytes.get(pos) {
            if b.is_ascii() {
                if first.ascii & (1 << b) != 0 {
                    return pos;
                }
                pos += 1;
            } else {
                let Some(c) = text[pos..].chars().next() else { break };
                if first.steps.iter().any(|&pc| self.reads(pc, c)) {
                    return pos;
                }
                pos += c.len_utf8();
            }
        }
        pos
    }

    /// The number of groups, the whole match (group 0) included.
    pub fn groups(&self) -> usize {
        self.names.len()
    }

    /// The names of the groups, by number.
    pub fn names(&self) -> &[Option<Rc<str>>] {
        &self.names
    }

    /// Does the regex match the whole text? Then where its groups are.
    pub fn whole(&self, text: &str) -> Option<Captures> {
        self.with_vm(|vm| self.search(vm, text, 0, Mode::Whole))
    }

    /// The leftmost match starting at byte `start` or after.
    pub fn find_at(&self, text: &str, start: usize) -> Option<Captures> {
        self.with_vm(|vm| self.search(vm, text, start, Mode::Search))
    }

    /// One search, with the faster engine when its table is small enough.
    fn search(&self, vm: &mut Vm, text: &str, start: usize, mode: Mode) -> Option<Captures> {
        if !self.trackable(text.len() - start) {
            return self.run(vm, text, start, mode, None).then(|| groups_of(&vm.best));
        }
        vm.track.begin(self.prog.len(), start, text.len() - start + 1);
        let found = self.track_search(vm, text, start, mode);
        vm.track.end();
        found.then(|| groups_of(&vm.scratch))
    }

    /// Can [`Regex::track`] follow a text of `len` bytes (its table of
    /// steps and places is not too large)?
    fn trackable(&self, len: usize) -> bool {
        self.prog.len().checked_mul(len + 1).is_some_and(|n| n <= MAX_DEAD_BITS)
    }

    /// The leftmost match from `start` (or the match of the whole text), by
    /// trying one way after the other in order of preference (see [`Track`]).
    /// True when found, with the slots of the match in `vm.scratch`.
    fn track_search(&self, vm: &mut Vm, text: &str, start: usize, mode: Mode) -> bool {
        let mut at = start;
        loop {
            if mode == Mode::Search {
                if let Some(first) = &self.first {
                    at = self.skip(first, text, at);
                    if at == text.len() {
                        return false;
                    }
                }
            }
            if self.track(vm, text, at, mode) {
                return true;
            }
            if mode == Mode::Whole {
                return false;
            }
            match text[at..].chars().next() {
                Some(c) => at += c.len_utf8(),
                None => return false,
            }
        }
    }

    /// Follows the ways of a match starting at `at`, the preferred one first,
    /// each until it fails; true when one reaches the end of the regex, with
    /// its groups in `vm.scratch`.
    fn track(&self, vm: &mut Vm, text: &str, at: usize, mode: Mode) -> bool {
        let Vm { scratch, track, .. } = vm;
        let bytes = text.as_bytes();
        scratch.fill(NONE);
        track.jobs.clear();
        track.jobs.push(Job::Explore(0, at));
        track.hi = track.hi.max(at);
        while let Some(job) = track.jobs.pop() {
            let (mut pc, mut pos) = match job {
                Job::Restore(slot, value) => {
                    scratch[slot] = value;
                    continue;
                }
                Job::Explore(pc, pos) => (pc, pos),
            };
            while !track.seen(pc, pos) {
                match self.prog[pc] {
                    step @ (Inst::Char(_) | Inst::Any | Inst::Class(_)) => {
                        let c = match bytes.get(pos) {
                            Some(&b) if b.is_ascii() => char::from(b),
                            Some(_) => text[pos..].chars().next().unwrap_or_default(),
                            None => break,
                        };
                        let reads = match step {
                            Inst::Char(x) => fold_if(c, self.fold) == x,
                            Inst::Any => c != '\n',
                            Inst::Class(k) => self.classes[k].matches(c, self.fold),
                            _ => false,
                        };
                        if !reads {
                            break;
                        }
                        pc += 1;
                        pos += c.len_utf8();
                        track.hi = track.hi.max(pos);
                    }
                    Inst::Split(a, b) => {
                        track.jobs.push(Job::Explore(b, pos));
                        pc = a;
                    }
                    Inst::Jmp(to) => pc = to,
                    Inst::Save(slot) => {
                        track.jobs.push(Job::Restore(slot, scratch[slot]));
                        scratch[slot] = pos;
                        pc += 1;
                    }
                    Inst::Assert(a) => {
                        if !self.holds(a, text, pos) {
                            break;
                        }
                        pc += 1;
                    }
                    Inst::Match => {
                        if mode == Mode::Whole && pos != text.len() {
                            break;
                        }
                        track.jobs.clear();
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Calls `f` with the memory of a search: the one kept from the last
    /// search, or a new one. Taken out while `f` runs, so a search inside
    /// it would only make its own.
    fn with_vm<T>(&self, f: impl FnOnce(&mut Vm) -> T) -> T {
        let mut vm = self.spare.borrow_mut().take().unwrap_or_else(|| {
            let (size, n) = (self.prog.len(), 2 * self.names.len());
            Vm {
                clist: Threads::new(size, n),
                nlist: Threads::new(size, n),
                stack: Vec::new(),
                scratch: vec![NONE; n],
                best: vec![NONE; n],
                track: Track::default(),
            }
        });
        let result = f(&mut vm);
        *self.spare.borrow_mut() = Some(vm);
        result
    }

    /// Every match, from left to right, without overlapping. An empty match
    /// right where the previous match ended is skipped, as in Rust: `a*` finds
    /// "" and "aaa" in "baaa". The same matches as calling [`Regex::find_at`]
    /// again and again, but the searches share what they learned (see [`Dead`]).
    pub fn find_all(&self, text: &str, max: usize) -> Result<Vec<Captures>, usize> {
        let slots = self.find_all_slots(text, max)?;
        Ok(slots.chunks(2 * self.names.len()).map(groups_of).collect())
    }

    /// The matches of [`Regex::find_all`], each as the start and the end of
    /// every group one after the other (`usize::MAX` for a group that took no
    /// part in it): one list for all, faster to make than a list for each.
    pub fn find_all_slots(&self, text: &str, max: usize) -> Result<Vec<usize>, usize> {
        self.with_vm(|vm| self.find_all_with(vm, text, max))
    }

    fn find_all_with(&self, vm: &mut Vm, text: &str, max: usize) -> Result<Vec<usize>, usize> {
        let tracked = self.trackable(text.len());
        if tracked {
            vm.track.begin(self.prog.len(), 0, text.len() + 1);
        }
        let found = self.find_all_in(vm, text, max, tracked);
        if tracked {
            vm.track.end();
        }
        found
    }

    fn find_all_in(&self, vm: &mut Vm, text: &str, max: usize, tracked: bool) -> Result<Vec<usize>, usize> {
        let mut out = Vec::new();
        let mut count = 0;
        let mut pos = 0;
        let mut last_end = None;
        let mut dead = if tracked { None } else { Dead::new(self.prog.len(), text) };
        while pos <= text.len() {
            let found =
                if tracked { self.track_search(vm, text, pos, Mode::Search) } else { self.run(vm, text, pos, Mode::Search, dead.as_mut()) };
            if !found {
                break;
            }
            let m = if tracked { &vm.scratch } else { &vm.best };
            // Group 0 always takes part in a match.
            let (s, e) = (m[0], m[1]);
            if tracked {
                // The ways that failed stay failed for the next searches, which
                // start at `e` or after; the way that matched ends at `e`.
                vm.track.forget(e);
            }
            if s == e && last_end == Some(e) {
                match text[pos..].chars().next() {
                    Some(c) => pos += c.len_utf8(),
                    None => break,
                }
                continue;
            }
            if count >= max {
                return Err(count);
            }
            out.extend_from_slice(if tracked { &vm.scratch } else { &vm.best });
            count += 1;
            pos = e;
            last_end = Some(e);
        }
        Ok(out)
    }

    fn holds(&self, a: Assert, text: &str, pos: usize) -> bool {
        match a {
            Assert::Start => pos == 0,
            Assert::End => pos == text.len(),
            Assert::Boundary | Assert::NotBoundary => {
                let before = text[..pos].chars().next_back().is_some_and(is_word_char);
                let after = text[pos..].chars().next().is_some_and(is_word_char);
                (before != after) == (a == Assert::Boundary)
            }
        }
    }

    /// Adds the way that reaches step `pc` at `pos`, and every way that
    /// follows from it without reading a character, in order of preference.
    #[allow(clippy::too_many_arguments)]
    fn add(
        &self,
        list: &mut Threads,
        stack: &mut Vec<Frame>,
        scratch: &mut [usize],
        pc: usize,
        text: &str,
        pos: usize,
        dead: Option<&Dead>,
    ) {
        let n = scratch.len();
        stack.push(Frame::Explore(pc));
        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Restore(slot, value) => scratch[slot] = value,
                Frame::Explore(mut pc) => loop {
                    if list.contains(pc) || dead.is_some_and(|d| d.has(pc, pos)) {
                        break;
                    }
                    list.insert(pc);
                    match self.prog[pc] {
                        Inst::Jmp(to) => pc = to,
                        Inst::Split(a, b) => {
                            stack.push(Frame::Explore(b));
                            pc = a;
                        }
                        Inst::Save(slot) => {
                            stack.push(Frame::Restore(slot, scratch[slot]));
                            scratch[slot] = pos;
                            pc += 1;
                        }
                        Inst::Assert(a) => {
                            if !self.holds(a, text, pos) {
                                break;
                            }
                            pc += 1;
                        }
                        Inst::Char(_) | Inst::Any | Inst::Class(_) | Inst::Match => {
                            list.slots[pc * n..(pc + 1) * n].copy_from_slice(scratch);
                            break;
                        }
                    }
                },
            }
        }
    }

    /// True when found, with the slots of the match in `vm.best`.
    fn run(&self, vm: &mut Vm, text: &str, start: usize, mode: Mode, mut dead: Option<&mut Dead>) -> bool {
        let n = 2 * self.names.len();
        let Vm { clist, nlist, stack, scratch, best, .. } = vm;
        clist.len = 0;
        nlist.len = 0;
        let mut matched = false;
        let mut pos = start;
        loop {
            if !matched && (pos == start || mode == Mode::Search) {
                if clist.len == 0 && mode == Mode::Search {
                    if let Some(first) = &self.first {
                        // No way is being followed: go straight to the next
                        // place where one can start. None at the end, where a
                        // match would be empty.
                        pos = self.skip(first, text, pos);
                        if pos == text.len() {
                            break;
                        }
                    }
                }
                scratch.fill(NONE);
                self.add(clist, stack, scratch, 0, text, pos, dead.as_deref());
            }
            if clist.len == 0 {
                break;
            }
            if let Some(d) = dead.as_deref_mut() {
                let room = MAX_PENDING.saturating_sub(d.pending.len());
                d.pending.extend(clist.dense[..clist.len].iter().take(room).map(|&pc| (pos, pc)));
            }
            let c = text[pos..].chars().next();
            let next = pos + c.map_or(0, char::len_utf8);
            for i in 0..clist.len {
                let pc = clist.dense[i];
                let reads = match self.prog[pc] {
                    Inst::Match => {
                        if mode == Mode::Whole && pos != text.len() {
                            continue;
                        }
                        // The ways after this one are less preferred: they stop here.
                        best.copy_from_slice(&clist.slots[pc * n..(pc + 1) * n]);
                        matched = true;
                        // The ways followed so far are not known to be dead.
                        if let Some(d) = dead.as_deref_mut() {
                            d.pending.clear();
                        }
                        break;
                    }
                    _ => c.is_some_and(|c| self.reads(pc, c)),
                };
                if reads {
                    scratch.copy_from_slice(&clist.slots[pc * n..(pc + 1) * n]);
                    self.add(nlist, stack, scratch, pc + 1, text, next, dead.as_deref());
                }
            }
            std::mem::swap(clist, nlist);
            nlist.len = 0;
            if c.is_none() {
                break;
            }
            pos = next;
        }
        if let Some(d) = dead {
            d.write_down();
        }
        matched
    }
}

/// The groups of a match, from its slots.
fn groups_of(slots: &[usize]) -> Captures {
    slots.chunks(2).map(|g| if g[0] == NONE || g[1] == NONE { None } else { Some((g[0], g[1])) }).collect()
}

// ----- the replacement of `replace_all` ------------------------------------

#[derive(Debug, PartialEq)]
pub enum Piece {
    Text(String),
    Group(usize),
}

/// Reads a replacement: `$1` is the text of group 1, `$<name>` of a named
/// group, `$0` the whole match, and `$$` a `$`.
pub fn replacement(new: &str, re: &Regex) -> Result<Vec<Piece>, RegexError> {
    let chars: Vec<char> = new.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    let groups = re.groups() - 1;
    let dollar_help = || {
        t!(
            "`$1` is the text of group 1, `$<name>` of a named group, `$0` the whole match; for the sign `$` itself, write `$$`",
            "`$1` est le texte du groupe 1, `$<nom>` celui d'un groupe nommé, `$0` tout le morceau trouvé ; pour le signe `$` lui-même, écrivez `$$`"
        )
    };
    while i < chars.len() {
        let c = chars[i];
        if c != '$' {
            cur.push(c);
            i += 1;
            continue;
        }
        let at = i;
        i += 1;
        let group = match chars.get(i) {
            Some('$') => {
                cur.push('$');
                i += 1;
                continue;
            }
            Some(d) if d.is_ascii_digit() => {
                let mut n: usize = 0;
                while let Some(d) = chars.get(i).and_then(|c| c.to_digit(10)) {
                    n = n.saturating_mul(10).saturating_add(d as usize);
                    i += 1;
                }
                if n > groups {
                    let has = match groups {
                        0 => t!("the regex has no group", "la regex n'a pas de groupe"),
                        1 => t!("the regex has 1 group", "la regex a 1 groupe"),
                        g => t!("the regex has {g} groups", "la regex a {g} groupes"),
                    };
                    return Err(error(
                        at,
                        i - at,
                        t!("there is no group {n}: {has}", "il n'y a pas de groupe {n} : {has}"),
                        t!("no such group", "pas de tel groupe"),
                        Some(dollar_help()),
                    ));
                }
                n
            }
            Some('<') => {
                let name_at = i + 1;
                let end = chars[name_at..].iter().position(|&c| c == '>').map(|p| name_at + p);
                let Some(end) = end else {
                    return Err(error(
                        at,
                        2,
                        t!("`$<` must be followed by a group name and `>`", "`$<` doit être suivi d'un nom de groupe et de `>`"),
                        t!("never closed", "jamais fermé"),
                        Some(dollar_help()),
                    ));
                };
                let name: String = chars[name_at..end].iter().collect();
                i = end + 1;
                match re.names.iter().position(|n| n.as_deref() == Some(name.as_str())) {
                    Some(g) => g,
                    None => {
                        let known: Vec<String> = re.names.iter().flatten().map(|n| format!("`{n}`")).collect();
                        let has = if known.is_empty() {
                            t!("the regex has no named group", "la regex n'a pas de groupe nommé")
                        } else {
                            t!("its named groups are {}", "ses groupes nommés sont {}", known.join(", "))
                        };
                        return Err(error(
                            at,
                            i - at,
                            t!("the regex has no group named `{name}`: {has}", "la regex n'a pas de groupe nommé `{name}` : {has}"),
                            t!("unknown group", "groupe inconnu"),
                            Some(dollar_help()),
                        ));
                    }
                }
            }
            _ => {
                return Err(error(
                    at,
                    1,
                    t!(
                        "`$` must be followed by a group number, `<name>` or `$`",
                        "`$` doit être suivi d'un numéro de groupe, de `<nom>` ou de `$`"
                    ),
                    t!("a lone `$`", "un `$` seul"),
                    Some(dollar_help()),
                ))
            }
        };
        if !cur.is_empty() {
            out.push(Piece::Text(std::mem::take(&mut cur)));
        }
        out.push(Piece::Group(group));
    }
    if !cur.is_empty() {
        out.push(Piece::Text(cur));
    }
    Ok(out)
}

// ----- messages ----------------------------------------------------------------

/// The error for a mistake in the regex (or the replacement, `in_replacement`)
/// `source`. When `exact`, `span` is the mistake itself, in the code;
/// otherwise it is the call, and a note shows the mistake in the regex.
pub fn regex_error(e: &RegexError, source: &str, in_replacement: bool, span: Span, exact: bool) -> Box<Diagnostic> {
    let message = if in_replacement {
        t!("invalid replacement: {}", "remplacement invalide : {}", e.message)
    } else {
        t!("invalid regex: {}", "regex invalide : {}", e.message)
    };
    let mut d = Diagnostic::error("E026", message).at(span);
    if exact {
        d = d.label(e.label.clone());
    } else {
        let label =
            if in_replacement { t!("with this replacement", "avec ce remplacement") } else { t!("with this regex", "avec cette regex") };
        let here = if in_replacement {
            t!("here in the replacement:", "ici dans le remplacement :")
        } else {
            t!("here in the regex:", "ici dans la regex :")
        };
        d = d.label(label).note(format!("{here}\n{}", pointed(source, e.at, e.len)));
    }
    if let Some(h) = &e.help {
        d = d.help(h.clone());
    }
    d.boxed()
}

/// The text with `^^^` under the characters from `at`, `len` of them: line
/// breaks and tabs are shown as `\n` and `\t`, a long text is cut.
fn pointed(source: &str, at: usize, len: usize) -> String {
    const AROUND: usize = 30;
    let chars: Vec<char> = source.chars().collect();
    let from = at.saturating_sub(AROUND);
    let to = (at + len.max(1) + AROUND).min(chars.len());
    let mut line = String::from("  ");
    let mut marks = String::from("  ");
    if from > 0 {
        line.push_str("...");
        marks.push_str("   ");
    }
    for (i, &c) in chars.iter().enumerate().take(to).skip(from) {
        let shown = match c {
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            '\r' => "\\r".to_string(),
            c if c.is_control() => " ".to_string(),
            c => c.to_string(),
        };
        let w = shown.chars().count();
        let mark = if i >= at && i < at + len.max(1) { '^' } else { ' ' };
        line.push_str(&shown);
        marks.extend(std::iter::repeat_n(mark, w));
    }
    if to < chars.len() {
        line.push_str("...");
    }
    if at >= chars.len() {
        marks.push('^');
    }
    format!("{line}\n{}", marks.trim_end())
}

/// The span, in the code, of the characters `at..at + len` of the text
/// written at `span` (a text without `{...}`), when it can be found.
pub fn span_in_text(span: Span, at: usize, len: usize) -> Option<Span> {
    let file = source::get(span.file)?;
    let code = file.text.get(span.start as usize..span.end as usize)?;
    let inner = code.strip_prefix('"')?.strip_suffix('"')?;
    // Where each character of the value is written: escapes take several.
    let mut places: Vec<(usize, usize)> = Vec::new();
    let mut it = inner.char_indices().peekable();
    while let Some((i, c)) = it.next() {
        let mut end = i + c.len_utf8();
        if c == '\\' {
            let (j, e) = it.next()?;
            end = j + e.len_utf8();
            if e == 'u' {
                for (j, h) in it.by_ref() {
                    end = j + h.len_utf8();
                    if h == '}' {
                        break;
                    }
                }
            }
        }
        places.push((i, end));
    }
    let base = span.start as usize + 1;
    let (start, end) = if at >= places.len() {
        // After the last character: the closing quote.
        let e = inner.len();
        (e, e + 1)
    } else {
        let last = (at + len.max(1) - 1).min(places.len() - 1);
        (places[at].0, places[last].1)
    };
    Some(Span::new(span.file, base + start, base + end))
}

// ----- the built-in functions ----------------------------------------------------

thread_local! {
    /// The last regexes used: `filter(lines, fn(l) = matches(l, "..."))` reads the regex once.
    static CACHE: RefCell<Vec<(Rc<str>, Rc<Regex>)>> = const { RefCell::new(Vec::new()) };
}

const CACHED: usize = 16;

/// Reads the regex given as argument `i` of `f`.
fn regex_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Rc<Regex>> {
    let Value::Text(pattern) = &a[i] else { return Err(wrong(f, i, Expected::Text, &a[i], span)) };
    let found = CACHE.with(|c| c.borrow().iter().find(|(p, _)| p == pattern).map(|(_, r)| r.clone()));
    if let Some(r) = found {
        return Ok(r);
    }
    let re = Rc::new(Regex::new(pattern).map_err(|e| regex_error(&e, pattern, false, span, false))?);
    CACHE.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= CACHED {
            c.remove(0);
        }
        c.push((Rc::from(&**pattern), re.clone()));
    });
    Ok(re)
}

/// The text and the regex of a call `f(text, regex, ...)`.
fn text_and_regex<'a>(a: &'a [Value], f: &str, span: Span) -> Fallible<(&'a str, Rc<Regex>)> {
    let s = text(a, 0, f, span)?;
    text(a, 1, f, span)?;
    Ok((s, regex_arg(a, 1, f, span)?))
}

fn group_text(s: &str, g: Option<(usize, usize)>) -> Value {
    text_value(g.map_or("", |(a, b)| &s[a..b]))
}

/// Every match: its groups, as slots (see [`Regex::find_all_slots`]).
fn all(s: &str, re: &Regex, span: Span) -> Fallible<Vec<usize>> {
    re.find_all_slots(s, MAX_LIST).map_err(|_| too_big(true, span))
}

/// The text of group `g` of a match given by its slots.
fn slot_text<'a>(s: &'a str, m: &[usize], g: usize) -> &'a str {
    match (m[2 * g], m[2 * g + 1]) {
        (NONE, _) | (_, NONE) => "",
        (a, b) => &s[a..b],
    }
}

fn whole_text(m: &Captures, s: &str) -> Value {
    group_text(s, m[0])
}

pub(super) fn matches(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "matches", span)?;
    Ok(Value::Bool(re.whole(s).is_some()))
}

pub(super) fn find_first(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "find_first", span)?;
    Ok(it.option(re.find_at(s, 0).map(|m| whole_text(&m, s))))
}

pub(super) fn find_all(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "find_all", span)?;
    Ok(Value::list(all(s, &re, span)?.chunks(2 * re.groups()).map(|m| text_value(slot_text(s, m, 0))).collect()))
}

fn groups_list(m: &Captures, s: &str) -> Value {
    Value::list(m.iter().map(|g| group_text(s, *g)).collect())
}

pub(super) fn captures(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "captures", span)?;
    Ok(it.option(re.find_at(s, 0).map(|m| groups_list(&m, s))))
}

pub(super) fn captures_all(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "captures_all", span)?;
    let groups = re.groups();
    Ok(Value::list(
        all(s, &re, span)?.chunks(2 * groups).map(|m| Value::list((0..groups).map(|g| text_value(slot_text(s, m, g))).collect())).collect(),
    ))
}

pub(super) fn named_captures(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "named_captures", span)?;
    if re.names().iter().all(Option::is_none) {
        return Err(Diagnostic::error(
            "E023",
            t!("`named_captures` needs a regex with named groups", "`named_captures` a besoin d'une regex avec des groupes nommés"),
        )
        .at(span)
        .help(t!(
            "name a group like this: `(?<year>[0-9]+)`; for groups without names, use `captures`",
            "nommez un groupe comme ceci : `(?<annee>[0-9]+)` ; pour des groupes sans nom, utilisez `captures`"
        ))
        .boxed());
    }
    Ok(it.option(re.find_at(s, 0).map(|m| {
        let mut r = Record::new();
        for (name, g) in re.names().iter().zip(&m) {
            if let Some(name) = name {
                r.set(name.clone(), group_text(s, *g));
            }
        }
        Value::Record(Rc::new(r))
    })))
}

pub(super) fn split_by(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (s, re) = text_and_regex(a, "split_by", span)?;
    let mut parts = Vec::new();
    let mut from = 0;
    // An empty match never cuts: `split_by(text, " *")` cuts at the spaces only.
    for m in all(s, &re, span)?.chunks(2 * re.groups()) {
        let (start, end) = (m[0], m[1]);
        if start != end {
            parts.push(text_value(&s[from..start]));
            from = end;
        }
    }
    parts.push(text_value(&s[from..]));
    Ok(Value::list(parts))
}

pub(super) fn replace_all(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "replace_all", span)?;
    text(a, 1, "replace_all", span)?;
    let new = match &a[2] {
        Value::Text(t) => Some(t.clone()),
        v if v.is_callable() => None,
        v => return Err(wrong("replace_all", 2, Expected::TextOrFunction, v, span)),
    };
    let re = regex_arg(a, 1, "replace_all", span)?;
    let pieces = match &new {
        Some(t) => Some(replacement(t, &re).map_err(|e| regex_error(&e, t, true, span, false))?),
        None => None,
    };
    let mut out = String::new();
    let mut from = 0;
    for m in all(s, &re, span)?.chunks(2 * re.groups()) {
        let (start, end) = (m[0], m[1]);
        out.push_str(&s[from..start]);
        match &pieces {
            Some(pieces) => {
                for p in pieces {
                    match p {
                        Piece::Text(t) => out.push_str(t),
                        Piece::Group(g) => out.push_str(slot_text(s, m, *g)),
                    }
                }
            }
            None => match call_given1(it, "replace_all", &a[2], text_value(&s[start..end]), span)? {
                Value::Text(t) => out.push_str(&t),
                other => return Err(wrong_return("replace_all", TEXT, &other.describe(), span)),
            },
        }
        if out.len() > MAX_TEXT {
            return Err(too_big(false, span));
        }
        from = end;
    }
    out.push_str(&s[from..]);
    Ok(text_value(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small generator of random numbers (xorshift), for the same cases each time.
    struct Rng(u64);

    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }

        fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
            items[self.below(items.len())]
        }
    }

    fn pattern(rng: &mut Rng, depth: usize) -> String {
        const ATOMS: &[&str] =
            &["a", "b", "é", "σ", "Σ", " ", ".", "[ab]", "[^a]", "[a-zé]", "[σς]", "\\w", "\\W", "\\d", "\\s", "\\b", "\\B", "^", "$"];
        const REPEATS: &[&str] = &["", "", "", "*", "+", "?", "*?", "+?", "??", "{1,2}", "{2}", "{0,}"];
        let mut out = String::new();
        for _ in 0..1 + rng.below(3) {
            let atom = match rng.below(6) {
                0 if depth > 0 => format!("({})", pattern(rng, depth - 1)),
                1 if depth > 0 => format!("(?:{}|{})", pattern(rng, depth - 1), pattern(rng, depth - 1)),
                _ => rng.pick(ATOMS).to_string(),
            };
            let repeat = if matches!(atom.as_str(), "^" | "$" | "\\b" | "\\B") { "" } else { rng.pick(REPEATS) };
            out += &atom;
            out += repeat;
        }
        if rng.below(4) == 0 {
            out += "|";
            out += &pattern(rng, depth.saturating_sub(1));
        }
        out
    }

    /// The engine that tries one way after the other gives exactly the answers
    /// of the one that follows every way at the same time (which it replaces
    /// when its table would be too large), groups included, with Unicode
    /// texts and ignoring the case.
    #[test]
    fn both_engines_give_the_same_answers() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        const CHARS: &[&str] = &["a", "A", "b", "é", "É", " ", "\n", "σ", "Σ", "ς", "1", "_"];
        for case in 0..20_000 {
            let mut p = pattern(&mut rng, 3);
            if case % 5 == 0 {
                p = format!("(?i){p}");
            }
            let Ok(re) = Regex::new(&p) else { continue };
            for _ in 0..4 {
                let text: String = (0..rng.below(12)).map(|_| rng.pick(CHARS)).collect();
                let slow_whole = re.with_vm(|vm| re.run(vm, &text, 0, Mode::Whole, None).then(|| groups_of(&vm.best)));
                assert_eq!(re.whole(&text), slow_whole, "whole {text:?} {p:?}");
                for (start, _) in text.char_indices().chain([(text.len(), ' ')]) {
                    let slow = re.with_vm(|vm| re.run(vm, &text, start, Mode::Search, None).then(|| groups_of(&vm.best)));
                    assert_eq!(re.find_at(&text, start), slow, "find_at {text:?} {start} {p:?}");
                }
                let slow_all = re.with_vm(|vm| re.find_all_in(vm, &text, usize::MAX, false));
                assert_eq!(re.find_all_slots(&text, usize::MAX), slow_all, "find_all {text:?} {p:?}");
            }
        }
    }
}
