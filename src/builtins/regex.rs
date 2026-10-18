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
}

impl Class {
    fn matches(&self, c: char, fold: bool) -> bool {
        let has = |c: char| self.items.iter().any(|i| i.has(c));
        let found = has(c) || (fold && (has(lower(c)) || has(upper(c)) || has(fold_case(c))));
        found != self.negated
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

/// The same character for every case: `A`, `a` → `a`; `Σ`, `σ`, `ς` → `σ`.
fn fold_case(c: char) -> char {
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
                    self.classes.push(Class { negated: false, items: vec![i] });
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
        self.classes.push(Class { negated, items });
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

enum Frame {
    Explore(usize),
    Restore(usize, usize),
}

/// The lists and the scratch space of a search, kept from one search to the
/// next by `find_all`.
struct Vm {
    clist: Threads,
    nlist: Threads,
    stack: Vec<Frame>,
    scratch: Vec<usize>,
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
        Ok(Regex { prog, classes: p.classes, fold, names: p.names })
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
        self.run(&mut self.vm(), text, 0, Mode::Whole, None)
    }

    /// The leftmost match starting at byte `start` or after.
    pub fn find_at(&self, text: &str, start: usize) -> Option<Captures> {
        self.run(&mut self.vm(), text, start, Mode::Search, None)
    }

    fn vm(&self) -> Vm {
        let (size, n) = (self.prog.len(), 2 * self.names.len());
        Vm { clist: Threads::new(size, n), nlist: Threads::new(size, n), stack: Vec::new(), scratch: vec![NONE; n] }
    }

    /// Every match, from left to right, without overlapping. An empty match
    /// right where the previous match ended is skipped, as in Rust: `a*` finds
    /// "" and "aaa" in "baaa". The same matches as calling [`Regex::find_at`]
    /// again and again, but the searches share what they learned (see [`Dead`]).
    pub fn find_all(&self, text: &str, max: usize) -> Result<Vec<Captures>, usize> {
        let mut out = Vec::new();
        let mut pos = 0;
        let mut last_end = None;
        let mut vm = self.vm();
        let mut dead = Dead::new(self.prog.len(), text);
        while pos <= text.len() {
            let Some(m) = self.run(&mut vm, text, pos, Mode::Search, dead.as_mut()) else { break };
            let (s, e) = m[0].unwrap_or((pos, pos));
            if s == e && last_end == Some(e) {
                match text[pos..].chars().next() {
                    Some(c) => pos += c.len_utf8(),
                    None => break,
                }
                continue;
            }
            if out.len() >= max {
                return Err(out.len());
            }
            out.push(m);
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

    fn run(&self, vm: &mut Vm, text: &str, start: usize, mode: Mode, mut dead: Option<&mut Dead>) -> Option<Captures> {
        let n = 2 * self.names.len();
        let Vm { clist, nlist, stack, scratch } = vm;
        clist.len = 0;
        nlist.len = 0;
        let mut matched: Option<Vec<usize>> = None;
        let mut pos = start;
        loop {
            if matched.is_none() && (pos == start || mode == Mode::Search) {
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
                    Inst::Char(x) => c.is_some_and(|c| (if self.fold { fold_case(c) } else { c }) == x),
                    Inst::Any => c.is_some_and(|c| c != '\n'),
                    Inst::Class(k) => c.is_some_and(|c| self.classes[k].matches(c, self.fold)),
                    Inst::Match => {
                        if mode == Mode::Whole && pos != text.len() {
                            continue;
                        }
                        // The ways after this one are less preferred: they stop here.
                        matched = Some(clist.slots[pc * n..(pc + 1) * n].to_vec());
                        // The ways followed so far are not known to be dead.
                        if let Some(d) = dead.as_deref_mut() {
                            d.pending.clear();
                        }
                        break;
                    }
                    _ => false,
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
        matched.map(|slots| slots.chunks(2).map(|g| if g[0] == NONE || g[1] == NONE { None } else { Some((g[0], g[1])) }).collect())
    }
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

fn all(s: &str, re: &Regex, span: Span) -> Fallible<Vec<Captures>> {
    re.find_all(s, MAX_LIST).map_err(|_| too_big(true, span))
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
    Ok(Value::list(all(s, &re, span)?.iter().map(|m| whole_text(m, s)).collect()))
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
    Ok(Value::list(all(s, &re, span)?.iter().map(|m| groups_list(m, s)).collect()))
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
    for m in all(s, &re, span)? {
        if let Some((start, end)) = m[0].filter(|(start, end)| start != end) {
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
    for m in all(s, &re, span)? {
        let Some((start, end)) = m[0] else { continue };
        out.push_str(&s[from..start]);
        match &pieces {
            Some(pieces) => {
                for p in pieces {
                    match p {
                        Piece::Text(t) => out.push_str(t),
                        Piece::Group(g) => out.push_str(m[*g].map_or("", |(a, b)| &s[a..b])),
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
