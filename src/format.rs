//! The official formatter: `rythme fmt` gives every program the same layout.
//!
//! The style, as in `examples/`:
//! - two spaces of indentation per level, never tabs;
//! - one space around operators (`a + b`, `x |> f`, `n -> n * 2`, `x = 1`),
//!   after commas and after `:` in records; none inside brackets (`f(x)`,
//!   `[1, 2]`, `{name: "Ada"}`), around `..` (`1..10`, `[first, ..rest]`) or
//!   after a minus sign (`-x`);
//! - the inside of a block, a `match` or a bracket one level deeper than the
//!   line that opens it, and `end` or the closing bracket back at that level;
//!   a line that continues an expression one level deeper than its first line;
//! - a `|>` pipeline written on several lines gets one step per line, `|>`
//!   first; a type written on several lines gets one variant per line, each
//!   after `|`;
//! - a list written on several lines ends with a comma; on one line, without;
//! - a block given to a call (`do |x| ... end`, `{ |x| ... }`) written on one
//!   line stays on one line, with one space around its parts
//!   (`xs.map do |x| x * 2 end`, `xs.map { |x| x * 2 }`); on several lines,
//!   its statements are one level deeper than the line of `do`;
//! - a call without parentheses keeps them out, with one space after its
//!   name (`puts "Hello"`, `print x, y`); a name followed by `[` or `-` that
//!   is not such a call is written as what it is (`xs[0]`, `x - 1`);
//! - statements separated by `;` go on their own lines, and `end` too;
//! - a top-level definition written on several lines is separated from what
//!   surrounds it by one blank line; one-line definitions may stay grouped;
//!   at most one blank line in a row, none at the start or the end of a block;
//! - every comment is kept as written; comments at the end of consecutive
//!   lines are aligned;
//! - no spaces at the end of lines, one new line at the end of the file.
//!
//! Elsewhere, the line breaks of the author are kept, and long lines too.
//!
//! The formatter never changes what a program means. The code is read by the
//! real lexer and parser first: a file with a syntax error is refused, with
//! the usual messages. A light parser then walks the same tokens to learn the
//! role of each one (a minus sign or a subtraction, a call or a group...) and
//! where each line goes; comments come from the text between tokens. Finally,
//! the result is read again: it must give the same syntax tree, with the same
//! comments, or the formatter refuses rather than change the program.

use std::path::{Path, PathBuf};

use crate::diagnostic::{self, Diagnostic};
use crate::lexer::{self, Tok, Token};
use crate::parser::{self, is_continuation};
use crate::symbol::Sym;
use crate::{io, source, suggest};

/// Spaces per indentation level.
const WIDTH: usize = 2;

/// The file number of code that is read but never shown in a message.
const UNSHOWN: u32 = u32::MAX;

/// Formats Rythme code. See [`format_named`].
pub fn format(source: &str) -> Result<String, Vec<Diagnostic>> {
    format_named("<input>", source)
}

/// Formats Rythme code in the canonical style. `name` (a file name) is used in
/// the messages. A program with a syntax error gives its diagnostics, ready to
/// be shown with [`Diagnostic::render`]; the code itself is never changed in a
/// way that changes its meaning. The line ends of the code (`\n` or `\r\n`)
/// are kept.
///
/// Like the parser and the checker, it walks the syntax tree recursively:
/// code nested thousands of levels deep needs a thread with a big stack, as
/// `rythme` has.
pub fn format_named(name: &str, source: &str) -> Result<String, Vec<Diagnostic>> {
    let Some(before) = syntax_tree(source) else { return Err(vec![syntax_error(name, source)]) };
    let out = layout(&short_uses(source)).filter(|out| syntax_tree(out).as_ref() == Some(&before) && comments(out) == comments(source));
    match out {
        Some(out) if uses_crlf(source) => Ok(out.replace('\n', "\r\n")),
        Some(out) => Ok(out),
        None => Err(vec![bug(name)]),
    }
}

/// `use "geometry.ry" as geo` written in the recommended form, `use
/// geometry as geo`, and `use geometry as geometry` as `use geometry`, when
/// the path can be written without quotes. `use "x.ry"` (whose definitions
/// are used without a name) stays as it is.
fn short_uses(src: &str) -> String {
    let Ok(items) = lexer::lex(src, UNSHOWN).and_then(parser::parse) else { return src.to_string() };
    let mut out = src.to_string();
    for item in items.iter().rev() {
        let crate::ast::Item::Use(u) = item else { continue };
        let Some((name, at)) = u.name else { continue };
        let Some(short) = short_path(&u.path) else { continue };
        let explicit = at.start >= u.path_span.end;
        if u.short && !explicit {
            continue;
        }
        let stem = short.rsplit('/').next().unwrap_or_default();
        let new = if *name.name() == *stem { short.clone() } else { format!("{short} as {name}") };
        let end = if explicit { at.end } else { u.path_span.end } as usize;
        out.replace_range(u.path_span.start as usize..end, &new);
    }
    out
}

/// `shapes/circle` for `"shapes/circle.ry"`: the path written without
/// quotes, when each part is a name (or `..`).
fn short_path(path: &str) -> Option<String> {
    let stem = path.strip_suffix(".ry")?;
    let parts: Vec<&str> = stem.split('/').collect();
    let name = |p: &str| {
        p.chars().next().is_some_and(|c| c.is_lowercase() || c == '_')
            && lexer::lex(p, UNSHOWN).is_ok_and(|l| matches!(l.tokens.as_slice(), [Token { tok: Tok::Ident(_), .. }, ..] if l.tokens.iter().filter(|t| !matches!(t.tok, Tok::Newline | Tok::Eof)).count() == 1))
    };
    let (last, dirs) = parts.split_last()?;
    (name(last) && **last != *"_" && dirs.iter().all(|d| *d == ".." || name(d))).then(|| stem.to_string())
}

/// The syntax tree of a program, without the positions: two programs that mean
/// the same thing give the same text.
fn syntax_tree(src: &str) -> Option<String> {
    let items = lexer::lex(src, UNSHOWN).and_then(parser::parse).ok()?;
    let debug = format!("{items:?}");
    let mut out = String::with_capacity(debug.len());
    let mut rest = debug.as_str();
    while let Some(i) = rest.find("Span { ") {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let len = span_len(rest).unwrap_or(1);
        out.push_str(if len == 1 { &rest[..1] } else { "Span" });
        rest = &rest[len..];
    }
    out.push_str(rest);
    Some(out)
}

/// The length of the `Span { file: 0, start: 12, end: 15 }` that `s` starts with.
fn span_len(s: &str) -> Option<usize> {
    let mut rest = s;
    for part in ["Span { file: ", ", start: ", ", end: "] {
        rest = rest.strip_prefix(part)?;
        let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        if digits == 0 {
            return None;
        }
        rest = &rest[digits..];
    }
    rest = rest.strip_prefix(" }")?;
    Some(s.len() - rest.len())
}

/// The diagnostic of a program that does not parse, pointing in the file.
fn syntax_error(name: &str, src: &str) -> Diagnostic {
    let file = source::add(name, src);
    match lexer::lex(src, file).and_then(parser::parse) {
        Err(d) => *d,
        Ok(_) => bug(name),
    }
}

fn bug(name: &str) -> Diagnostic {
    Diagnostic::error(
        "",
        t!(
            "`{name}` could not be formatted safely, so it was left as it is",
            "`{name}` n'a pas pu être mis en forme sans risque, il est donc laissé tel quel"
        ),
    )
    .help(t!(
        "this is a bug in Rythme, not in your program: please report it, with the file, at https://github.com/maxlestage/Rythme/issues",
        "c'est un bug de Rythme, pas de votre programme : merci de le signaler, avec le fichier, sur https://github.com/maxlestage/Rythme/issues"
    ))
}

/// Does the code end its lines Windows-style, with `\r\n`?
fn uses_crlf(src: &str) -> bool {
    src.find('\n').is_some_and(|i| src[..i].ends_with('\r'))
}

/// Every comment of a program, in order.
fn comments(src: &str) -> Option<Vec<String>> {
    let (_, pieces, _) = read(src)?;
    let mut out = Vec::new();
    for p in &pieces {
        out.extend(p.gap.trailing.iter().cloned());
        out.extend(p.gap.lines.iter().filter_map(|t| match t {
            Trivia::Comment(c) => Some(c.clone()),
            Trivia::Blank => None,
        }));
    }
    Some(out)
}

// ----- tokens and what is between them ------------------------------------

/// A line between two tokens.
#[derive(Clone, Debug, PartialEq)]
enum Trivia {
    Blank,
    Comment(String),
}

/// What is written before a token: line breaks and comments.
#[derive(Clone, Debug, Default)]
struct Gap {
    /// A comment at the end of the line of the previous token.
    trailing: Option<String>,
    /// The lines between the line of the previous token and the line of this one.
    lines: Vec<Trivia>,
    /// The token starts a line (after a new line or a `;`).
    broken: bool,
}

impl Gap {
    /// Reads the text between two tokens (`after_token`: a token comes before).
    fn read(text: &str, after_token: bool) -> Gap {
        let mut gap = Gap::default();
        // Still on the line of the previous token?
        let mut first_line = after_token;
        let mut comment = None;
        let mut semicolon = false;
        let mut rest = text;
        while let Some(c) = rest.chars().next() {
            match c {
                '#' => {
                    let end = rest.find('\n').unwrap_or(rest.len());
                    comment = Some(rest[..end].trim_end().to_string());
                    rest = &rest[end..];
                    continue;
                }
                '\n' => {
                    gap.broken = true;
                    if first_line {
                        gap.trailing = comment.take();
                        first_line = false;
                    } else if let Some(c) = comment.take() {
                        gap.lines.push(Trivia::Comment(c));
                    } else if !semicolon {
                        gap.lines.push(Trivia::Blank);
                    }
                    semicolon = false;
                }
                ';' => {
                    gap.broken = true;
                    semicolon = true;
                }
                _ => {}
            }
            rest = &rest[c.len_utf8()..];
        }
        // A comment on the last line of the file, without a new line after it.
        if let Some(c) = comment {
            if first_line {
                gap.trailing = Some(c);
            } else {
                gap.lines.push(Trivia::Comment(c));
            }
        }
        gap
    }
}

/// A token, as written, with what comes before it. The last one is the end of the file.
struct Piece {
    tok: Tok,
    text: String,
    gap: Gap,
}

/// The tokens of a program (with the new lines, as the parser sees them), its
/// pieces, and the piece of each token (`usize::MAX` for new lines).
fn read(src: &str) -> Option<(Vec<Token>, Vec<Piece>, Vec<usize>)> {
    let tokens = lexer::lex(src, UNSHOWN).ok()?.tokens;
    let mut pieces: Vec<Piece> = Vec::new();
    let mut piece_of = vec![usize::MAX; tokens.len()];
    let mut end = if src.starts_with('\u{FEFF}') { '\u{FEFF}'.len_utf8() } else { 0 };
    for (i, t) in tokens.iter().enumerate() {
        if t.tok == Tok::Newline {
            continue;
        }
        let start = t.span.start as usize;
        let gap = Gap::read(src.get(end..start)?, !pieces.is_empty());
        // Inside a text, the lexer reads `\r\n` as `\n`.
        let text = src.get(start..t.span.end as usize)?.replace("\r\n", "\n");
        piece_of[i] = pieces.len();
        pieces.push(Piece { tok: t.tok.clone(), text, gap });
        end = t.span.end as usize;
    }
    Some((tokens, pieces, piece_of))
}

// ----- the shape of the program ---------------------------------------------

/// How a token sits next to its neighbours on a line.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Role {
    /// Names, values, keywords, operators: spaces around.
    Word,
    /// `(`, `[` or `{` of a group, a list or a record.
    Open,
    /// `(` of a call, `[` of an index: right after what comes before.
    Call,
    Close,
    Comma,
    Dot,
    Question,
    /// `x ?`: a `?` after a name that must stay apart from it, or it would be
    /// read as a part of the name (`even?`).
    SpacedQuestion,
    /// `-x`: a minus sign before a value.
    Minus,
    /// `..rest` in a list or a record.
    Spread,
    /// `1..10`.
    Range,
    Colon,
}

fn space_between(a: Role, b: Role) -> bool {
    // `- -x`, not `--x`.
    if a == Role::Minus && b == Role::Minus {
        return true;
    }
    !matches!(b, Role::Close | Role::Comma | Role::Dot | Role::Question | Role::Call | Role::Range | Role::Colon)
        && !(a == Role::SpacedQuestion && b == Role::Call)
        && !matches!(a, Role::Open | Role::Call | Role::Dot | Role::Minus | Role::Spread | Role::Range)
}

/// Where a token goes when it starts a line.
#[derive(Clone, Copy, Debug)]
enum Indent {
    /// At the start of the line: top-level code.
    Top,
    /// At the level of the line of piece `n`.
    Same(usize),
    /// One level deeper than the line of piece `n`.
    Inner(usize),
    /// Two levels deeper: a `match` subject continued on the next line, which
    /// must not look like a case.
    Deeper(usize),
    /// `then` or `else` of the `if` number `n`.
    Branch(usize),
}

struct IfShape {
    if_: usize,
    then: usize,
    then_body: usize,
    /// The `if` whose `else` this one follows: `else if`.
    parent: Option<usize>,
}

/// A bracket with elements separated by commas.
struct List {
    close: usize,
    items: bool,
}

struct Shape {
    role: Vec<Role>,
    indent: Vec<Indent>,
    /// `)`, `]`, `}` and `end`: the comments before them belong inside.
    closer: Vec<bool>,
    /// The first thing after an opening bracket, in a block or in a `match`.
    body_start: Vec<bool>,
    /// Must start a line: `end`, and the statements of a block.
    force: Vec<bool>,
    /// Starts a statement, a case or an element: a blank line may come before.
    separate: Vec<bool>,
    ifs: Vec<IfShape>,
    /// The `|>` of each pipeline.
    pipes: Vec<Vec<usize>>,
    /// The variants of each type, with the `|` before each one.
    types: Vec<Vec<(Option<usize>, usize)>>,
    lists: Vec<List>,
    /// The first and last pieces of each top-level item.
    items: Vec<(usize, usize)>,
    /// The statements and the end of each block given to a call: written on
    /// one line, they stay on it.
    inline: Vec<(usize, usize)>,
}

/// Walks the tokens like the parser does (`parser.rs`), to learn the shape of
/// the program. It only reads code that the parser accepted: `None` means a
/// difference between the two, a bug.
struct Shaper<'a> {
    toks: &'a [Token],
    piece_of: &'a [usize],
    pos: usize,
    /// Are new lines significant here? (not inside brackets)
    sig: Vec<bool>,
    /// The last piece read.
    last: usize,
    s: Shape,
    /// The functions with a name ending with `?` (see `parser::predicates`).
    preds: Vec<Sym>,
    /// The names given to values (see `parser::value_names`).
    values: std::collections::HashSet<Sym>,
    /// In the arguments of a call without parentheses (as in the parser).
    no_do: bool,
}

type Item<'a> = fn(&mut Shaper<'a>) -> Option<usize>;

impl<'a> Shaper<'a> {
    fn new(toks: &'a [Token], piece_of: &'a [usize], pieces: usize) -> Shaper<'a> {
        let indent = (0..pieces).map(|i| if i == 0 || i + 1 == pieces { Indent::Top } else { Indent::Inner(i - 1) }).collect();
        let s = Shape {
            role: vec![Role::Word; pieces],
            indent,
            closer: vec![false; pieces],
            body_start: vec![false; pieces],
            force: vec![false; pieces],
            separate: vec![false; pieces],
            ifs: Vec::new(),
            pipes: Vec::new(),
            types: Vec::new(),
            lists: Vec::new(),
            items: Vec::new(),
            inline: Vec::new(),
        };
        Shaper {
            toks,
            piece_of,
            pos: 0,
            sig: vec![true],
            last: 0,
            s,
            preds: parser::predicates(toks),
            values: parser::value_names(toks),
            no_do: false,
        }
    }

    // ----- tokens, as in the parser

    fn significant(&self) -> bool {
        self.sig.last().copied().unwrap_or(true)
    }

    fn skip_newlines(&mut self) {
        while self.toks[self.pos].tok == Tok::Newline {
            self.pos += 1;
        }
    }

    fn skip_soft(&mut self) {
        if !self.significant() {
            self.skip_newlines();
        }
    }

    fn raw(&self) -> &'a Tok {
        let toks = self.toks;
        &toks[self.pos].tok
    }

    fn peek(&mut self) -> &'a Tok {
        self.skip_soft();
        self.raw()
    }

    fn past_newlines(&self) -> &'a Tok {
        let toks = self.toks;
        let mut i = self.pos;
        while toks[i].tok == Tok::Newline {
            i += 1;
        }
        &toks[i].tok
    }

    /// The next token, past new lines only if the next line continues the expression.
    fn peek_cont(&mut self) -> &'a Tok {
        self.skip_soft();
        let next = self.past_newlines();
        if *self.raw() == Tok::Newline && !is_continuation(next) {
            self.raw()
        } else {
            next
        }
    }

    fn take_cont(&mut self) -> Option<usize> {
        if is_continuation(self.past_newlines()) {
            self.skip_newlines();
        }
        self.advance()
    }

    fn advance(&mut self) -> Option<usize> {
        self.skip_soft();
        if matches!(self.raw(), Tok::Newline | Tok::Eof) {
            return None;
        }
        let p = self.piece_of[self.pos];
        self.pos += 1;
        self.last = p;
        Some(p)
    }

    fn take(&mut self, role: Role) -> Option<usize> {
        let p = self.advance()?;
        self.s.role[p] = role;
        Some(p)
    }

    fn with_sig<T>(&mut self, sig: bool, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        self.sig.push(sig);
        let no_do = std::mem::replace(&mut self.no_do, false);
        let r = f(self);
        self.no_do = no_do;
        self.sig.pop();
        r
    }

    fn set(&mut self, piece: usize, indent: Indent) {
        self.s.indent[piece] = indent;
    }

    // ----- program and statements

    fn program(&mut self) -> Option<()> {
        loop {
            self.skip_newlines();
            if *self.raw() == Tok::Eof {
                return Some(());
            }
            let first = match self.peek() {
                Tok::Type => self.type_def()?,
                Tok::Test => self.test_def()?,
                Tok::Use => self.use_def()?,
                _ => self.stmt()?,
            };
            self.set(first, Indent::Top);
            self.s.separate[first] = true;
            if !matches!(self.raw(), Tok::Newline | Tok::Eof) {
                return None;
            }
            self.s.items.push((first, self.last));
        }
    }

    fn stmt(&mut self) -> Option<usize> {
        self.skip_soft();
        if *self.raw() == Tok::Fn && matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Ident(_))) {
            return self.fn_def(true);
        }
        if parser::command_at(self.toks, self.pos, &self.preds, &self.values).is_some() {
            return self.command();
        }
        // Like the parser: `[first, ..] = list`, `{name, ..} = person`, `Some(x) = value`.
        let first = if self.pattern_before_eq() { self.pattern()? } else { self.expr()? };
        if *self.raw() == Tok::Eq {
            let eq = self.advance()?;
            self.set(eq, Indent::Inner(first));
            self.skip_newlines();
            let value = self.expr()?;
            self.set(value, Indent::Inner(eq));
        }
        Some(first)
    }

    /// A call without parentheses: `print x, y`, and a `do` block given to it.
    fn command(&mut self) -> Option<usize> {
        let name = self.primary()?;
        let no_do = std::mem::replace(&mut self.no_do, true);
        let args = self.command_args(name);
        self.no_do = no_do;
        args?;
        if *self.raw() == Tok::Do {
            self.block_arg()?;
        }
        Some(name)
    }

    fn command_args(&mut self, name: usize) -> Option<()> {
        loop {
            let arg = self.expr()?;
            self.set(arg, Indent::Inner(name));
            if *self.raw() != Tok::Comma {
                return Some(());
            }
            let comma = self.take(Role::Comma)?;
            self.set(comma, Indent::Inner(name));
            self.skip_newlines();
        }
    }

    /// Does a definition with a bracketed pattern start here? The same test as the
    /// parser's `pattern_before_eq`.
    fn pattern_before_eq(&self) -> bool {
        let mut i = self.pos;
        if matches!(self.toks[i].tok, Tok::Ctor(_)) {
            i += 1;
        }
        if !matches!(self.toks[i].tok, Tok::LParen | Tok::LBracket | Tok::LBrace) {
            return false;
        }
        let mut depth = 0usize;
        loop {
            match self.toks[i].tok {
                Tok::LParen | Tok::LBracket | Tok::LBrace => depth += 1,
                Tok::RParen | Tok::RBracket | Tok::RBrace => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Tok::Eof => return false,
                _ => {}
            }
            i += 1;
        }
        matches!(self.toks.get(i + 1).map(|t| &t.tok), Some(Tok::Eq))
    }

    /// The statements of a block, up to `end`, one level deeper than the line of `anchor`.
    fn block(&mut self, anchor: usize) -> Option<()> {
        self.block_until(anchor, &Tok::End).map(|_| ())
    }

    /// The statements of a block, up to `close` (`end`, or `}` for `{ |x| ... }`):
    /// the first statement (or the end) and the end.
    fn block_until(&mut self, anchor: usize, close: &Tok) -> Option<(usize, usize)> {
        self.with_sig(true, |p| {
            let mut first = None;
            loop {
                p.skip_newlines();
                match p.raw() {
                    t if t == close => {
                        let e = p.end(anchor)?;
                        return Some((first.unwrap_or(e), e));
                    }
                    Tok::Eof => return None,
                    _ => {}
                }
                let s = p.stmt()?;
                p.set(s, Indent::Inner(anchor));
                p.s.force[s] = true;
                p.s.separate[s] = true;
                p.s.body_start[s] = first.is_none();
                first.get_or_insert(s);
                if !(matches!(p.raw(), Tok::Newline | Tok::Eof | Tok::End) || p.raw() == close) {
                    return None;
                }
            }
        })
    }

    /// A block given to a call: `do |x, y| ... end` or `{ |x| ... }`.
    fn block_arg(&mut self) -> Option<()> {
        let open = self.advance()?;
        let close = if self.toks[self.pos - 1].tok == Tok::LBrace { Tok::RBrace } else { Tok::End };
        match self.raw() {
            Tok::BarBar => {
                self.advance()?;
            }
            Tok::Bar => {
                self.take(Role::Open)?;
                self.with_sig(false, |p| {
                    while *p.peek() != Tok::Bar {
                        let param = p.pattern_atom()?;
                        p.set(param, Indent::Inner(open));
                        if *p.peek() == Tok::Comma {
                            p.take(Role::Comma)?;
                        } else if *p.peek() != Tok::Bar {
                            return None;
                        }
                    }
                    p.take(Role::Close)
                })?;
            }
            _ => {}
        }
        let region = self.block_until(open, &close)?;
        self.s.inline.push(region);
        Some(())
    }

    /// Does a `?` at `pos` that follows a name (as the operator) need a
    /// space before it, so that it is not read as a part of the name?
    fn spaced_question(&self) -> bool {
        let i = self.pos;
        if i == 0 || !matches!(self.toks[i - 1].tok, Tok::Ident(_)) {
            return false;
        }
        let mut toks = vec![self.toks[i - 1].clone(), self.toks[i].clone()];
        // The same tokens, written next to each other, followed by `(`.
        let end = toks[0].span.end as usize;
        toks[1].span = crate::source::Span::new(toks[0].span.file, end, end + 1);
        let next_paren = matches!(self.toks.get(i + 1).map(|t| &t.tok), Some(Tok::LParen));
        next_paren || parser::predicate_at(&toks, 0, &self.preds).is_some()
    }

    /// The `end` (or `}`) of a block.
    fn end(&mut self, anchor: usize) -> Option<usize> {
        let e = self.advance()?;
        self.set(e, Indent::Same(anchor));
        self.s.closer[e] = true;
        self.s.force[e] = true;
        Some(e)
    }

    fn fn_def(&mut self, named: bool) -> Option<usize> {
        let f = self.advance()?;
        if named {
            self.advance()?;
            // `fn adult?(person)`.
            if *self.raw() == Tok::Question {
                self.take(Role::Question)?;
            }
        }
        if *self.peek() != Tok::LParen {
            return None;
        }
        let open = self.take(Role::Call)?;
        self.comma_list(open, &Tok::RParen, Self::pattern)?;
        if named && *self.raw() == Tok::If {
            let k = self.advance()?;
            self.set(k, Indent::Inner(f));
            let guard = self.expr()?;
            self.set(guard, Indent::Inner(f));
        }
        match self.raw() {
            Tok::Eq => {
                let eq = self.advance()?;
                self.set(eq, Indent::Inner(f));
                self.skip_newlines();
                let body = self.expr()?;
                self.set(body, Indent::Inner(eq));
            }
            Tok::Newline => self.block(f)?,
            _ => return None,
        }
        Some(f)
    }

    fn type_def(&mut self) -> Option<usize> {
        let k = self.advance()?;
        self.advance()?;
        if *self.peek() != Tok::Eq {
            return None;
        }
        self.advance()?;
        self.skip_newlines();
        let mut bar = None;
        if *self.peek() == Tok::Bar {
            let b = self.advance()?;
            self.set(b, Indent::Inner(k));
            bar = Some(b);
        }
        let mut variants = Vec::new();
        loop {
            self.skip_newlines();
            let v = self.advance()?;
            self.set(v, Indent::Inner(k));
            if *self.raw() == Tok::LParen {
                let open = self.take(Role::Call)?;
                self.comma_list(open, &Tok::RParen, Self::advance)?;
            }
            variants.push((bar, v));
            if *self.past_newlines() != Tok::Bar {
                break;
            }
            self.skip_newlines();
            let b = self.advance()?;
            self.set(b, Indent::Inner(k));
            bar = Some(b);
        }
        self.s.types.push(variants);
        Some(k)
    }

    fn test_def(&mut self) -> Option<usize> {
        let k = self.advance()?;
        self.advance()?;
        match self.raw() {
            Tok::Eq => {
                let eq = self.advance()?;
                self.set(eq, Indent::Inner(k));
                self.skip_newlines();
                let body = self.expr()?;
                self.set(body, Indent::Inner(eq));
            }
            Tok::Newline => self.block(k)?,
            _ => return None,
        }
        Some(k)
    }

    /// `use geometry`, `use shapes/circle as circle`, `use "x.ry"`.
    fn use_def(&mut self) -> Option<usize> {
        let k = self.advance()?;
        self.advance()?;
        while *self.raw() == Tok::Slash {
            self.take(Role::Dot)?;
            self.advance()?;
        }
        if matches!(self.raw(), Tok::Ident(s) if &*s.name() == "as") {
            self.advance()?;
            self.advance()?;
        }
        Some(k)
    }

    // ----- expressions

    fn expr(&mut self) -> Option<usize> {
        self.or_expr()
    }

    /// `left op right op right...`: the lines that continue it are one level
    /// deeper than its first line.
    fn operators(&mut self, ops: &[Tok], operand: Item<'a>) -> Option<usize> {
        let left = operand(self)?;
        while ops.contains(self.peek_cont()) {
            let op = self.take_cont()?;
            self.set(op, Indent::Inner(left));
            self.skip_newlines();
            let right = operand(self)?;
            self.set(right, Indent::Inner(left));
        }
        Some(left)
    }

    fn or_expr(&mut self) -> Option<usize> {
        self.operators(&[Tok::Or], Self::and_expr)
    }

    fn and_expr(&mut self) -> Option<usize> {
        self.operators(&[Tok::And], Self::not_expr)
    }

    fn not_expr(&mut self) -> Option<usize> {
        if *self.peek() == Tok::Not {
            let t = self.advance()?;
            let e = self.not_expr()?;
            self.set(e, Indent::Inner(t));
            return Some(t);
        }
        self.cmp_expr()
    }

    fn cmp_expr(&mut self) -> Option<usize> {
        let left = self.pipe_expr()?;
        if matches!(self.peek_cont(), Tok::EqEq | Tok::NotEq | Tok::Lt | Tok::Le | Tok::Gt | Tok::Ge) {
            let op = self.take_cont()?;
            self.set(op, Indent::Inner(left));
            self.skip_newlines();
            let right = self.pipe_expr()?;
            self.set(right, Indent::Inner(left));
        }
        Some(left)
    }

    fn pipe_expr(&mut self) -> Option<usize> {
        let left = self.range_expr()?;
        let mut pipes = Vec::new();
        while *self.peek_cont() == Tok::PipeGt {
            let pipe = self.take_cont()?;
            self.set(pipe, Indent::Inner(left));
            pipes.push(pipe);
            self.skip_newlines();
            let step = self.pipe_step()?;
            self.set(step, Indent::Inner(left));
        }
        if !pipes.is_empty() {
            self.s.pipes.push(pipes);
        }
        Some(left)
    }

    fn pipe_step(&mut self) -> Option<usize> {
        let first = self.primary()?;
        while *self.raw() == Tok::Dot {
            self.take(Role::Dot)?;
            self.member()?;
            if *self.raw() == Tok::LParen {
                let open = self.take(Role::Call)?;
                self.comma_list(open, &Tok::RParen, Self::expr)?;
                self.postfix_ops(first, true)?;
                return Some(first);
            }
        }
        if *self.raw() == Tok::LParen {
            let open = self.take(Role::Call)?;
            self.comma_list(open, &Tok::RParen, Self::expr)?;
        }
        self.postfix_ops(first, true)?;
        Some(first)
    }

    fn range_expr(&mut self) -> Option<usize> {
        let left = self.add_expr()?;
        if *self.peek_cont() == Tok::DotDot {
            let op = self.take_cont()?;
            self.s.role[op] = Role::Range;
            self.set(op, Indent::Inner(left));
            self.skip_newlines();
            let right = self.add_expr()?;
            self.set(right, Indent::Inner(left));
        }
        Some(left)
    }

    fn add_expr(&mut self) -> Option<usize> {
        self.operators(&[Tok::Plus, Tok::Minus], Self::mul_expr)
    }

    fn mul_expr(&mut self) -> Option<usize> {
        self.operators(&[Tok::Star, Tok::Slash, Tok::SlashSlash, Tok::Percent], Self::unary)
    }

    fn unary(&mut self) -> Option<usize> {
        if *self.peek() != Tok::Minus {
            return self.power();
        }
        let minus = self.take(Role::Minus)?;
        let e = self.unary()?;
        self.set(e, Indent::Inner(minus));
        Some(minus)
    }

    fn power(&mut self) -> Option<usize> {
        // A name may be given a block: `print do ... end`.
        let callable = matches!(self.peek(), Tok::Ident(_)) && !parser::unless_at(self.toks, self.pos);
        let base = self.primary()?;
        self.postfix_ops(base, callable)?;
        if *self.peek_cont() == Tok::StarStar {
            let op = self.take_cont()?;
            self.set(op, Indent::Inner(base));
            self.skip_newlines();
            let exp = self.unary()?;
            self.set(exp, Indent::Inner(base));
        }
        Some(base)
    }

    /// The name after a `.`, with its `?` if it is a part of it (`.even?`).
    fn member(&mut self) -> Option<usize> {
        let name = self.advance()?;
        if parser::predicate_at(self.toks, self.pos - 1, &self.preds).is_some() {
            self.take(Role::Question)?;
        }
        Some(name)
    }

    /// Calls, `[index]`, `?`, `.field`, `.method()` and blocks after an expression.
    /// `callable`: the expression so far is a name, a call or a field, which
    /// a block may follow.
    fn postfix_ops(&mut self, first: usize, mut callable: bool) -> Option<()> {
        loop {
            // A block given to the call, on its line (as the parser reads it).
            let block = (matches!(self.raw(), Tok::Do) && !self.no_do)
                || (matches!(self.raw(), Tok::LBrace)
                    && matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Bar | Tok::BarBar)));
            if block {
                if !callable {
                    return Some(());
                }
                self.block_arg()?;
                continue;
            }
            self.skip_soft();
            match self.raw() {
                Tok::LParen => {
                    let open = self.take(Role::Call)?;
                    self.set(open, Indent::Inner(first));
                    self.comma_list(open, &Tok::RParen, Self::expr)?;
                    callable = true;
                }
                Tok::LBracket => {
                    callable = false;
                    let open = self.take(Role::Call)?;
                    self.set(open, Indent::Inner(first));
                    self.with_sig(false, |p| {
                        let i = p.expr()?;
                        p.set(i, Indent::Inner(open));
                        p.s.body_start[i] = true;
                        if *p.peek() != Tok::RBracket {
                            return None;
                        }
                        p.close(open)
                    })?;
                }
                Tok::Question => {
                    let role = if self.spaced_question() { Role::SpacedQuestion } else { Role::Question };
                    self.take(role)?;
                    callable = false;
                }
                _ => {
                    if *self.peek_cont() != Tok::Dot {
                        return Some(());
                    }
                    let dot = self.take_cont()?;
                    self.s.role[dot] = Role::Dot;
                    self.set(dot, Indent::Inner(first));
                    self.member()?;
                    if *self.raw() == Tok::LParen {
                        let open = self.take(Role::Call)?;
                        self.comma_list(open, &Tok::RParen, Self::expr)?;
                    }
                    callable = true;
                }
            }
        }
    }

    fn close(&mut self, open: usize) -> Option<usize> {
        let c = self.take(Role::Close)?;
        self.set(c, Indent::Same(open));
        self.s.closer[c] = true;
        Some(c)
    }

    /// `item, item, ...` up to `close`, after the opening bracket `open`.
    fn comma_list(&mut self, open: usize, close: &Tok, item: Item<'a>) -> Option<()> {
        self.with_sig(false, |p| {
            let mut items = false;
            while p.peek() != close {
                let first = item(p)?;
                p.set(first, Indent::Inner(open));
                p.s.separate[first] = true;
                p.s.body_start[first] = !items;
                items = true;
                if *p.peek() == Tok::Comma {
                    let comma = p.take(Role::Comma)?;
                    p.set(comma, Indent::Inner(open));
                } else if p.peek() != close {
                    return None;
                }
            }
            let close = p.close(open)?;
            p.s.lists.push(List { close, items });
            Some(())
        })
    }

    fn primary(&mut self) -> Option<usize> {
        match self.peek() {
            Tok::Ident(_) if parser::unless_at(self.toks, self.pos) => self.if_expr(),
            Tok::Ident(_) => {
                let name = self.advance()?;
                // `even?`: one name.
                if parser::predicate_at(self.toks, self.pos - 1, &self.preds).is_some() {
                    self.take(Role::Question)?;
                }
                Some(name)
            }
            Tok::Int(_) | Tok::Big(_) | Tok::Float(_) | Tok::True | Tok::False | Tok::Str(_) | Tok::Ctor(_) | Tok::Symbol(_) => {
                self.advance()
            }
            Tok::LParen => {
                let open = self.take(Role::Open)?;
                self.with_sig(false, |p| {
                    if *p.peek() != Tok::RParen {
                        let e = p.expr()?;
                        p.set(e, Indent::Inner(open));
                        p.s.body_start[e] = true;
                        if *p.peek() != Tok::RParen {
                            return None;
                        }
                    }
                    p.close(open)
                })?;
                Some(open)
            }
            Tok::LBracket => {
                let open = self.take(Role::Open)?;
                self.comma_list(open, &Tok::RBracket, Self::list_item)?;
                Some(open)
            }
            Tok::LBrace => {
                let open = self.take(Role::Open)?;
                self.comma_list(open, &Tok::RBrace, Self::record_item)?;
                Some(open)
            }
            Tok::Fn => self.fn_def(false),
            Tok::If => self.if_expr(),
            Tok::Match => self.match_expr(),
            Tok::Do => {
                let d = self.advance()?;
                self.block(d)?;
                Some(d)
            }
            _ => None,
        }
    }

    fn spread(&mut self) -> Option<usize> {
        let dots = self.take(Role::Spread)?;
        let e = self.expr()?;
        self.set(e, Indent::Inner(dots));
        Some(dots)
    }

    fn list_item(&mut self) -> Option<usize> {
        if *self.peek() == Tok::DotDot {
            return self.spread();
        }
        self.expr()
    }

    fn record_item(&mut self) -> Option<usize> {
        if *self.peek() == Tok::DotDot {
            return self.spread();
        }
        let key = self.advance()?;
        if *self.peek() == Tok::Colon {
            self.take(Role::Colon)?;
            let value = self.expr()?;
            self.set(value, Indent::Inner(key));
        }
        Some(key)
    }

    fn if_expr(&mut self) -> Option<usize> {
        let if_ = self.advance()?;
        let cond = self.expr()?;
        self.set(cond, Indent::Inner(if_));
        if *self.past_newlines() == Tok::Then {
            self.skip_newlines();
        }
        if *self.peek() != Tok::Then {
            return None;
        }
        let then = self.advance()?;
        self.skip_newlines();
        let then_body = self.expr()?;
        self.set(then_body, Indent::Inner(then));
        if *self.past_newlines() == Tok::Else {
            self.skip_newlines();
        }
        if *self.peek() != Tok::Else {
            return None;
        }
        let else_ = self.advance()?;
        self.skip_newlines();
        let else_body = self.expr()?;
        self.set(else_body, Indent::Inner(else_));
        let n = self.s.ifs.len();
        if let Some(inner) = self.s.ifs.last_mut().filter(|inner| inner.if_ == else_body) {
            inner.parent = Some(n);
        }
        self.s.ifs.push(IfShape { if_, then, then_body, parent: None });
        self.set(then, Indent::Branch(n));
        self.set(else_, Indent::Branch(n));
        Some(if_)
    }

    fn match_expr(&mut self) -> Option<usize> {
        let m = self.advance()?;
        let subject = self.with_sig(true, Self::expr)?;
        self.set(subject, Indent::Inner(m));
        for p in subject + 1..=self.last {
            if matches!(self.s.indent[p], Indent::Inner(a) if a == subject) {
                self.set(p, Indent::Deeper(subject));
            }
        }
        if *self.raw() != Tok::Newline {
            return None;
        }
        self.with_sig(true, |p| {
            let mut first = true;
            loop {
                p.skip_newlines();
                match p.raw() {
                    Tok::End => {
                        p.end(m)?;
                        return Some(m);
                    }
                    Tok::Eof => return None,
                    _ => {}
                }
                let pat = p.pattern()?;
                p.set(pat, Indent::Inner(m));
                p.s.separate[pat] = true;
                p.s.body_start[pat] = first;
                first = false;
                if *p.peek() == Tok::If {
                    let k = p.advance()?;
                    p.set(k, Indent::Inner(pat));
                    let guard = p.expr()?;
                    p.set(guard, Indent::Inner(pat));
                }
                if *p.peek() != Tok::Arrow {
                    return None;
                }
                let arrow = p.advance()?;
                p.set(arrow, Indent::Inner(pat));
                p.skip_newlines();
                let body = p.expr()?;
                p.set(body, Indent::Inner(arrow));
                if !matches!(p.raw(), Tok::Newline | Tok::End) {
                    return None;
                }
            }
        })
    }

    // ----- patterns

    fn pattern(&mut self) -> Option<usize> {
        let first = self.pattern_part()?;
        while *self.peek() == Tok::Bar {
            let bar = self.advance()?;
            self.set(bar, Indent::Inner(first));
            let alt = self.pattern_part()?;
            self.set(alt, Indent::Inner(first));
        }
        Some(first)
    }

    /// A pattern, a range `1..9`, or a text and the rest, `"Mr " + name`.
    fn pattern_part(&mut self) -> Option<usize> {
        let first = self.pattern_atom()?;
        if matches!(self.peek(), Tok::DotDot | Tok::Plus) {
            let op = if *self.peek() == Tok::DotDot { self.take(Role::Range)? } else { self.advance()? };
            self.set(op, Indent::Inner(first));
            let second = self.pattern_atom()?;
            self.set(second, Indent::Inner(first));
        }
        Some(first)
    }

    fn pattern_atom(&mut self) -> Option<usize> {
        match self.peek() {
            // `geo.Circle(r)`: a constructor of a file used as `geo`.
            Tok::Ident(_) if self.toks.get(self.pos + 1).is_some_and(|t| t.tok == Tok::Dot) => {
                let m = self.advance()?;
                self.take(Role::Dot)?;
                self.advance()?;
                if *self.raw() == Tok::LParen {
                    let open = self.take(Role::Call)?;
                    self.comma_list(open, &Tok::RParen, Self::pattern)?;
                }
                Some(m)
            }
            Tok::Ident(_) | Tok::Int(_) | Tok::Big(_) | Tok::Float(_) | Tok::True | Tok::False | Tok::Str(_) | Tok::Symbol(_) => {
                self.advance()
            }
            Tok::Ctor(_) => {
                let c = self.advance()?;
                if *self.raw() == Tok::LParen {
                    let open = self.take(Role::Call)?;
                    self.comma_list(open, &Tok::RParen, Self::pattern)?;
                }
                Some(c)
            }
            Tok::Minus => {
                let minus = self.take(Role::Minus)?;
                self.advance()?;
                Some(minus)
            }
            Tok::LParen => {
                let open = self.take(Role::Open)?;
                self.with_sig(false, |p| {
                    if *p.peek() != Tok::RParen {
                        let inner = p.pattern()?;
                        p.set(inner, Indent::Inner(open));
                        p.s.body_start[inner] = true;
                        if *p.peek() != Tok::RParen {
                            return None;
                        }
                    }
                    p.close(open)
                })?;
                Some(open)
            }
            Tok::LBracket => {
                let open = self.take(Role::Open)?;
                self.with_sig(false, |p| p.list_pattern(open))?;
                Some(open)
            }
            Tok::LBrace => {
                let open = self.take(Role::Open)?;
                self.comma_list(open, &Tok::RBrace, Self::record_pattern_item)?;
                Some(open)
            }
            _ => None,
        }
    }

    /// `[a, b, ..rest]`, after `[`.
    fn list_pattern(&mut self, open: usize) -> Option<()> {
        let mut items = false;
        while *self.peek() != Tok::RBracket {
            let first = if *self.peek() == Tok::DotDot {
                let dots = self.take(Role::Spread)?;
                if matches!(self.peek(), Tok::Ident(_)) {
                    self.advance()?;
                }
                dots
            } else {
                self.pattern()?
            };
            self.set(first, Indent::Inner(open));
            self.s.separate[first] = true;
            self.s.body_start[first] = !items;
            items = true;
            let rest = self.s.role[first] == Role::Spread;
            if *self.peek() == Tok::Comma {
                let comma = self.take(Role::Comma)?;
                self.set(comma, Indent::Inner(open));
            } else {
                break;
            }
            if rest {
                break;
            }
        }
        if *self.peek() != Tok::RBracket {
            return None;
        }
        let close = self.close(open)?;
        self.s.lists.push(List { close, items });
        Some(())
    }

    fn record_pattern_item(&mut self) -> Option<usize> {
        if *self.peek() == Tok::DotDot {
            return self.take(Role::Spread);
        }
        let key = self.advance()?;
        if *self.peek() == Tok::Colon {
            self.take(Role::Colon)?;
            let pat = self.pattern()?;
            self.set(pat, Indent::Inner(key));
        }
        Some(key)
    }
}

// ----- the layout ---------------------------------------------------------

/// Tokens the formatter adds or removes.
struct Edits {
    skip: Vec<bool>,
    comma_after: Vec<bool>,
    bar_before: Vec<bool>,
}

/// The formatted code, or `None` if the shape of the program was not understood.
fn layout(src: &str) -> Option<String> {
    let (tokens, mut pieces, piece_of) = read(src)?;
    let mut shaper = Shaper::new(&tokens, &piece_of, pieces.len());
    shaper.program()?;
    let shape = shaper.s;
    let edits = normalize(&mut pieces, &shape);
    let mut printer = Printer {
        pieces: &pieces,
        shape: &shape,
        edits: &edits,
        out: Vec::new(),
        code: String::new(),
        level: 0,
        comment: None,
        last_role: Role::Word,
        line_of: vec![0; pieces.len()],
        starts_line: vec![false; pieces.len()],
    };
    printer.print();
    Some(render(&printer.out))
}

/// Moves the line break (and the comments) after the token `p` before it:
/// `x |>` + new line + `f` becomes `x` + new line + `|> f`.
fn break_before(pieces: &mut [Piece], p: usize) {
    let after = std::mem::take(&mut pieces[p + 1].gap);
    let gap = &mut pieces[p].gap;
    if let Some(c) = after.trailing {
        if gap.broken {
            gap.lines.push(Trivia::Comment(c));
        } else {
            gap.trailing = Some(c);
        }
    }
    gap.lines.extend(after.lines);
    gap.broken = true;
}

/// The name of the function defined by the item starting at `first`.
fn fn_name(pieces: &[Piece], first: usize) -> Option<Sym> {
    match (&pieces[first].tok, &pieces.get(first + 1)?.tok) {
        (Tok::Fn, Tok::Ident(name)) => Some(*name),
        _ => None,
    }
}

/// Decides the line breaks and blank lines of the canonical style.
fn normalize(pieces: &mut [Piece], shape: &Shape) -> Edits {
    let n = pieces.len();
    let mut e = Edits { skip: vec![false; n], comma_after: vec![false; n], bar_before: vec![false; n] };
    // A block written on one line stays on one line.
    let mut force = shape.force.clone();
    for &(first, last) in &shape.inline {
        if (first..=last).all(|i| !pieces[i].gap.broken) {
            force[first..=last].iter_mut().for_each(|f| *f = false);
        }
    }
    for (p, force) in pieces.iter_mut().zip(&force) {
        p.gap.broken |= *force;
    }
    // A pipeline written on several lines: one step per line.
    for pipes in &shape.pipes {
        if pipes.iter().any(|&p| pieces[p].gap.broken || pieces[p + 1].gap.broken) {
            for &p in pipes {
                break_before(pieces, p);
            }
        }
    }
    // A type written on several lines: one variant per line, after `|`.
    for variants in &shape.types {
        let multi = variants.iter().any(|&(bar, v)| pieces[v].gap.broken || bar.is_some_and(|b| pieces[b].gap.broken));
        for (i, &(bar, v)) in variants.iter().enumerate() {
            match bar {
                Some(b) if multi => break_before(pieces, b),
                None if multi => {
                    e.bar_before[v] = true;
                    pieces[v].gap.broken = true;
                }
                Some(b) if i == 0 => e.skip[b] = true,
                _ => {}
            }
        }
    }
    // A list on several lines ends with a comma; on one line, without.
    for list in &shape.lists {
        let last = list.close - 1;
        let comma = pieces[last].tok == Tok::Comma;
        if !list.items {
            continue;
        }
        if pieces[list.close].gap.broken {
            e.comma_after[last] = !comma;
        } else if comma && !pieces[last].gap.broken {
            e.skip[last] = true;
        }
    }
    for (i, p) in pieces.iter_mut().enumerate() {
        let lines = &mut p.gap.lines;
        lines.dedup_by(|a, b| *a == Trivia::Blank && *b == Trivia::Blank);
        // Blank lines separate statements, cases, elements and steps (`|> f`),
        // not the parts of an expression. A blank line after `##` is kept: it
        // decides whether the comment documents what follows.
        if !(shape.separate[i] || is_continuation(&p.tok) || i + 1 == n) {
            let mut doc = false;
            lines.retain(|t| {
                let keep = *t != Trivia::Blank || doc;
                doc = matches!(t, Trivia::Comment(c) if c.starts_with("##"));
                keep
            });
        }
        if i == 0 || shape.body_start[i] {
            while lines.first() == Some(&Trivia::Blank) {
                lines.remove(0);
            }
        }
        if i + 1 == n || shape.closer[i] {
            while lines.last() == Some(&Trivia::Blank) {
                lines.pop();
            }
        }
    }
    // A definition on several lines has blank lines around it. The definitions
    // of one function (`fn fib(0) = 0`, `fn fib(n) = ...`) stay together.
    let multi_line = |pieces: &[Piece], (first, last): (usize, usize)| {
        (first..=last).any(|i| (i > first && (pieces[i].gap.broken || e.bar_before[i])) || pieces[i].text.contains('\n'))
    };
    // The first piece of each group of items, and whether it spans several lines.
    let mut groups: Vec<(usize, bool)> = Vec::new();
    for (k, &item) in shape.items.iter().enumerate() {
        let multi = multi_line(pieces, item);
        let name = fn_name(pieces, item.0);
        match groups.last_mut() {
            Some(group) if name.is_some() && name == fn_name(pieces, shape.items[k - 1].0) => group.1 |= multi,
            _ => groups.push((item.0, multi)),
        }
    }
    for pair in groups.windows(2) {
        let ((_, before), (first, after)) = (pair[0], pair[1]);
        let lines = &mut pieces[first].gap.lines;
        if before && lines.first() != Some(&Trivia::Blank) {
            lines.insert(0, Trivia::Blank);
        }
        if after {
            // Before the comments that introduce it.
            let at = lines.len() - lines.iter().rev().take_while(|t| matches!(t, Trivia::Comment(_))).count();
            if at == 0 || lines[at - 1] != Trivia::Blank {
                lines.insert(at, Trivia::Blank);
            }
        }
    }
    e
}

/// A line of the result.
enum Out {
    Blank,
    /// A comment on its own line, at a level.
    Comment(usize, String),
    /// Code at a level, with maybe a comment at the end.
    Code {
        level: usize,
        code: String,
        comment: Option<String>,
    },
}

struct Printer<'a> {
    pieces: &'a [Piece],
    shape: &'a Shape,
    edits: &'a Edits,
    out: Vec<Out>,
    /// The current line, without its indentation.
    code: String,
    level: usize,
    comment: Option<String>,
    last_role: Role,
    /// The level of the line of each piece.
    line_of: Vec<usize>,
    starts_line: Vec<bool>,
}

impl Printer<'_> {
    fn print(&mut self) {
        let n = self.pieces.len();
        for i in 0..n {
            if !self.edits.skip[i] {
                self.gap(i);
                if i + 1 == n {
                    break;
                }
                self.starts_line[i] = self.code.is_empty();
                if self.edits.bar_before[i] {
                    self.put("|", Role::Word);
                }
                let pieces = self.pieces;
                self.put(&pieces[i].text, self.shape.role[i]);
                if self.edits.comma_after[i] {
                    self.put(",", Role::Comma);
                }
            }
            self.line_of[i] = self.level;
        }
        self.flush();
    }

    fn put(&mut self, text: &str, role: Role) {
        if !self.code.is_empty() && space_between(self.last_role, role) {
            self.code.push(' ');
        }
        self.code.push_str(text);
        self.last_role = role;
    }

    fn flush(&mut self) {
        if !self.code.is_empty() || self.comment.is_some() {
            let code = std::mem::take(&mut self.code);
            self.out.push(Out::Code { level: self.level, code, comment: self.comment.take() });
        }
    }

    /// What comes before piece `i`: comments, and a new line if it starts one.
    fn gap(&mut self, i: usize) {
        let pieces = self.pieces;
        let gap = &pieces[i].gap;
        if let Some(c) = &gap.trailing {
            self.comment = Some(c.clone());
        }
        let first = self.out.is_empty() && self.code.is_empty();
        if !(gap.broken || gap.trailing.is_some() || !gap.lines.is_empty() || first) {
            return;
        }
        self.flush();
        let level = self.level_of(i);
        // Comments before `end` or a closing bracket belong inside.
        let inside = if self.shape.closer[i] { level + 1 } else { level };
        for t in &gap.lines {
            self.out.push(match t {
                Trivia::Blank => Out::Blank,
                Trivia::Comment(c) => Out::Comment(inside, c.clone()),
            });
        }
        self.level = level;
    }

    /// The level of piece `i` when it starts a line.
    fn level_of(&self, i: usize) -> usize {
        match self.shape.indent[i] {
            Indent::Top => 0,
            Indent::Same(a) => self.line_of[a],
            Indent::Inner(a) => self.line_of[a] + 1,
            Indent::Deeper(a) => self.line_of[a] + 2,
            Indent::Branch(k) => self.branch(k, i == self.shape.ifs[k].then),
        }
    }

    /// The level of `then` or `else` at the start of a line:
    /// ```text
    /// if a then b          x = if a then b        x = if a then
    /// else c                 else c                 b
    ///                                             else
    ///                                               c
    /// ```
    fn branch(&self, k: usize, then: bool) -> usize {
        let f = &self.shape.ifs[k];
        // `else if` on one line: the same level as the `if` it follows.
        if let Some(parent) = f.parent {
            if !self.pieces[f.if_].gap.broken {
                return self.branch(parent, then);
            }
        }
        let base = self.line_of[f.if_];
        if then || self.pieces[f.then].gap.broken {
            return base + 1;
        }
        if self.starts_line[f.if_] || self.pieces[f.then_body].gap.broken {
            base
        } else {
            base + 1
        }
    }
}

/// The text of the lines, with the comments at the end of consecutive lines aligned.
fn render(out: &[Out]) -> String {
    let mut text = String::new();
    let mut i = 0;
    while i < out.len() {
        let run = out[i..].iter().take_while(|o| matches!(o, Out::Code { comment: Some(_), .. })).count().max(1);
        let column = out[i..i + run].iter().map(width).max().unwrap_or(0) + 2;
        for o in &out[i..i + run] {
            match o {
                Out::Blank => {}
                Out::Comment(level, c) => {
                    text.push_str(&" ".repeat(level * WIDTH));
                    text.push_str(c);
                }
                Out::Code { level, code, comment } => {
                    text.push_str(&" ".repeat(level * WIDTH));
                    text.push_str(code);
                    if let Some(c) = comment {
                        text.push_str(&" ".repeat(column - width(o)));
                        text.push_str(c);
                    }
                }
            }
            text.push('\n');
        }
        i += run;
    }
    text
}

/// The width of the code of a line, in characters.
fn width(o: &Out) -> usize {
    match o {
        Out::Code { level, code, .. } => match code.rfind('\n') {
            // A text written on several lines: its last line.
            Some(i) => code[i + 1..].chars().count(),
            None => level * WIDTH + code.chars().count(),
        },
        _ => 0,
    }
}

// ----- the `rythme fmt` command -----------------------------------------------

/// `rythme fmt [--check] FILE|FOLDER|- ...`. Returns the exit code.
pub fn command(args: &[String]) -> i32 {
    let mut check = false;
    let mut targets = Vec::new();
    for a in args {
        match a.as_str() {
            "--check" | "--verifier" | "--vérifier" => check = true,
            o if o.starts_with("--") => {
                let mut msg = t!("unknown option `{o}` for `rythme fmt`", "option inconnue `{o}` pour `rythme fmt`");
                if let Some(best) = suggest::closest(o, ["--check"]) {
                    msg.push_str(&t!("\ndid you mean `{best}`?", "\nvouliez-vous dire `{best}` ?"));
                }
                msg.push_str(&t!("\nsee `rythme help fmt`", "\nvoir `rythme help fmt`"));
                eprintln!("{msg}");
                return 2;
            }
            _ => targets.push(a.as_str()),
        }
    }
    if targets.is_empty() {
        eprintln!(
            "{}",
            t!(
                "which files? Example: rythme fmt hello.ry\n  a folder formats all its .ry files: rythme fmt .\n  - reads the code from the standard input\n  --check only checks, for continuous integration\nsee `rythme help fmt`",
                "quels fichiers ? Exemple : rythme fmt bonjour.ry\n  un dossier met en forme tous ses fichiers .ry : rythme fmt .\n  - lit le code sur l'entrée standard\n  --check vérifie seulement, pour l'intégration continue\nvoir `rythme help fmt`"
            )
        );
        return 2;
    }
    let mut run = Run { check, changed: 0, unchanged: 0, errors: 0, color: diagnostic::use_color() };
    let mut files = 0;
    for target in targets {
        if target == "-" {
            run.stdin();
            continue;
        }
        // `rythme fmt hello` formats `hello.ry`, like `rythme run hello`.
        let with_ext = PathBuf::from(format!("{target}.ry"));
        let path = if !Path::new(target).exists() && Path::new(target).extension().is_none() && with_ext.is_file() {
            with_ext.as_path()
        } else {
            Path::new(target)
        };
        if path.is_dir() {
            let mut found = Vec::new();
            ry_files(path, &mut found);
            if found.is_empty() {
                eprintln!("{}", t!("no .ry file in `{target}`", "aucun fichier .ry dans `{target}`"));
            }
            for f in found {
                run.file(&f);
                files += 1;
            }
        } else if path.is_file() {
            run.file(path);
            files += 1;
        } else {
            run.report(&not_found(path));
            run.errors += 1;
        }
    }
    if files > 0 {
        run.summary();
    }
    if run.errors > 0 || (check && run.changed > 0) {
        1
    } else {
        0
    }
}

struct Run {
    check: bool,
    changed: usize,
    unchanged: usize,
    errors: usize,
    color: bool,
}

impl Run {
    fn report(&self, d: &Diagnostic) {
        io::err(&format!("{}\n", d.render(self.color)));
    }

    fn file(&mut self, path: &Path) {
        let shown = path.display().to_string();
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                self.report(&Diagnostic::error(
                    "E017",
                    t!("cannot read the file `{shown}`: {e}", "impossible de lire le fichier `{shown}` : {e}"),
                ));
                self.errors += 1;
                return;
            }
        };
        let out = match format_named(&shown, &src) {
            Ok(out) => out,
            Err(diags) => {
                diags.iter().for_each(|d| self.report(d));
                self.errors += 1;
                return;
            }
        };
        if out == src {
            self.unchanged += 1;
        } else if self.check {
            io::out_line(&t!("needs formatting: {shown}", "à mettre en forme : {shown}"));
            self.changed += 1;
        } else if let Err(e) = std::fs::write(path, out) {
            self.report(&Diagnostic::error(
                "E017",
                t!("cannot write the file `{shown}`: {e}", "impossible d'écrire le fichier `{shown}` : {e}"),
            ));
            self.errors += 1;
        } else {
            io::out_line(&t!("formatted: {shown}", "mis en forme : {shown}"));
            self.changed += 1;
        }
    }

    /// `-`: the code comes from the standard input, the result goes to the standard output.
    fn stdin(&mut self) {
        let mut src = String::new();
        if let Err(e) = std::io::Read::read_to_string(&mut std::io::stdin(), &mut src) {
            self.report(&Diagnostic::error(
                "E017",
                t!("cannot read the standard input: {e}", "impossible de lire l'entrée standard : {e}"),
            ));
            self.errors += 1;
            return;
        }
        match format_named("<stdin>", &src) {
            Ok(out) if self.check => {
                if out != src {
                    io::out_line(&t!("needs formatting: <stdin>", "à mettre en forme : <stdin>"));
                    self.changed += 1;
                }
            }
            Ok(out) => io::out(&out),
            Err(diags) => {
                diags.iter().for_each(|d| self.report(d));
                self.errors += 1;
            }
        }
    }

    fn summary(&self) {
        let (changed, unchanged, errors) = (self.changed, self.unchanged, self.errors);
        let line = if !self.check {
            t!("{changed} file(s) formatted, {unchanged} already formatted", "{changed} fichier(s) mis en forme, {unchanged} déjà en forme")
        } else if changed == 0 && errors == 0 {
            t!("✓ {unchanged} file(s) already formatted", "✓ {unchanged} fichier(s) déjà en forme")
        } else {
            t!(
                "✗ {changed} file(s) to format, {unchanged} already formatted; `rythme fmt` (without --check) formats them",
                "✗ {changed} fichier(s) à mettre en forme, {unchanged} déjà en forme ; `rythme fmt` (sans --check) les met en forme"
            )
        };
        io::out_line(&line);
        if errors > 0 {
            io::out_line(&t!(
                "✗ {errors} file(s) left as they are because of errors",
                "✗ {errors} fichier(s) laissé(s) tel(s) quel(s) à cause d'erreurs"
            ));
        }
    }
}

/// The `.ry` files of a folder and of its subfolders (except hidden ones, like `.git`), sorted.
pub(crate) fn ry_files(dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let path = e.path();
        let hidden = e.file_name().to_string_lossy().starts_with('.');
        match e.file_type() {
            Ok(t) if t.is_dir() && !hidden => ry_files(&path, found),
            Ok(t) if t.is_file() && path.extension().is_some_and(|x| x == "ry") => found.push(path),
            _ => {}
        }
    }
}

pub(crate) fn not_found(path: &Path) -> Diagnostic {
    let shown = path.display();
    let mut d = Diagnostic::error("E017", t!("file or folder `{shown}` not found", "fichier ou dossier `{shown}` introuvable"));
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    let wanted = path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    let names: Vec<String> =
        std::fs::read_dir(dir).map(|es| es.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default();
    if let Some(best) = suggest::closest(&wanted, names.iter().map(String::as_str)) {
        d = d.help(t!("did you mean `{}`?", "vouliez-vous dire `{}` ?", dir.join(best).display()));
    }
    d
}

#[cfg(test)]
mod tests {
    use super::format;

    #[track_caller]
    fn fmt(src: &str) -> String {
        match format(src) {
            Ok(out) => {
                assert_eq!(format(&out).ok().as_deref(), Some(out.as_str()), "not idempotent:\n{out}");
                out
            }
            Err(d) => panic!("cannot format:\n{src}\n{}", d[0].render(false)),
        }
    }

    #[track_caller]
    fn same(src: &str) {
        assert_eq!(fmt(src), src);
    }

    #[track_caller]
    fn gives(src: &str, expected: &str) {
        assert_eq!(fmt(src), expected, "from:\n{src}");
    }

    #[test]
    fn spaces() {
        gives("x=1+2*3\n", "x = 1 + 2 * 3\n");
        gives("print( f (1 ,2) , [ 1,2 ] , { a : 1 , ..r } )\n", "print(f(1, 2), [1, 2], {a: 1, ..r})\n");
        gives("y = - x + -1 - - 2\n", "y = -x + -1 - -2\n");
        gives("y = --x\n", "y = - -x\n");
        gives("z = 1 .. 10\nw = [ p , .. rest ]\n", "z = 1..10\nw = [p, ..rest]\n");
        gives("v = xs . map ( f ) [ 0 ] ?\n", "v = xs.map(f)[0]?\n");
        gives("f = fn ( x ) = not x and(x or  y)\n", "f = fn(x) = not x and (x or y)\n");
        gives("b = 2**-1\n", "b = 2 ** -1\n");
        same("r = {type: 1, \"full name\": 2}\n");
        gives("fn f(1 .. 9 | - 2.5 .. -1) = 1\nfn f(\"Mr \"+name) = name\n", "fn f(1..9 | -2.5..-1) = 1\nfn f(\"Mr \" + name) = name\n");
    }

    #[test]
    fn indentation_and_blocks() {
        gives("fn f(x)\n        y = x\n    y\n  end\n", "fn f(x)\n  y = x\n  y\nend\n");
        gives("fn f(x) = match x\n0 -> 1\n    _ -> 2 end\n", "fn f(x) = match x\n  0 -> 1\n  _ -> 2\nend\n");
        gives("x = do a = 1; a + 1 end\n", "x = do\n  a = 1\n  a + 1\nend\n");
        same("xs |> each(fn(x)\n  print(x)\nend)\n");
        same("students = [\n  {name: \"Ada\"},\n  {name: \"Alan\"},\n]\n");
        gives(
            "students = [\n  {name: \"Ada\"},\n    {name: \"Alan\"}\n  ]\n",
            "students = [\n  {name: \"Ada\"},\n  {name: \"Alan\"},\n]\n",
        );
    }

    #[test]
    fn calls_without_parentheses() {
        same("puts \"Hello\"\nprint x, y\nassert_eq total, 42\nputs -1\nputs [1, 2]\n");
        same("xs.each do |x| puts x end\n");
        gives("puts\"a\"\nprint  x ,y\n", "puts \"a\"\nprint x, y\n");
        // The author's choice stays: with or without parentheses.
        same("puts(\"a\")\nputs \"a\"\n");
        // Values: an index and a subtraction, written as such.
        gives("xs = [1]\nputs xs [0]\nxs [0]\n", "xs = [1]\nputs xs[0]\nxs[0]\n");
        gives("x = 1\nx -1\n", "x = 1\nx - 1\n");
        gives("each xs do |v|\nprint v\n  end\n", "each xs do |v|\n  print v\nend\n");
    }

    #[test]
    fn conditions() {
        same("fn mention(avg) =\n  if avg >= 18 then \"excellent\"\n  else if avg >= 15 then \"very good\"\n  else \"keep going\"\n");
        gives("x = if a then 1\nelse 2\n", "x = if a then 1\n  else 2\n");
        same("x = if a then\n  1\nelse if b then\n  2\nelse\n  3\n");
        same("x = if a\n  then 1\n  else 2\n");
    }

    #[test]
    fn pipelines_get_one_step_per_line() {
        same("1..15 |> map(fizzbuzz) |> each(print)\n");
        gives("xs |> map(f)\n  |> sum\n", "xs\n  |> map(f)\n  |> sum\n");
        gives("total = xs |>\n  map(f) |> # double\n  sum\n", "total = xs\n  |> map(f)  # double\n  |> sum\n");
        same("fn count_words(text) =\n  split(text)\n    |> group_by(fn(w) = w)\n    |> entries\n");
        gives("m = match xs\n|> len\n0 -> 1\nend\n", "m = match xs\n    |> len\n  0 -> 1\nend\n");
    }

    #[test]
    fn types() {
        same("type Tree = Leaf | Node(left, value, right)\n");
        gives("type T = | A | B\n", "type T = A | B\n");
        gives("type Shape = Circle(r)\n  | Rect(w, h)\n", "type Shape =\n  | Circle(r)\n  | Rect(w, h)\n");
        gives("type Shape =\nCircle(r) |\nRect(w, h)\n", "type Shape =\n  | Circle(r)\n  | Rect(w, h)\n");
    }

    #[test]
    fn comments_are_kept() {
        same("# header\n\n## Doubles.\nfn double(x) = x * 2  # the result\n");
        gives("x = 1 # one\ntotal = x + 2   # three\n", "x = 1          # one\ntotal = x + 2  # three\n");
        same("fn f(x)\n  # before\n  x\n  # after\nend\n");
        same("xs = [\n  1,  # one\n  # two is next\n  2,\n]\n");
        gives("x = 1 # no new line at the end", "x = 1  # no new line at the end\n");
    }

    #[test]
    fn blank_lines() {
        gives("\n\nx = 1\n\n\n\ny = 2\n\n\n", "x = 1\n\ny = 2\n");
        gives("fn f()\n\n  1\n\nend\n", "fn f()\n  1\nend\n");
        gives("x = 1\nfn f()\n  1\nend\ny = 2\n", "x = 1\n\nfn f()\n  1\nend\n\ny = 2\n");
        gives("x = 1\n# about f\nfn f()\n  1\nend\n", "x = 1\n\n# about f\nfn f()\n  1\nend\n");
        same("fn fib(0) = 0\nfn fib(1) = 1\nfn fib(n) =\n  fib(n - 1) + fib(n - 2)\n");
        same("fn fast_fib(n) = step(n, 0, 1)\nfn step(0, a, _) = a\n");
    }

    #[test]
    fn line_ends_and_odd_text() {
        gives("x = 1\r\ny = 2\r\n", "x = 1\r\ny = 2\r\n");
        gives("\u{FEFF}x  =  1", "x = 1\n");
        same("t = \"a  \n  b\"\n");
        same("");
        gives("\n\n", "");
        gives("# only a comment", "# only a comment\n");
    }

    #[test]
    fn the_tree_ignores_positions_only() {
        use super::syntax_tree;
        assert_eq!(syntax_tree("x = 1 + 2"), syntax_tree("x  =\n  1 + 2"));
        assert_ne!(syntax_tree("x = 1 + 2"), syntax_tree("x = 1 - 2"));
        assert_ne!(syntax_tree("## a\nfn f() = 1"), syntax_tree("## b\nfn f() = 1"));
        assert!(!syntax_tree("x = [1]").unwrap().contains("start"));
        assert!(syntax_tree("t = \"Span \\{ file: 1 \\}\"").unwrap().contains("Span { file: 1 }"));
    }

    #[test]
    fn syntax_errors_are_refused() {
        let d = format("x = (1 +\n").unwrap_err();
        assert_eq!(d[0].code, "E001");
        assert!(format("print(\"oops)").is_err());
    }
}
