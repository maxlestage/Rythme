//! Turns tokens into a syntax tree.
//!
//! New lines end statements, except inside `()`, `[]` and `{}`, or when the
//! next line starts with an operator such as `|>` or `.` (to continue a chain).

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::ast::*;
use crate::bigint::{self, BigInt};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::lexer::{self, describe, Lexed, StrPart, Tok, Token};
use crate::source::Span;
use crate::symbol::Sym;
use crate::value::Value;

pub fn parse(lexed: Lexed) -> Fallible<Vec<Item>> {
    let preds = Rc::new(predicates(&lexed.tokens));
    let values = Rc::new(value_names(&lexed.tokens));
    let mut p = Parser {
        toks: lexed.tokens,
        pos: 0,
        sig: vec![true],
        docs: lexed.docs,
        last: Span::default(),
        depth: 0,
        in_text: false,
        preds,
        values,
        no_do: false,
        commands: HashSet::new(),
    };
    p.program()
}

/// The names of the functions defined with a `?` at the end in this code:
/// `fn adult?(person) = ...`.
pub(crate) fn predicates(toks: &[Token]) -> Vec<Sym> {
    let mut out = Vec::new();
    for w in toks.windows(3) {
        if let [Token { tok: Tok::Fn, .. }, name @ Token { tok: Tok::Ident(s), .. }, q] = w {
            if q.tok == Tok::Question && q.span.start == name.span.end {
                out.push(Sym::new(&format!("{s}?")));
            }
        }
    }
    out
}

/// `even?` written at `i`: a name directly followed by `?` is one name, the
/// name of a function that answers a question, when `(` follows directly
/// (`even?(4)`) or when a function of that name exists: a built-in one, or
/// one defined in the code (`preds`). Otherwise the `?` is the operator that
/// stops on `None` or `Err` (`value?`).
pub(crate) fn predicate_at(toks: &[Token], i: usize, preds: &[Sym]) -> Option<Sym> {
    let (Tok::Ident(s), Some(q)) = (&toks[i].tok, toks.get(i + 1)) else { return None };
    if q.tok != Tok::Question || q.span.start != toks[i].span.end {
        return None;
    }
    let name = Sym::new(&format!("{s}?"));
    let call = toks.get(i + 2).is_some_and(|t| t.tok == Tok::LParen && t.span.start == q.span.end);
    (call || preds.contains(&name) || crate::builtins::lookup(&name.name()).is_some()).then_some(name)
}

/// Does the name `unless` at `i` start `unless condition then a else b`? It
/// does when the start of a value follows on the same line: a name, a number,
/// a text, a symbol, `not`..., or a bracket after a space (`unless(x)` and
/// `unless[0]`, without a space, use a value named `unless`).
pub(crate) fn unless_at(toks: &[Token], i: usize) -> bool {
    if !matches!(&toks[i].tok, Tok::Ident(s) if &*s.name() == "unless") {
        return false;
    }
    let Some(next) = toks.get(i + 1) else { return false };
    match next.tok {
        Tok::Int(_)
        | Tok::Float(_)
        | Tok::Str(_)
        | Tok::Ident(_)
        | Tok::Ctor(_)
        | Tok::Symbol(_)
        | Tok::True
        | Tok::False
        | Tok::Not
        | Tok::If
        | Tok::Match
        | Tok::Fn => true,
        Tok::LParen | Tok::LBracket => next.span.start > toks[i].span.end,
        _ => false,
    }
}

/// The names this code gives to values: defined with `=` (`x = 1`,
/// `[a, b] = xs`, `f = fn(x) = x`), parameters of functions and blocks, and
/// the names of the patterns of `match` (before `->`). The names on a line
/// before `=` or `->` all count, and those inside texts too: a few more
/// names than needed, never fewer. A call without parentheses whose first
/// argument starts with `[` or `-` needs a name that is not among them (see
/// [`command_at`]).
pub(crate) fn value_names(toks: &[Token]) -> HashSet<Sym> {
    let mut out = HashSet::new();
    names_of_values(toks, &mut out);
    out
}

fn names_of_values(toks: &[Token], out: &mut HashSet<Sym>) {
    for (i, t) in toks.iter().enumerate() {
        match &t.tok {
            // `pattern = value` and `pattern -> body`: the names before it, on its line.
            Tok::Eq | Tok::Arrow => {
                let mut depth = 0usize;
                let mut found = Vec::new();
                let mut j = i;
                while j > 0 {
                    j -= 1;
                    match &toks[j].tok {
                        Tok::RParen | Tok::RBracket | Tok::RBrace => depth += 1,
                        Tok::LParen | Tok::LBracket | Tok::LBrace if depth == 0 => break,
                        Tok::LParen | Tok::LBracket | Tok::LBrace => depth -= 1,
                        Tok::Newline if depth == 0 => break,
                        Tok::Eq | Tok::Arrow | Tok::Do | Tok::Then | Tok::Else => break,
                        // `fn name(x) = ...` and `fn(x) = ...`: `name` is a
                        // function, and the parameters are found below.
                        Tok::Fn => {
                            found.retain(|&k| k != j + 1);
                            break;
                        }
                        Tok::Ident(_) => found.push(j),
                        _ => {}
                    }
                }
                out.extend(found.into_iter().filter_map(|k| match &toks[k].tok {
                    Tok::Ident(s) => Some(*s),
                    _ => None,
                }));
            }
            // `fn name(params)` and `fn(params)`.
            Tok::Fn => {
                let mut j = i + 1;
                while matches!(toks.get(j).map(|t| &t.tok), Some(Tok::Ident(_) | Tok::Question)) {
                    j += 1;
                }
                if !matches!(toks.get(j).map(|t| &t.tok), Some(Tok::LParen)) {
                    continue;
                }
                let mut depth = 0usize;
                while let Some(t) = toks.get(j) {
                    match &t.tok {
                        Tok::LParen | Tok::LBracket | Tok::LBrace => depth += 1,
                        Tok::RParen | Tok::RBracket | Tok::RBrace => {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                break;
                            }
                        }
                        Tok::Ident(s) => {
                            out.insert(*s);
                        }
                        Tok::Eof => break,
                        _ => {}
                    }
                    j += 1;
                }
            }
            // `do |x, y|` and `{ |x| ... }`.
            Tok::Do | Tok::LBrace if matches!(toks.get(i + 1).map(|t| &t.tok), Some(Tok::Bar)) => {
                for t in toks[i + 2..].iter().take_while(|t| !matches!(t.tok, Tok::Bar | Tok::Newline | Tok::Eof)) {
                    if let Tok::Ident(s) = &t.tok {
                        out.insert(*s);
                    }
                }
            }
            Tok::Str(parts) => {
                for part in parts {
                    if let StrPart::Code(code) = part {
                        names_of_values(code, out);
                    }
                }
            }
            _ => {}
        }
    }
}

/// A call without parentheses, as in Ruby: `puts "Hello"`, `print x, y`,
/// `assert_eq total, 42`. It starts a statement: a name (`even?` too), then,
/// on the same line, the start of a value: a number, a text, a symbol, a
/// name, a constructor, `true`, `false`, `not`, `if`, `match` or `fn`. A `[` after a space
/// (`puts [1, 2]`) or a `-` after a space and right before a number or a
/// name (`puts -1`) starts an argument too, but only when the code gives no
/// value this name (see [`value_names`]): like in Ruby, `x [0]` and `x -1`
/// keep their meaning, `x[0]` and `x - 1`, when `x` is a value. A `(` right
/// after the name, or after a space, is the usual call. Gives the index of
/// the first token of the arguments.
pub(crate) fn command_at(toks: &[Token], i: usize, preds: &[Sym], values: &HashSet<Sym>) -> Option<usize> {
    let Tok::Ident(s) = &toks[i].tok else { return None };
    if &*s.name() == "_" || unless_at(toks, i) {
        return None;
    }
    let (name, j) = match predicate_at(toks, i, preds) {
        Some(name) => (name, i + 2),
        None => (*s, i + 1),
    };
    let next = toks.get(j)?;
    let spaced = next.span.start > toks[j - 1].span.end;
    let free = || !values.contains(&name);
    let ok = match &next.tok {
        Tok::Int(_)
        | Tok::Big(_)
        | Tok::Float(_)
        | Tok::Str(_)
        | Tok::Symbol(_)
        | Tok::Ident(_)
        | Tok::Ctor(_)
        | Tok::True
        | Tok::False
        | Tok::Not
        | Tok::If
        | Tok::Match
        | Tok::Fn => true,
        Tok::LBracket => spaced && free(),
        Tok::Minus => {
            spaced
                && free()
                && toks.get(j + 1).is_some_and(|n| {
                    n.span.start == next.span.end && matches!(n.tok, Tok::Int(_) | Tok::Big(_) | Tok::Float(_) | Tok::Ident(_))
                })
        }
        _ => false,
    };
    ok.then_some(j)
}

struct Parser {
    toks: Vec<Token>,
    pos: usize,
    /// Are new lines significant here? (no inside brackets)
    sig: Vec<bool>,
    docs: HashMap<usize, String>,
    /// Span of the last consumed token.
    last: Span,
    /// How many expressions are being read inside each other.
    depth: usize,
    /// Reading the `{...}` of a text.
    in_text: bool,
    /// The functions defined with a name ending with `?` (see `predicates`).
    preds: Rc<Vec<Sym>>,
    /// The names given to values (see `value_names`).
    values: Rc<HashSet<Sym>>,
    /// Reading the arguments of a call without parentheses: a `do` block
    /// that follows is given to that call, not to its last argument.
    no_do: bool,
    /// The places of the calls without parentheses read so far.
    commands: HashSet<Span>,
}

/// Deeper nesting is surely a mistake, and would exhaust the stack.
const MAX_NESTING: usize = 256;

/// The deepest expression accepted (see `Expr::depth`): a chain like
/// `a + b + c + ...` nests one level per operator, without brackets.
const MAX_DEPTH: u32 = 1000;

/// Tokens that may start a line and still continue the previous expression.
pub(crate) fn is_continuation(t: &Tok) -> bool {
    matches!(
        t,
        Tok::PipeGt
            | Tok::Dot
            | Tok::Plus
            | Tok::Star
            | Tok::Slash
            | Tok::SlashSlash
            | Tok::Percent
            | Tok::StarStar
            | Tok::EqEq
            | Tok::NotEq
            | Tok::Lt
            | Tok::Le
            | Tok::Gt
            | Tok::Ge
            | Tok::And
            | Tok::Or
            | Tok::DotDot
    )
}

fn starts_expression(t: &Tok) -> bool {
    matches!(
        t,
        Tok::Int(_)
            | Tok::Big(_)
            | Tok::Float(_)
            | Tok::Str(_)
            | Tok::Ident(_)
            | Tok::Ctor(_)
            | Tok::True
            | Tok::False
            | Tok::LBracket
            | Tok::LBrace
            | Tok::LParen
            | Tok::Fn
            | Tok::Symbol(_)
    )
}

fn merge_fn(prev: &mut Rc<FnDef>, new: &mut Option<Rc<FnDef>>) -> Fallible<()> {
    let Some(n) = new.as_ref() else { return Ok(()) };
    if prev.name.is_none() || prev.name != n.name {
        return Ok(());
    }
    if prev.arity != n.arity {
        let name = n.name.unwrap();
        return Err(Diagnostic::error(
            "E004",
            t!(
                "all the definitions of `{name}` must have the same number of parameters",
                "toutes les définitions de `{name}` doivent avoir le même nombre de paramètres"
            ),
        )
        .at(n.span)
        .label(t!("{} parameter(s) here", "{} paramètre(s) ici", n.arity))
        .secondary(prev.span, t!("{} parameter(s) here", "{} paramètre(s) ici", prev.arity))
        .boxed());
    }
    let n = Rc::try_unwrap(new.take().unwrap()).expect("fresh definition");
    let p = Rc::get_mut(prev).expect("fresh definition");
    p.clauses.extend(n.clauses);
    if p.doc.is_none() {
        p.doc = n.doc;
    }
    Ok(())
}

/// Adds a statement to a block; consecutive `fn` with the same name become one function.
fn push_stmt(stmts: &mut Vec<Stmt>, s: Stmt) -> Fallible<()> {
    let Stmt::Fn(new) = s else {
        stmts.push(s);
        return Ok(());
    };
    let mut new = Some(new);
    if let Some(Stmt::Fn(prev)) = stmts.last_mut() {
        merge_fn(prev, &mut new)?;
    }
    if let Some(new) = new {
        stmts.push(Stmt::Fn(new));
    }
    Ok(())
}

fn push_item(items: &mut Vec<Item>, it: Item) -> Fallible<()> {
    let Item::Stmt(Stmt::Fn(new)) = it else {
        items.push(it);
        return Ok(());
    };
    let mut new = Some(new);
    if let Some(Item::Stmt(Stmt::Fn(prev))) = items.last_mut() {
        merge_fn(prev, &mut new)?;
    }
    if let Some(new) = new {
        items.push(Item::Stmt(Stmt::Fn(new)));
    }
    Ok(())
}

impl Parser {
    // ----- token access -------------------------------------------------

    fn significant(&self) -> bool {
        *self.sig.last().unwrap_or(&true)
    }

    fn skip_soft(&mut self) {
        if !self.significant() {
            while matches!(self.toks[self.pos].tok, Tok::Newline) {
                self.pos += 1;
            }
        }
    }

    fn peek(&mut self) -> &Tok {
        self.skip_soft();
        &self.toks[self.pos].tok
    }

    fn peek_token(&mut self) -> Token {
        self.skip_soft();
        self.toks[self.pos].clone()
    }

    fn raw(&self) -> &Tok {
        &self.toks[self.pos].tok
    }

    fn advance(&mut self) -> Token {
        self.skip_soft();
        let t = self.toks[self.pos].clone();
        if t.tok != Tok::Eof {
            self.pos += 1;
        }
        self.last = t.span;
        t
    }

    fn skip_newlines(&mut self) {
        while matches!(self.toks[self.pos].tok, Tok::Newline) {
            self.pos += 1;
        }
    }

    fn past_newlines(&self) -> &Tok {
        let mut i = self.pos;
        while matches!(self.toks[i].tok, Tok::Newline) {
            i += 1;
        }
        &self.toks[i].tok
    }

    /// The next token, looking past new lines only if the next line continues
    /// the current expression (it starts with `|>`, `.`, `+`...).
    fn peek_cont(&mut self) -> &Tok {
        self.skip_soft();
        let mut i = self.pos;
        while matches!(self.toks[i].tok, Tok::Newline) {
            i += 1;
        }
        if i != self.pos && !is_continuation(&self.toks[i].tok) {
            return &self.toks[self.pos].tok;
        }
        &self.toks[i].tok
    }

    fn take_cont(&mut self) -> Token {
        if is_continuation(self.past_newlines()) {
            self.skip_newlines();
        }
        self.advance()
    }

    fn eat(&mut self, t: &Tok) -> bool {
        if self.peek() == t {
            self.advance();
            true
        } else {
            false
        }
    }

    fn nested<T>(&mut self, sig: bool, f: impl FnOnce(&mut Self) -> Fallible<T>) -> Fallible<T> {
        self.sig.push(sig);
        let no_do = std::mem::replace(&mut self.no_do, false);
        let r = f(self);
        self.no_do = no_do;
        self.sig.pop();
        r
    }

    // ----- errors -------------------------------------------------------

    fn unexpected(&mut self, expected: String) -> Box<Diagnostic> {
        let tok = self.peek_token();
        self.unexpected_token(&tok, expected)
    }

    fn unexpected_token(&self, tok: &Token, expected: String) -> Box<Diagnostic> {
        let found = describe(&tok.tok);
        let mut d = Diagnostic::error("E001", t!("expected {expected}, found {found}", "{expected} attendu, mais trouvé {found}"))
            .at(tok.span)
            .label(t!("expected {expected} here", "{expected} attendu ici"));
        d = self.hint_for(d, &tok.tok);
        if tok.tok == Tok::Eof {
            d = d.incomplete();
        }
        d.boxed()
    }

    fn hint_for(&self, d: Diagnostic, tok: &Tok) -> Diagnostic {
        match tok {
            Tok::Eq => d.help(t!("to compare two values, use `==`", "pour comparer deux valeurs, utilisez `==`")),
            Tok::FatArrow => d.help(t!(
                "use `->` in `match` cases; a dict is built with `dict([[key, value], ...])`",
                "utilisez `->` dans les cas d'un `match` ; un dictionnaire se construit avec `dict([[clé, valeur], ...])`"
            )),
            Tok::AmpAmp => d.help(t!("use `and` to combine conditions", "utilisez `and` pour combiner des conditions")),
            Tok::BarBar => d.help(t!("use `or` to combine conditions", "utilisez `or` pour combiner des conditions")),
            Tok::Bang => d.help(t!("use `not` to negate a condition: `not done`", "utilisez `not` pour inverser une condition : `not fini`")),
            Tok::PlusEq | Tok::MinusEq | Tok::StarEq | Tok::SlashEq => d.help(immutable_help()),
            Tok::Reserved(s) => d.help(lexer::reserved_help(&s.name())),
            // Inside `"{...}"`, braces and `:` are rather meant literally (see `text`).
            Tok::LBrace | Tok::Colon if self.in_text => d,
            Tok::LBrace => d.help(t!(
                "Rythme does not use braces for blocks: write `do` ... `end`, or put a function body on the following lines and finish with `end`",
                "Rythme n'utilise pas d'accolades pour les blocs : écrivez `do` ... `end`, ou mettez le corps d'une fonction sur les lignes suivantes et terminez par `end`"
            )),
            Tok::Colon => d.help(t!(
                "Rythme does not use `:` to start a block; see `rythme help syntax`",
                "Rythme n'utilise pas `:` pour commencer un bloc ; voir `rythme help syntax`"
            )),
            _ => d,
        }
    }

    fn reserved_error(&self, s: Sym, span: Span) -> Box<Diagnostic> {
        let word = s.name();
        Diagnostic::error("E001", t!("`{word}` is not used in Rythme", "`{word}` n'est pas utilisé en Rythme"))
            .at(span)
            .label(t!("comes from another language", "vient d'un autre langage"))
            .help(lexer::reserved_help(&word))
            .boxed()
    }

    // ----- program and statements --------------------------------------

    fn program(&mut self) -> Fallible<Vec<Item>> {
        let mut items = Vec::new();
        loop {
            self.skip_newlines();
            if matches!(self.raw(), Tok::Eof) {
                return Ok(items);
            }
            let item = match self.peek() {
                Tok::Type => self.type_def()?,
                Tok::Test => self.test_def()?,
                Tok::Use => self.use_def()?,
                _ => Item::Stmt(self.stmt()?),
            };
            let stmt = if let Item::Stmt(s) = &item { Some(s) } else { None };
            self.stmt_end(stmt)?;
            push_item(&mut items, item)?;
        }
    }

    fn stmt_end(&mut self, stmt: Option<&Stmt>) -> Fallible<()> {
        if matches!(self.raw(), Tok::Newline | Tok::Eof | Tok::End) {
            return Ok(());
        }
        let tok = self.toks[self.pos].clone();
        if let Tok::Reserved(s) = tok.tok {
            return Err(self.reserved_error(s, tok.span));
        }
        let found = describe(&tok.tok);
        let mut d = Diagnostic::error("E001", t!("unexpected here: {found}", "inattendu ici : {found}"))
            .at(tok.span)
            .label(t!("the statement should end before this", "l'instruction devrait se terminer avant ceci"));
        let callee = match stmt {
            Some(Stmt::Expr(Expr { kind: ExprKind::Var(s), .. })) => Some(s.name().to_string()),
            _ => None,
        };
        let command = matches!(stmt, Some(Stmt::Expr(e)) if self.commands.contains(&e.span));
        match (&callee, &tok.tok) {
            (None, t) if command && starts_expression(t) => {
                d = d.help(t!(
                    "the arguments of a call without parentheses are separated by commas: `print a, b`",
                    "les arguments d'un appel sans parenthèses sont séparés par des virgules : `print a, b`"
                ));
            }
            (Some(name), t) if starts_expression(t) => {
                d = d.help(t!(
                    "to call a function, use parentheses: `{name}(...)`",
                    "pour appeler une fonction, utilisez des parenthèses : `{name}(...)`"
                ));
            }
            (_, Tok::LBrace) | (_, Tok::Colon) | (_, Tok::PlusEq) | (_, Tok::MinusEq) | (_, Tok::StarEq) | (_, Tok::SlashEq) => {
                d = self.hint_for(d, &tok.tok);
            }
            (_, Tok::Do) => {
                d = d.help(t!(
                    "a block `do |x| ... end` is given to a call, on its line: `names.each do |name| print(name) end`",
                    "un bloc `do |x| ... end` est donné à un appel, sur sa ligne : `noms.each do |nom| print(nom) end`"
                ));
            }
            (_, Tok::Else) => {
                d = d.help(t!(
                    "`else` must follow `if condition then value` (an `if` branch holds one expression; use `do ... end` for several lines)",
                    "`else` doit suivre `if condition then valeur` (une branche de `if` contient une seule expression ; utilisez `do ... end` pour plusieurs lignes)"
                ));
            }
            _ => {
                d = self.hint_for(d, &tok.tok);
                if d.helps.is_empty() {
                    d = d.help(t!("put each statement on its own line", "mettez chaque instruction sur sa propre ligne"));
                }
            }
        }
        Err(d.boxed())
    }

    fn stmt(&mut self) -> Fallible<Stmt> {
        let tok = self.peek_token();
        match tok.tok {
            Tok::Fn if matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Ident(_))) => {
                return Ok(Stmt::Fn(Rc::new(self.fn_def(true)?)));
            }
            Tok::Type | Tok::Test | Tok::Use => {
                let word = lexer::keyword_text(&tok.tok).unwrap_or("");
                return Err(Diagnostic::error(
                    "E001",
                    t!(
                        "`{word}` can only be used at the top level of a file",
                        "`{word}` ne peut être utilisé qu'au premier niveau d'un fichier"
                    ),
                )
                .at(tok.span)
                .help(t!("move it outside of the function or block", "déplacez-le en dehors de la fonction ou du bloc"))
                .boxed());
            }
            Tok::Reserved(s) => return Err(self.reserved_error(s, tok.span)),
            _ => {}
        }
        if let Some(first) = command_at(&self.toks, self.pos, &self.preds, &self.values) {
            return Ok(Stmt::Expr(self.command(first)?));
        }
        if self.pattern_before_eq() {
            // Read as a pattern, so that a definition accepts what `match` accepts.
            let pat = self.pattern().map_err(|mut d| {
                if !d.helps.is_empty() {
                    return d;
                }
                d.helps.push(t!(
                    "before `=` comes a name or a pattern, like `[first, ..rest]`, `{{name, ..}}` or `Some(x)`",
                    "avant `=` vient un nom ou un motif, comme `[premier, ..reste]`, `{{nom, ..}}` ou `Some(x)`"
                ));
                d
            })?;
            if !matches!(self.raw(), Tok::Eq) {
                return Err(self.unexpected(t!("`=`", "`=`")));
            }
            self.advance();
            self.skip_newlines();
            let value = self.expr()?;
            return Ok(Stmt::Let(pat, value));
        }
        let e = self.expr()?;
        match self.raw() {
            Tok::Eq => {
                let eq = self.advance();
                let pat = self.expr_to_pattern(e, eq.span)?;
                self.skip_newlines();
                let value = self.expr()?;
                Ok(Stmt::Let(pat, value))
            }
            Tok::PlusEq | Tok::MinusEq | Tok::StarEq | Tok::SlashEq => {
                let t = self.advance();
                Err(Diagnostic::error(
                    "E011",
                    t!("values cannot be changed in Rythme", "les valeurs ne peuvent pas être modifiées en Rythme"),
                )
                .at(t.span)
                .help(immutable_help())
                .boxed())
            }
            _ => Ok(Stmt::Expr(e)),
        }
    }

    /// A call without parentheses (see [`command_at`]): `print x, y`, then
    /// a `do` block that is given to it, as its last argument.
    fn command(&mut self, first: usize) -> Fallible<Expr> {
        self.deeper(|p| {
            let callee = p.primary()?;
            debug_assert_eq!(p.pos, first);
            let start = callee.span;
            let no_do = std::mem::replace(&mut p.no_do, true);
            let args = p.command_args();
            p.no_do = no_do;
            let args = args?;
            let span = start.to(p.last);
            let mut e = p.limit(Expr::new(ExprKind::Call(Box::new(callee), args), span), span)?;
            if matches!(p.raw(), Tok::Do) {
                let block = p.block_arg()?;
                e = with_block(e, block);
            }
            p.commands.insert(e.span);
            Ok(e)
        })
    }

    /// `a, b, c`: the arguments of a call without parentheses, up to the end
    /// of the line (a new line may follow a comma).
    fn command_args(&mut self) -> Fallible<Vec<Expr>> {
        let mut args = vec![self.expr()?];
        while matches!(self.raw(), Tok::Comma) {
            self.advance();
            self.skip_newlines();
            args.push(self.expr()?);
        }
        Ok(args)
    }

    /// Does a definition with a bracketed pattern start here: `[first, ..] = list`,
    /// `{name, ..} = person`, `Some(x) = value`?
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
        matches!(self.toks[i + 1].tok, Tok::Eq)
    }

    /// `{name, ..others}`: the other fields cannot be given a name.
    fn named_record_rest(&self, span: Span) -> Box<Diagnostic> {
        Diagnostic::error(
            "E001",
            t!("a record pattern cannot give a name to the other fields", "un motif d'enregistrement ne peut pas nommer les autres champs"),
        )
        .at(span)
        .help(t!(
            "write `{{name, ..}}` to ignore the other fields; for a record without some fields, use `remove(record, \"name\")`",
            "écrivez `{{nom, ..}}` pour ignorer les autres champs ; pour un enregistrement sans certains champs, utilisez `remove(enregistrement, \"nom\")`"
        ))
        .boxed()
    }

    fn expr_to_pattern(&self, e: Expr, eq: Span) -> Fallible<Pattern> {
        let span = e.span;
        let kind = match e.kind {
            ExprKind::Var(s) if &*s.name() == "_" => PatKind::Wildcard,
            // `ok? = 1`: the `?` is read as the operator, unless a function `ok?` exists.
            ExprKind::Try(x) if matches!(x.kind, ExprKind::Var(_)) && x.span.end + 1 == span.end => {
                let ExprKind::Var(s) = x.kind else { unreachable!() };
                let s = format!("{s}?");
                return Err(Diagnostic::error(
                    "E001",
                    t!("only a function can have a name that ends with `?`", "seule une fonction peut avoir un nom qui finit par `?`"),
                )
                .at(span)
                .help(t!(
                    "define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`",
                    "définissez-la avec `fn` : `fn {s}(x) = ...` ; pour une valeur, utilisez un nom sans `?`"
                ))
                .boxed());
            }
            ExprKind::Var(s) if s.name().ends_with('?') => {
                return Err(Diagnostic::error(
                    "E001",
                    t!("only a function can have a name that ends with `?`", "seule une fonction peut avoir un nom qui finit par `?`"),
                )
                .at(span)
                .help(t!(
                    "define it with `fn`: `fn {s}(x) = ...`; for a value, use a name without `?`",
                    "définissez-la avec `fn` : `fn {s}(x) = ...` ; pour une valeur, utilisez un nom sans `?`"
                ))
                .boxed());
            }
            ExprKind::Var(s) => PatKind::Bind(s),
            ExprKind::Symbol(s) => PatKind::Symbol(s),
            ExprKind::Ctor(s) => PatKind::Ctor(s, Vec::new()),
            ExprKind::Call(callee, args) if matches!(callee.kind, ExprKind::Ctor(_)) => {
                let ExprKind::Ctor(s) = callee.kind else { unreachable!() };
                let pats = args.into_iter().map(|a| self.expr_to_pattern(a, eq)).collect::<Fallible<Vec<_>>>()?;
                PatKind::Ctor(s, pats)
            }
            ExprKind::List(items) => {
                let mut pats = Vec::new();
                let mut rest = None;
                let n = items.len();
                for (i, item) in items.into_iter().enumerate() {
                    match item {
                        ListItem::One(x) => pats.push(self.expr_to_pattern(x, eq)?),
                        ListItem::Spread(x) => {
                            let ok = i + 1 == n;
                            match x.kind {
                                ExprKind::Var(s) if ok => {
                                    rest = Some(if &*s.name() == "_" { None } else { Some((s, x.span)) });
                                }
                                _ => {
                                    return Err(Diagnostic::error(
                                        "E001",
                                        t!("invalid `..` in this pattern", "`..` invalide dans ce motif"),
                                    )
                                    .at(x.span)
                                    .help(t!(
                                        "`..name` must be last: `[first, ..rest]`",
                                        "`..nom` doit être en dernier : `[premier, ..reste]`"
                                    ))
                                    .boxed())
                                }
                            }
                        }
                    }
                }
                PatKind::List(pats, rest)
            }
            ExprKind::Record(items) => {
                let mut fields = Vec::new();
                for item in items {
                    match item {
                        RecordItem::Field(k, _, v) => fields.push((k, self.expr_to_pattern(v, eq)?)),
                        RecordItem::Spread(x) => return Err(self.named_record_rest(x.span)),
                    }
                }
                PatKind::Record(fields)
            }
            ExprKind::Range(lo, hi) if expr_num(&lo).is_some() && expr_num(&hi).is_some() => {
                PatKind::Range(expr_num(&lo).unwrap(), expr_num(&hi).unwrap())
            }
            ExprKind::Binary(BinOp::Add, l, r, _) if matches!(l.kind, ExprKind::Text(_)) && matches!(r.kind, ExprKind::Var(_)) => {
                let ExprKind::Text(start) = l.kind else { unreachable!() };
                PatKind::Prefix(start, Box::new(self.expr_to_pattern(*r, eq)?))
            }
            ExprKind::Int(n) => PatKind::Int(n),
            ExprKind::Big(n) => PatKind::Big(n),
            ExprKind::Float(f) => PatKind::Float(f),
            ExprKind::Text(t) => PatKind::Text(t),
            ExprKind::Bool(b) => PatKind::Bool(b),
            ExprKind::Unit => PatKind::Unit,
            other => {
                let mut d = Diagnostic::error("E001", t!("cannot give a value to this", "impossible de donner une valeur à ceci"))
                    .at(span)
                    .label(t!("this is not a name", "ceci n'est pas un nom"))
                    .secondary(eq, t!("`=` gives a name to a value", "`=` donne un nom à une valeur"));
                d = match other {
                    ExprKind::Call(callee, _) if matches!(callee.kind, ExprKind::Var(_)) => d.help(t!(
                        "to define a function, write `fn name(x) = ...`",
                        "pour définir une fonction, écrivez `fn nom(x) = ...`"
                    )),
                    ExprKind::Field(_, name, _) => d.help(t!(
                        "records cannot be changed; build a new one: `new = {{..old, {name}: value}}`",
                        "les enregistrements ne peuvent pas être modifiés ; construisez-en un nouveau : `nouveau = {{..ancien, {name}: valeur}}`"
                    )),
                    ExprKind::Index(..) => d.help(t!(
                        "values cannot be changed; build a new one: `set_at(list, i, x)` for a list, `put(record, name, x)` for a record",
                        "les valeurs ne peuvent pas être modifiées ; construisez-en une nouvelle : `set_at(liste, i, x)` pour une liste, `put(enreg, nom, x)` pour un enregistrement"
                    )),
                    _ => d.help(t!("to compare two values, use `==`", "pour comparer deux valeurs, utilisez `==`")),
                };
                return Err(d.boxed());
            }
        };
        Ok(Pattern { kind, span })
    }

    fn block(&mut self, opener: Span, what: &str) -> Fallible<Vec<Stmt>> {
        self.deeper(|p| p.block_inner(opener, what))
    }

    fn block_inner(&mut self, opener: Span, what: &str) -> Fallible<Vec<Stmt>> {
        self.nested(true, |p| {
            let mut stmts = Vec::new();
            loop {
                p.skip_newlines();
                match p.raw() {
                    Tok::End => {
                        p.advance();
                        return Ok(stmts);
                    }
                    Tok::Eof => {
                        let span = p.toks[p.pos].span;
                        return Err(missing_end(span, opener, what));
                    }
                    _ => {}
                }
                let s = p.stmt()?;
                p.stmt_end(Some(&s))?;
                push_stmt(&mut stmts, s)?;
            }
        })
    }

    // ----- definitions -------------------------------------------------

    fn fn_def(&mut self, named: bool) -> Fallible<FnDef> {
        self.skip_soft();
        let doc = self.docs.get(&self.pos).map(|d| Rc::from(d.as_str()));
        let fn_tok = self.advance();
        let (name, name_span) = if named {
            let t = self.advance();
            match t.tok {
                // `fn adult?(person)`: a name may end with `?`.
                Tok::Ident(s) if self.raw() == &Tok::Question && self.toks[self.pos].span.start == t.span.end => {
                    let q = self.advance();
                    (Some(Sym::new(&format!("{s}?"))), t.span.to(q.span))
                }
                Tok::Ident(s) => (Some(s), t.span),
                _ => return Err(self.unexpected_token(&t, t!("a function name", "un nom de fonction"))),
            }
        } else {
            (None, fn_tok.span)
        };
        if !matches!(self.peek(), Tok::LParen) {
            let mut d = self.unexpected(t!("`(` and the parameters", "`(` et les paramètres"));
            // `fn greet = ...`: the `=` is right, only the parentheses are missing.
            if matches!(self.peek(), Tok::Eq) {
                d.helps.clear();
            }
            d.helps.push(t!(
                "even without parameters, write the parentheses: `fn greet() = print(\"hi\")`",
                "même sans paramètres, écrivez les parenthèses : `fn saluer() = print(\"salut\")`"
            ));
            return Err(d);
        }
        self.advance();
        let params = self.comma_list(&Tok::RParen, |p| p.pattern())?;
        let guard = if named && matches!(self.raw(), Tok::If) {
            self.advance();
            Some(self.expr()?)
        } else {
            None
        };
        let head = fn_tok.span.to(self.last);
        let body = match self.raw() {
            Tok::Eq => {
                self.advance();
                self.skip_newlines();
                self.expr()?
            }
            Tok::Newline => {
                let stmts = self.block(fn_tok.span, "fn")?;
                Expr::new(ExprKind::Block(stmts), head.to(self.last))
            }
            _ => {
                let tok = self.toks[self.pos].clone();
                let mut d = if matches!(tok.tok, Tok::Arrow | Tok::FatArrow) {
                    let arrow = lexer::symbol_text(&tok.tok);
                    Diagnostic::error("E001", t!("functions use `=`, not `{arrow}`", "les fonctions utilisent `=`, pas `{arrow}`"))
                        .at(tok.span)
                        .help(t!("write `fn(x) = x * 2`", "écrivez `fn(x) = x * 2`"))
                        .boxed()
                } else {
                    self.unexpected_token(&tok, t!("`=` or a new line", "`=` ou une nouvelle ligne"))
                };
                d.helps.push(t!(
                    "a function is written `fn name(x) = expression`,\nor with its body on the following lines, finished by `end`",
                    "une fonction s'écrit `fn nom(x) = expression`,\nou avec son corps sur les lignes suivantes, terminé par `end`"
                ));
                return Err(d);
            }
        };
        let arity = params.len();
        Ok(FnDef { name, clauses: vec![Rc::new(Clause { params, guard, body, span: head })], arity, doc, span: name_span })
    }

    fn type_def(&mut self) -> Fallible<Item> {
        self.skip_soft();
        let doc = self.docs.get(&self.pos).map(|d| Rc::from(d.as_str()));
        let kw = self.advance();
        let t = self.advance();
        let name = match t.tok {
            Tok::Ctor(s) => s,
            Tok::Ident(s) => {
                let n = s.name();
                let cap = capitalize(&n);
                return Err(Diagnostic::error(
                    "E001",
                    t!("type names start with an uppercase letter", "les noms de types commencent par une majuscule"),
                )
                .at(t.span)
                .help(t!("write `type {cap} = ...`", "écrivez `type {cap} = ...`"))
                .boxed());
            }
            _ => return Err(self.unexpected_token(&t, t!("a type name", "un nom de type"))),
        };
        if !self.eat(&Tok::Eq) {
            let mut d = self.unexpected(t!("`=`", "`=`"));
            d.helps
                .push(t!("example: `type Shape = Circle(radius) | Square(side)`", "exemple : `type Forme = Cercle(rayon) | Carre(cote)`"));
            return Err(d);
        }
        self.skip_newlines();
        self.eat(&Tok::Bar);
        let mut variants = Vec::new();
        loop {
            self.skip_newlines();
            let t = self.advance();
            let vname = match t.tok {
                Tok::Ctor(s) => s,
                Tok::Ident(s) => {
                    let cap = capitalize(&s.name());
                    return Err(Diagnostic::error(
                        "E001",
                        t!("variant names start with an uppercase letter", "les noms de variantes commencent par une majuscule"),
                    )
                    .at(t.span)
                    .help(t!("write `{cap}`", "écrivez `{cap}`"))
                    .boxed());
                }
                _ => return Err(self.unexpected_token(&t, t!("a variant name, like `Circle`", "un nom de variante, comme `Cercle`"))),
            };
            let mut fields = Vec::new();
            if matches!(self.raw(), Tok::LParen) {
                self.advance();
                fields = self.comma_list(&Tok::RParen, |p| {
                    let t = p.advance();
                    match t.tok {
                        Tok::Ident(s) => Ok(s),
                        _ => {
                            let mut d = p.unexpected_token(&t, t!("a field name", "un nom de champ"));
                            d.helps.push(t!(
                                "fields are named in lowercase: `Circle(radius)`",
                                "les champs se nomment en minuscules : `Cercle(rayon)`"
                            ));
                            Err(d)
                        }
                    }
                })?;
            }
            variants.push(VariantDef { name: vname, fields, span: t.span.to(self.last) });
            if matches!(self.past_newlines(), Tok::Bar) {
                self.skip_newlines();
                self.advance();
                continue;
            }
            break;
        }
        Ok(Item::Type(TypeDef { name, variants, doc, span: kw.span.to(t.span) }))
    }

    fn test_def(&mut self) -> Fallible<Item> {
        let kw = self.advance();
        let t = self.advance();
        let name = match t.tok {
            Tok::Str(parts) if parts.iter().all(|p| matches!(p, StrPart::Lit(_))) => {
                parts.into_iter().map(|p| if let StrPart::Lit(s) = p { s } else { String::new() }).collect::<String>()
            }
            _ => {
                let mut d = self.unexpected_token(&t, t!("the name of the test, as a text", "le nom du test, sous forme de texte"));
                d.helps.push(t!(
                    "example: `test \"addition works\" = assert_eq(1 + 1, 2)`",
                    "exemple : `test \"l'addition marche\" = assert_eq(1 + 1, 2)`"
                ));
                return Err(d);
            }
        };
        let body = match self.raw() {
            Tok::Eq => {
                self.advance();
                self.skip_newlines();
                self.expr()?
            }
            Tok::Newline => {
                let stmts = self.block(kw.span, "test")?;
                Expr::new(ExprKind::Block(stmts), kw.span.to(self.last))
            }
            _ => return Err(self.unexpected(t!("`=` or a new line", "`=` ou une nouvelle ligne"))),
        };
        Ok(Item::Test(Rc::from(name), Rc::new(body), kw.span.to(t.span)))
    }

    fn use_def(&mut self) -> Fallible<Item> {
        let kw = self.advance();
        let t = self.advance();
        let (path, path_span, short) = match t.tok {
            Tok::Str(parts) if parts.iter().all(|p| matches!(p, StrPart::Lit(_))) => {
                let path: String = parts.into_iter().map(|p| if let StrPart::Lit(s) = p { s } else { String::new() }).collect();
                (path, t.span, false)
            }
            // `use geometry`, `use shapes/circle`, `use ../common/tools`.
            Tok::Ident(_) | Tok::DotDot => {
                let mut path = String::new();
                let mut last = t.clone();
                loop {
                    match &last.tok {
                        Tok::Ident(s) => path.push_str(&s.name()),
                        Tok::DotDot => path.push_str(".."),
                        _ => return Err(self.use_path_error(&last)),
                    }
                    if !matches!(self.raw(), Tok::Slash) {
                        break;
                    }
                    self.advance();
                    path.push('/');
                    last = self.advance();
                }
                if !matches!(last.tok, Tok::Ident(_)) {
                    return Err(self.use_path_error(&last));
                }
                if matches!(self.raw(), Tok::Dot) {
                    let dot = self.toks[self.pos].span;
                    return Err(Diagnostic::error(
                        "E001",
                        t!("write the name of the file without `.ry`", "écrivez le nom du fichier sans `.ry`"),
                    )
                    .at(dot)
                    .help(t!("example: `use {path}`", "exemple : `use {path}`"))
                    .boxed());
                }
                path.push_str(".ry");
                (path, t.span.to(last.span), true)
            }
            _ => return Err(self.use_path_error(&t)),
        };
        let mut name = None;
        if matches!(self.raw(), Tok::Ident(s) if &*s.name() == "as") {
            self.advance();
            let n = self.advance();
            match n.tok {
                Tok::Ident(s) if &*s.name() != "_" => name = Some((s, n.span)),
                _ => {
                    let mut d = self.unexpected_token(&n, t!("a name for the file, in lowercase", "un nom pour le fichier, en minuscules"));
                    d.helps.push(t!(
                        "example: `use geometry as geo`, then `geo.area(2)`",
                        "exemple : `use geometrie as geo`, puis `geo.aire(2)`"
                    ));
                    return Err(d);
                }
            }
        } else if short {
            // `use shapes/circle`: the name of the file, `circle`.
            let stem = path.trim_end_matches(".ry").rsplit('/').next().unwrap_or_default().to_string();
            name = Some((Sym::new(&stem), self.last));
        }
        Ok(Item::Use(Use { path: Rc::from(path), span: kw.span.to(path_span), path_span, name, short }))
    }

    fn use_path_error(&self, t: &Token) -> Box<Diagnostic> {
        let mut d = self.unexpected_token(t, t!("the name of a file", "le nom d'un fichier"));
        d.helps.push(t!(
            "example: `use geometry` for the file `geometry.ry`, then `geometry.area(2)`",
            "exemple : `use geometrie` pour le fichier `geometrie.ry`, puis `geometrie.aire(2)`"
        ));
        if matches!(t.tok, Tok::Ctor(_)) {
            d.helps.push(t!(
                "for a file whose name starts with an uppercase letter, write its path as a text: `use \"Shapes.ry\" as shapes`",
                "pour un fichier dont le nom commence par une majuscule, écrivez son chemin comme un texte : `use \"Formes.ry\" as formes`"
            ));
        }
        d
    }

    // ----- expressions -------------------------------------------------

    pub fn expr(&mut self) -> Fallible<Expr> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Fallible<Expr> {
        let mut l = self.and_expr()?;
        loop {
            match self.peek_cont() {
                Tok::Or => {
                    let op = self.take_cont();
                    self.skip_newlines();
                    let r = self.and_expr()?;
                    l = self.limit(binary(BinOp::Or, l, r, op.span), op.span)?;
                }
                Tok::BarBar => return Err(self.unexpected(t!("the end of the expression", "la fin de l'expression"))),
                _ => return Ok(l),
            }
        }
    }

    fn and_expr(&mut self) -> Fallible<Expr> {
        let mut l = self.not_expr()?;
        loop {
            match self.peek_cont() {
                Tok::And => {
                    let op = self.take_cont();
                    self.skip_newlines();
                    let r = self.not_expr()?;
                    l = self.limit(binary(BinOp::And, l, r, op.span), op.span)?;
                }
                Tok::AmpAmp => return Err(self.unexpected(t!("the end of the expression", "la fin de l'expression"))),
                _ => return Ok(l),
            }
        }
    }

    fn not_expr(&mut self) -> Fallible<Expr> {
        if matches!(self.peek(), Tok::Not) {
            let t = self.advance();
            let e = self.deeper(|p| p.not_expr())?;
            let span = t.span.to(e.span);
            return Ok(Expr::new(ExprKind::Unary(UnOp::Not, Box::new(e)), span));
        }
        self.cmp_expr()
    }

    fn cmp_op(t: &Tok) -> Option<BinOp> {
        Some(match t {
            Tok::EqEq => BinOp::Eq,
            Tok::NotEq => BinOp::Ne,
            Tok::Lt => BinOp::Lt,
            Tok::Le => BinOp::Le,
            Tok::Gt => BinOp::Gt,
            Tok::Ge => BinOp::Ge,
            _ => return None,
        })
    }

    fn cmp_expr(&mut self) -> Fallible<Expr> {
        let l = self.pipe_expr()?;
        let Some(op) = Self::cmp_op(self.peek_cont()) else { return Ok(l) };
        let t = self.take_cont();
        self.skip_newlines();
        let r = self.pipe_expr()?;
        let e = binary(op, l, r, t.span);
        if Self::cmp_op(self.peek_cont()).is_some() {
            let t2 = self.take_cont();
            return Err(Diagnostic::error("E001", t!("comparisons cannot be chained", "les comparaisons ne peuvent pas s'enchaîner"))
                .at(t2.span)
                .help(t!("combine them with `and`: `a < b and b < c`", "combinez-les avec `and` : `a < b and b < c`"))
                .boxed());
        }
        Ok(e)
    }

    fn pipe_expr(&mut self) -> Fallible<Expr> {
        let mut l = self.range_expr()?;
        let mut piped = false;
        while matches!(self.peek_cont(), Tok::PipeGt) {
            let op = self.take_cont();
            self.skip_newlines();
            l = self.deeper(|p| p.pipe_step(l))?;
            l = self.limit(l, op.span)?;
            piped = true;
        }
        let arithmetic = matches!(
            self.peek_cont(),
            Tok::Plus | Tok::Minus | Tok::Star | Tok::Slash | Tok::SlashSlash | Tok::Percent | Tok::StarStar | Tok::DotDot
        );
        if piped && arithmetic {
            let op = self.peek_token();
            let sym = lexer::symbol_text(&op.tok);
            return Err(Diagnostic::error("E001", t!("`{sym}` cannot follow a `|>` step directly", "`{sym}` ne peut pas suivre directement une étape `|>`"))
                .at(op.span)
                .help(t!(
                    "`|>` comes after the calculations: put the pipeline in parentheses,\nlike `(x |> f) {sym} 1`, or continue it with a function: `x |> f |> fn(v) = v {sym} 1`",
                    "`|>` passe après les calculs : mettez l'enchaînement entre parenthèses,\ncomme `(x |> f) {sym} 1`, ou continuez-le avec une fonction : `x |> f |> fn(v) = v {sym} 1`"
                ))
                .boxed());
        }
        Ok(l)
    }

    /// After `|>`: a function, with its other arguments if any. The value on the
    /// left becomes the first argument; `?`, `[i]` and `.field` written after
    /// the step apply to its result.
    fn pipe_step(&mut self, input: Expr) -> Fallible<Expr> {
        let start = input.span;
        let mut callee = self.primary()?;
        // `x |> record.field` or `x |> value.method(args)`.
        while matches!(self.raw(), Tok::Dot) {
            self.advance();
            let (name, name_span) = self.member()?;
            if matches!(self.raw(), Tok::LParen) {
                self.advance();
                let mut args = self.comma_list(&Tok::RParen, |p| p.expr())?;
                args.insert(0, input);
                let call = Expr::new(ExprKind::Method(Box::new(callee), Sym::new(&name), args, name_span), start.to(self.last));
                return self.postfix_ops(call);
            }
            let span = callee.span.to(name_span);
            callee = Expr::new(ExprKind::Field(Box::new(callee), name, name_span), span);
        }
        let mut args = vec![input];
        if matches!(self.raw(), Tok::LParen) {
            self.advance();
            args.extend(self.comma_list(&Tok::RParen, |p| p.expr())?);
        }
        let call = Expr::new(ExprKind::Call(Box::new(callee), args), start.to(self.last));
        self.postfix_ops(call)
    }

    /// The name after a `.`, and where it is: `even?` too (see `predicate_at`).
    fn member(&mut self) -> Fallible<(Rc<str>, Span)> {
        let t = self.advance();
        let name = self.member_name(&t)?;
        if let Some(pred) = predicate_at(&self.toks, self.pos - 1, &self.preds) {
            let q = self.advance();
            return Ok((pred.name(), t.span.to(q.span)));
        }
        Ok((name, t.span))
    }

    /// Is the `do` or `{` here followed by the parameters of a block: `|x|`?
    fn block_follows(&self) -> bool {
        matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Bar | Tok::BarBar))
    }

    /// `do |x| ... end` or `{ |x| ... }` written alone, not after a call.
    fn lone_block(&mut self) -> Box<Diagnostic> {
        let t = self.peek_token();
        Diagnostic::error("E001", t!("a block must follow a call", "un bloc doit suivre un appel"))
            .at(t.span)
            .label(t!("not given to a function", "donné à aucune fonction"))
            .help(t!(
                "write the block on the line of the call it is given to: `xs.map do |x| x * 2 end`;\nfor a function as a value, write `fn(x) = x * 2`",
                "écrivez le bloc sur la ligne de l'appel auquel il est donné : `xs.map do |x| x * 2 end` ;\npour une fonction comme valeur, écrivez `fn(x) = x * 2`"
            ))
            .boxed()
    }

    /// A block given to a call, `do |x, y| ... end` or `{ |x| ... }`: a
    /// function, the same as `fn(x, y) = ...`.
    fn block_arg(&mut self) -> Fallible<Expr> {
        self.deeper(|p| {
            let open = p.advance();
            let brace = open.tok == Tok::LBrace;
            let mut params = Vec::new();
            match p.raw() {
                Tok::BarBar => {
                    p.advance();
                }
                Tok::Bar => {
                    p.advance();
                    params = p.nested(false, |p| {
                        let mut params = Vec::new();
                        loop {
                            if p.eat(&Tok::Bar) {
                                return Ok(params);
                            }
                            params.push(p.deeper(|p| p.pattern_atom())?);
                            if p.eat(&Tok::Comma) {
                                continue;
                            }
                            if p.eat(&Tok::Bar) {
                                return Ok(params);
                            }
                            let mut d = p.unexpected(t!("`,` or `|`", "`,` ou `|`"));
                            d.helps.push(t!(
                                "the parameters of a block go between bars: `do |name, age| ... end`",
                                "les paramètres d'un bloc vont entre deux barres : `do |nom, age| ... end`"
                            ));
                            return Err(d);
                        }
                    })?;
                }
                _ => {}
            }
            let head = open.span.to(p.last);
            let stmts = if brace { p.brace_block(open.span)? } else { p.block_inner(open.span, "do")? };
            let span = open.span.to(p.last);
            let body = Expr::new(ExprKind::Block(stmts), span);
            let arity = params.len();
            let def = FnDef {
                name: None,
                clauses: vec![Rc::new(Clause { params, guard: None, body, span: head })],
                arity,
                doc: None,
                span: open.span,
            };
            Ok(Expr::new(ExprKind::Lambda(Rc::new(def)), span))
        })
    }

    /// The statements of `{ |x| ... }`, up to `}`.
    fn brace_block(&mut self, opener: Span) -> Fallible<Vec<Stmt>> {
        self.nested(true, |p| {
            let mut stmts = Vec::new();
            loop {
                p.skip_newlines();
                match p.raw() {
                    Tok::RBrace => {
                        p.advance();
                        return Ok(stmts);
                    }
                    Tok::Eof => {
                        let d = p.unexpected(t!("`}}`", "`}}`"));
                        return Err(d.secondary(opener, t!("the block starts here", "le bloc commence ici")).boxed());
                    }
                    _ => {}
                }
                let s = p.stmt()?;
                if !matches!(p.raw(), Tok::RBrace) {
                    p.stmt_end(Some(&s))?;
                }
                push_stmt(&mut stmts, s)?;
            }
        })
    }

    /// The name after a `.`: a field or a function. Keywords are allowed, so
    /// that records can have fields like `type` or `class`.
    fn member_name(&self, t: &Token) -> Fallible<Rc<str>> {
        match &t.tok {
            Tok::Ident(s) | Tok::Ctor(s) | Tok::Reserved(s) => Ok(s.name()),
            other => match lexer::keyword_text(other) {
                Some(k) => Ok(Rc::from(k)),
                None => Err(self.unexpected_token(t, t!("a field or function name after `.`", "un nom de champ ou de fonction après `.`"))),
            },
        }
    }

    fn range_expr(&mut self) -> Fallible<Expr> {
        let l = self.add_expr()?;
        if matches!(self.peek_cont(), Tok::DotDot) {
            self.take_cont();
            self.skip_newlines();
            let r = self.add_expr()?;
            let span = l.span.to(r.span);
            return Ok(Expr::new(ExprKind::Range(Box::new(l), Box::new(r)), span));
        }
        Ok(l)
    }

    fn add_expr(&mut self) -> Fallible<Expr> {
        let mut l = self.mul_expr()?;
        loop {
            let op = match self.peek_cont() {
                Tok::Plus => BinOp::Add,
                Tok::Minus => BinOp::Sub,
                _ => return Ok(l),
            };
            let t = self.take_cont();
            self.skip_newlines();
            let r = self.mul_expr()?;
            l = self.limit(binary(op, l, r, t.span), t.span)?;
        }
    }

    fn mul_expr(&mut self) -> Fallible<Expr> {
        let mut l = self.unary()?;
        loop {
            let op = match self.peek_cont() {
                Tok::Star => BinOp::Mul,
                Tok::Slash => BinOp::Div,
                Tok::SlashSlash => BinOp::IntDiv,
                Tok::Percent => BinOp::Mod,
                _ => return Ok(l),
            };
            let t = self.take_cont();
            self.skip_newlines();
            let r = self.unary()?;
            l = self.limit(binary(op, l, r, t.span), t.span)?;
        }
    }

    fn unary(&mut self) -> Fallible<Expr> {
        match self.peek() {
            Tok::Minus => {
                let t = self.advance();
                let e = self.deeper(|p| p.unary())?;
                let span = t.span.to(e.span);
                Ok(match e.kind {
                    kind @ (ExprKind::Int(_) | ExprKind::Big(_)) => Expr::new(negated(kind), span),
                    ExprKind::Float(f) => Expr::new(ExprKind::Float(-f), span),
                    kind => Expr::new(ExprKind::Unary(UnOp::Neg, Box::new(Expr::new(kind, e.span))), span),
                })
            }
            Tok::Bang => Err(self.unexpected(t!("an expression", "une expression"))),
            _ => self.power(),
        }
    }

    fn power(&mut self) -> Fallible<Expr> {
        let base = self.postfix()?;
        if matches!(self.peek_cont(), Tok::StarStar) {
            let t = self.take_cont();
            self.skip_newlines();
            // `**` groups to the right: `2 ** 3 ** 2` is `2 ** (3 ** 2)`.
            let exp = self.deeper(|p| p.unary())?;
            return self.limit(binary(BinOp::Pow, base, exp, t.span), t.span);
        }
        Ok(base)
    }

    fn postfix(&mut self) -> Fallible<Expr> {
        // The arguments of the calls that follow are nested inside this level too.
        self.deeper(|p| {
            let e = p.primary_inner()?;
            let e = p.limit(e, p.last)?;
            p.postfix_ops(e)
        })
    }

    /// Calls, `[index]`, `?`, `.field`, `.method()` and blocks (`do |x| ...
    /// end`) after an expression.
    fn postfix_ops(&mut self, mut e: Expr) -> Fallible<Expr> {
        loop {
            e = self.limit(e, self.last)?;
            // A block starts on the line of the call it is given to.
            if (matches!(self.raw(), Tok::Do) && !self.no_do) || (matches!(self.raw(), Tok::LBrace) && self.block_follows()) {
                if !matches!(e.kind, ExprKind::Call(..) | ExprKind::Method(..) | ExprKind::Field(..) | ExprKind::Var(_)) {
                    return Ok(e);
                }
                let block = self.block_arg()?;
                e = with_block(e, block);
                continue;
            }
            self.skip_soft();
            match self.raw() {
                Tok::LParen => {
                    self.advance();
                    let args = self.comma_list(&Tok::RParen, |p| p.expr())?;
                    let span = e.span.to(self.last);
                    e = Expr::new(ExprKind::Call(Box::new(e), args), span);
                }
                Tok::LBracket => {
                    self.advance();
                    let idx = self.nested(false, |p| {
                        let i = p.expr()?;
                        if !p.eat(&Tok::RBracket) {
                            return Err(p.unexpected(t!("`]`", "`]`")));
                        }
                        Ok(i)
                    })?;
                    let span = e.span.to(self.last);
                    e = Expr::new(ExprKind::Index(Box::new(e), Box::new(idx)), span);
                }
                Tok::Question => {
                    let t = self.advance();
                    let span = e.span.to(t.span);
                    e = Expr::new(ExprKind::Try(Box::new(e)), span);
                }
                _ => {
                    if !matches!(self.peek_cont(), Tok::Dot) {
                        return Ok(e);
                    }
                    self.take_cont();
                    let (name, name_span) = self.member()?;
                    if matches!(self.raw(), Tok::LParen) {
                        self.advance();
                        let args = self.comma_list(&Tok::RParen, |p| p.expr())?;
                        let span = e.span.to(self.last);
                        e = Expr::new(ExprKind::Method(Box::new(e), Sym::new(&name), args, name_span), span);
                    } else {
                        let span = e.span.to(name_span);
                        e = Expr::new(ExprKind::Field(Box::new(e), name, name_span), span);
                    }
                }
            }
        }
    }

    /// Parses `item, item, ...` up to `close` (the opening bracket is already consumed).
    fn comma_list<T>(&mut self, close: &Tok, mut item: impl FnMut(&mut Self) -> Fallible<T>) -> Fallible<Vec<T>> {
        let open_span = self.last;
        let close = close.clone();
        self.nested(false, |p| {
            let mut items = Vec::new();
            loop {
                if p.eat(&close) {
                    return Ok(items);
                }
                items.push(item(p)?);
                if p.eat(&Tok::Comma) {
                    continue;
                }
                if p.eat(&close) {
                    return Ok(items);
                }
                let closing = lexer::symbol_text(&close);
                let mut d = p.unexpected(t!("`,` or `{closing}`", "`,` ou `{closing}`"));
                if starts_expression(p.peek()) {
                    d.helps.push(t!("separate the elements with commas", "séparez les éléments par des virgules"));
                }
                if d.incomplete {
                    d.secondary.push((open_span, t!("opened here", "ouvert ici")));
                }
                return Err(d);
            }
        })
    }

    /// Runs `f` one nesting level deeper, refusing absurdly deep code.
    fn deeper<T>(&mut self, f: impl FnOnce(&mut Self) -> Fallible<T>) -> Fallible<T> {
        if self.depth >= MAX_NESTING {
            let at = self.peek_token().span;
            return Err(Diagnostic::error(
                "E001",
                t!(
                    "this code is nested too deeply (more than {MAX_NESTING} levels)",
                    "ce code est imbriqué trop profondément (plus de {MAX_NESTING} niveaux)"
                ),
            )
            .at(at)
            .help(t!("split it into several named values or functions", "découpez-le en plusieurs valeurs ou fonctions nommées"))
            .boxed());
        }
        self.depth += 1;
        let r = f(self);
        self.depth -= 1;
        r
    }

    /// Refuses an expression that is too deep (see `MAX_DEPTH`); `at` is
    /// where it became too deep.
    fn limit(&self, e: Expr, at: Span) -> Fallible<Expr> {
        if e.depth <= MAX_DEPTH {
            return Ok(e);
        }
        Err(Diagnostic::error(
            "E001",
            t!(
                "this expression is too long or nested too deeply (more than {MAX_DEPTH} levels)",
                "cette expression est trop longue ou imbriquée trop profondément (plus de {MAX_DEPTH} niveaux)"
            ),
        )
        .at(at)
        .help(t!(
            "split it into several named values or functions;\nto combine many values, put them in a list: `sum([a, b, c])`, `join([x, y, z])`",
            "découpez-la en plusieurs valeurs ou fonctions nommées ;\npour combiner beaucoup de valeurs, mettez-les dans une liste : `sum([a, b, c])`, `join([x, y, z])`"
        ))
        .boxed())
    }

    fn primary(&mut self) -> Fallible<Expr> {
        self.deeper(|p| {
            let e = p.primary_inner()?;
            p.limit(e, p.last)
        })
    }

    fn primary_inner(&mut self) -> Fallible<Expr> {
        let tok = self.peek_token();
        let span = tok.span;
        match tok.tok {
            Tok::Int(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::Int(n), span))
            }
            Tok::Big(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::Big(n), span))
            }
            Tok::Float(f) => {
                self.advance();
                Ok(Expr::new(ExprKind::Float(f), span))
            }
            Tok::True | Tok::False => {
                self.advance();
                Ok(Expr::new(ExprKind::Bool(tok.tok == Tok::True), span))
            }
            Tok::Str(parts) => {
                self.advance();
                self.text(parts, span)
            }
            Tok::Ident(_) if unless_at(&self.toks, self.pos) => self.if_expr(true),
            Tok::Ident(s) => {
                self.advance();
                if let Some(name) = predicate_at(&self.toks, self.pos - 1, &self.preds) {
                    let q = self.advance();
                    return Ok(Expr::new(ExprKind::Var(name), span.to(q.span)));
                }
                Ok(Expr::new(ExprKind::Var(s), span))
            }
            Tok::Symbol(s) => {
                self.advance();
                Ok(Expr::new(ExprKind::Symbol(s), span))
            }
            Tok::Ctor(s) => {
                self.advance();
                Ok(Expr::new(ExprKind::Ctor(s), span))
            }
            Tok::Reserved(s) => Err(self.reserved_error(s, span)),
            Tok::LParen => {
                self.advance();
                self.nested(false, |p| {
                    if p.eat(&Tok::RParen) {
                        return Ok(Expr::new(ExprKind::Unit, span.to(p.last)));
                    }
                    let e = p.expr()?;
                    if matches!(p.peek(), Tok::Comma) {
                        let c = p.advance();
                        return Err(Diagnostic::error("E001", t!("Rythme has no tuples", "Rythme n'a pas de tuples"))
                            .at(c.span)
                            .help(t!(
                                "group values in a list `[a, b]` or a record `{{x: a, y: b}}`",
                                "regroupez les valeurs dans une liste `[a, b]` ou un enregistrement `{{x: a, y: b}}`"
                            ))
                            .boxed());
                    }
                    if !p.eat(&Tok::RParen) {
                        return Err(p.unexpected(t!("`)`", "`)`")));
                    }
                    Ok(e)
                })
            }
            Tok::LBracket => {
                self.advance();
                let items = self.comma_list(&Tok::RBracket, |p| {
                    if p.eat(&Tok::DotDot) {
                        Ok(ListItem::Spread(p.expr()?))
                    } else {
                        Ok(ListItem::One(p.expr()?))
                    }
                })?;
                Ok(Expr::new(ExprKind::List(items), span.to(self.last)))
            }
            Tok::LBrace if self.block_follows() => Err(self.lone_block()),
            Tok::LBrace => {
                self.advance();
                let items = self.comma_list(&Tok::RBrace, |p| p.record_item())?;
                Ok(Expr::new(ExprKind::Record(items), span.to(self.last)))
            }
            Tok::Fn => {
                if matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Ident(_))) {
                    return Err(Diagnostic::error(
                        "E001",
                        t!(
                            "a named function cannot be used inside an expression",
                            "une fonction nommée ne peut pas être utilisée dans une expression"
                        ),
                    )
                    .at(span)
                    .help(t!(
                        "define it on its own line, or use an anonymous function: `fn(x) = x * 2`",
                        "définissez-la sur sa propre ligne, ou utilisez une fonction anonyme : `fn(x) = x * 2`"
                    ))
                    .boxed());
                }
                let def = self.fn_def(false)?;
                Ok(Expr::new(ExprKind::Lambda(Rc::new(def)), span.to(self.last)))
            }
            Tok::If => self.if_expr(false),
            Tok::Match => self.match_expr(),
            Tok::Do if self.block_follows() => Err(self.lone_block()),
            Tok::Do => {
                self.advance();
                let stmts = self.block(span, "do")?;
                Ok(Expr::new(ExprKind::Block(stmts), span.to(self.last)))
            }
            Tok::SlashSlash => {
                let mut d = self.unexpected(t!("an expression", "une expression"));
                d.helps.push(t!("comments start with `#`", "les commentaires commencent par `#`"));
                Err(d)
            }
            Tok::Else => {
                let mut d = self.unexpected(t!("an expression", "une expression"));
                d.helps.push(t!(
                    "`else` must follow `if condition then value`; a branch holds a single expression (use `do ... end` for several lines)",
                    "`else` doit suivre `if condition then valeur` ; une branche contient une seule expression (utilisez `do ... end` pour plusieurs lignes)"
                ));
                Err(d)
            }
            _ => Err(self.unexpected(t!("an expression", "une expression"))),
        }
    }

    fn record_item(&mut self) -> Fallible<RecordItem> {
        if self.eat(&Tok::DotDot) {
            return Ok(RecordItem::Spread(self.expr()?));
        }
        let t = self.advance();
        let Some(key) = field_key(&t.tok) else {
            let mut d = self.unexpected_token(&t, t!("a field name", "un nom de champ"));
            d.helps.push(t!(
                "a record is written `{{name: \"Ada\", age: 36}}`; for a block of code, use `do ... end`",
                "un enregistrement s'écrit `{{nom: \"Ada\", age: 36}}` ; pour un bloc de code, utilisez `do ... end`"
            ));
            return Err(d);
        };
        if self.eat(&Tok::Colon) {
            return Ok(RecordItem::Field(key, t.span, self.expr()?));
        }
        match t.tok {
            Tok::Ident(s) => Ok(RecordItem::Field(key, t.span, Expr::new(ExprKind::Var(s), t.span))),
            // `{class}` would read a variable named `class`, which cannot exist.
            Tok::Reserved(s) => Err(self.reserved_error(s, t.span)),
            _ => Err(self.unexpected(t!("`:` and a value", "`:` et une valeur"))),
        }
    }

    fn text(&mut self, parts: Vec<StrPart>, span: Span) -> Fallible<Expr> {
        if let [StrPart::Lit(s)] = parts.as_slice() {
            return Ok(Expr::new(ExprKind::Text(Rc::from(s.as_str())), span));
        }
        let mut out = Vec::new();
        for part in parts {
            match part {
                StrPart::Lit(s) => out.push(TemplatePart::Lit(Rc::from(s))),
                StrPart::Code(tokens) => {
                    let mut sub = Parser {
                        toks: tokens,
                        pos: 0,
                        sig: vec![false],
                        docs: HashMap::new(),
                        last: span,
                        depth: self.depth,
                        in_text: true,
                        preds: self.preds.clone(),
                        values: self.values.clone(),
                        no_do: false,
                        commands: HashSet::new(),
                    };
                    let e =
                        sub.expr().and_then(
                            |e| {
                                if matches!(sub.peek(), Tok::Eof) {
                                    Ok(e)
                                } else {
                                    Err(sub.unexpected(t!("`}}`", "`}}`")))
                                }
                            },
                        );
                    match e {
                        Ok(e) => out.push(TemplatePart::Expr(e)),
                        Err(mut d) => {
                            d.incomplete = false;
                            lexer::add_literal_brace_help(&mut d);
                            return Err(d);
                        }
                    }
                }
            }
        }
        Ok(Expr::new(ExprKind::Template(out), span))
    }

    /// `if c then a else b`, or `unless c then a else b` (the same as
    /// `if c then b else a`).
    fn if_expr(&mut self, unless: bool) -> Fallible<Expr> {
        let kw = self.advance();
        let word = if unless { "unless" } else { "if" };
        let cond = self.expr()?;
        if matches!(self.past_newlines(), Tok::Then) {
            self.skip_newlines();
        }
        if !self.eat(&Tok::Then) {
            let mut d = self.unexpected(t!("`then`", "`then`"));
            d.helps.push(t!(
                "an `{word}` is written: `{word} condition then value else other_value`",
                "un `{word}` s'écrit : `{word} condition then valeur else autre_valeur`"
            ));
            return Err(d);
        }
        self.skip_newlines();
        let then_branch = self.expr()?;
        if matches!(self.past_newlines(), Tok::Else) {
            self.skip_newlines();
        }
        if !self.eat(&Tok::Else) {
            let next = self.past_newlines().clone();
            let at = {
                let mut i = self.pos;
                while matches!(self.toks[i].tok, Tok::Newline) {
                    i += 1;
                }
                self.toks[i].span
            };
            let mut d = Diagnostic::error("E019", t!("this `{word}` has no `else`", "ce `{word}` n'a pas de `else`"))
                .at(kw.span)
                .label(t!("every `{word}` must have an `else`", "chaque `{word}` doit avoir un `else`"))
                .secondary(at, t!("expected `else` here", "`else` attendu ici"));
            match next {
                Tok::Reserved(s) if matches!(&*s.name(), "elif" | "elsif") => {
                    d = d.help(t!("write `else if`", "écrivez `else if`"));
                }
                Tok::Eof => d = d.incomplete(),
                _ => {}
            }
            d = d.help(if unless {
                t!(
                    "in Rythme, `unless` always gives a value, so it needs both cases:\n  unless age >= 18 then \"minor\" else \"adult\"",
                    "en Rythme, `unless` donne toujours une valeur, il lui faut donc les deux cas :\n  unless age >= 18 then \"mineur\" else \"majeur\""
                )
            } else {
                t!(
                    "in Rythme, `if` always gives a value, so it needs both cases:\n  if age >= 18 then \"adult\" else \"minor\"",
                    "en Rythme, `if` donne toujours une valeur, il lui faut donc les deux cas :\n  if age >= 18 then \"majeur\" else \"mineur\""
                )
            });
            if !matches!(next, Tok::Eof | Tok::End | Tok::RParen) {
                d = d.help(t!(
                    "a branch holds a single expression; for several lines, use `do ... end`",
                    "une branche contient une seule expression ; pour plusieurs lignes, utilisez `do ... end`"
                ));
            }
            return Err(d.boxed());
        }
        self.skip_newlines();
        let else_branch = self.expr()?;
        let span = kw.span.to(else_branch.span);
        let (yes, no) = if unless { (else_branch, then_branch) } else { (then_branch, else_branch) };
        Ok(Expr::new(ExprKind::If(Box::new(cond), Box::new(yes), Box::new(no)), span))
    }

    fn match_expr(&mut self) -> Fallible<Expr> {
        let kw = self.advance();
        // New lines are significant while reading the subject, even inside brackets:
        // the cases must start on a new line.
        let subject = self.nested(true, |p| p.expr())?;
        if !matches!(self.raw(), Tok::Newline) {
            let mut d = self.unexpected(t!("a new line", "une nouvelle ligne"));
            d.helps.push(t!(
                "write each case on its own line, then `end`:\n  match x\n    0 -> \"zero\"\n    _ -> \"other\"\n  end",
                "écrivez chaque cas sur sa propre ligne, puis `end` :\n  match x\n    0 -> \"zéro\"\n    _ -> \"autre\"\n  end"
            ));
            return Err(d);
        }
        let arms = self.nested(true, |p| {
            let mut arms = Vec::new();
            loop {
                p.skip_newlines();
                match p.raw() {
                    Tok::End => {
                        p.advance();
                        break;
                    }
                    Tok::Eof => return Err(missing_end(p.toks[p.pos].span, kw.span, "match")),
                    Tok::Bar => {
                        let t = p.advance();
                        return Err(Diagnostic::error(
                            "E001",
                            t!("no `|` is needed before a case", "aucun `|` n'est nécessaire avant un cas"),
                        )
                        .at(t.span)
                        .help(t!("write each case as `pattern -> result`", "écrivez chaque cas sous la forme `motif -> résultat`"))
                        .boxed());
                    }
                    _ => {}
                }
                let pat = p.pattern()?;
                let guard = if p.eat(&Tok::If) { Some(p.expr()?) } else { None };
                if !p.eat(&Tok::Arrow) {
                    let mut d = p.unexpected(t!("`->`", "`->`"));
                    d.helps.push(t!("each case is written `pattern -> result`", "chaque cas s'écrit `motif -> résultat`"));
                    return Err(d);
                }
                p.skip_newlines();
                let body = p.expr()?;
                arms.push(Arm { pat, guard, body });
                match p.raw() {
                    Tok::Newline | Tok::End => {}
                    Tok::Eof => return Err(missing_end(p.toks[p.pos].span, kw.span, "match")),
                    _ => {
                        let mut d = p.unexpected(t!("a new line", "une nouvelle ligne"));
                        d.helps.push(t!("put each case on its own line", "mettez chaque cas sur sa propre ligne"));
                        return Err(d);
                    }
                }
            }
            Ok(arms)
        })?;
        if arms.is_empty() {
            return Err(Diagnostic::error("E001", t!("a `match` needs at least one case", "un `match` doit avoir au moins un cas"))
                .at(kw.span)
                .boxed());
        }
        let span = kw.span.to(self.last);
        Ok(Expr::new(ExprKind::Match(Box::new(subject), arms), span))
    }

    // ----- patterns ----------------------------------------------------

    fn pattern(&mut self) -> Fallible<Pattern> {
        self.deeper(|p| p.pattern_inner())
    }

    fn pattern_inner(&mut self) -> Fallible<Pattern> {
        let first = self.pattern_part()?;
        if !matches!(self.peek(), Tok::Bar) {
            return Ok(first);
        }
        let mut span = first.span;
        let mut alts = vec![first];
        while self.eat(&Tok::Bar) {
            let p = self.pattern_part()?;
            span = span.to(p.span);
            alts.push(p);
        }
        Ok(Pattern { kind: PatKind::Or(alts), span })
    }

    /// A pattern, a range of numbers `1..9`, or a text that starts with a
    /// text, `"Mr " + name`.
    fn pattern_part(&mut self) -> Fallible<Pattern> {
        let first = self.pattern_atom()?;
        match self.peek() {
            Tok::DotDot => {
                self.advance();
                let lo = range_bound(&first)?;
                let second = match self.peek() {
                    Tok::Int(_) | Tok::Big(_) | Tok::Float(_) | Tok::Minus => self.pattern_atom()?,
                    _ => {
                        let mut d = self.unexpected(t!("a number after `..`", "un nombre après `..`"));
                        d.helps.push(range_help());
                        return Err(d);
                    }
                };
                let hi = range_bound(&second)?;
                Ok(Pattern { kind: PatKind::Range(lo, hi), span: first.span.to(second.span) })
            }
            Tok::Plus => {
                let plus = self.advance();
                let PatKind::Text(start) = first.kind else {
                    return Err(Diagnostic::error(
                        "E001",
                        t!("only a text can come before `+` in a pattern", "seul un texte peut venir avant `+` dans un motif"),
                    )
                    .at(first.span.to(plus.span))
                    .help(prefix_help())
                    .help(t!(
                        "to look at the end of a text, use a guard: `s if ends_with(s, \"!\") -> ...`",
                        "pour regarder la fin d'un texte, utilisez une condition : `s if ends_with(s, \"!\") -> ...`"
                    ))
                    .boxed());
                };
                let rest = self.pattern_atom()?;
                if !matches!(rest.kind, PatKind::Bind(_) | PatKind::Wildcard) {
                    return Err(Diagnostic::error(
                        "E001",
                        t!("after `+`, a pattern takes a name or `_`", "après `+`, un motif prend un nom ou `_`"),
                    )
                    .at(rest.span)
                    .help(prefix_help())
                    .boxed());
                }
                let span = first.span.to(rest.span);
                Ok(Pattern { kind: PatKind::Prefix(start, Box::new(rest)), span })
            }
            _ => Ok(first),
        }
    }

    fn pattern_atom(&mut self) -> Fallible<Pattern> {
        let tok = self.peek_token();
        let span = tok.span;
        let kind = match tok.tok {
            // `geo.Circle(r)`: a constructor of the file used as `geo`.
            Tok::Ident(m)
                if matches!(self.toks.get(self.pos + 1).map(|t| &t.tok), Some(Tok::Dot))
                    && matches!(self.toks.get(self.pos + 2).map(|t| &t.tok), Some(Tok::Ctor(_))) =>
            {
                self.advance();
                self.advance();
                let c = self.advance();
                let Tok::Ctor(c) = c.tok else { unreachable!() };
                let s = Sym::new(&format!("{m}.{c}"));
                if matches!(self.raw(), Tok::LParen) {
                    self.advance();
                    let args = self.comma_list(&Tok::RParen, |p| p.pattern())?;
                    return Ok(Pattern { kind: PatKind::Ctor(s, args), span: span.to(self.last) });
                }
                return Ok(Pattern { kind: PatKind::Ctor(s, Vec::new()), span: span.to(self.last) });
            }
            Tok::Ident(s) => {
                self.advance();
                if &*s.name() == "_" {
                    PatKind::Wildcard
                } else {
                    PatKind::Bind(s)
                }
            }
            Tok::Ctor(s) => {
                self.advance();
                if matches!(self.raw(), Tok::LParen) {
                    self.advance();
                    let args = self.comma_list(&Tok::RParen, |p| p.pattern())?;
                    return Ok(Pattern { kind: PatKind::Ctor(s, args), span: span.to(self.last) });
                }
                PatKind::Ctor(s, Vec::new())
            }
            Tok::Int(n) => {
                self.advance();
                PatKind::Int(n)
            }
            Tok::Big(n) => {
                self.advance();
                PatKind::Big(n)
            }
            Tok::Float(f) => {
                self.advance();
                PatKind::Float(f)
            }
            Tok::Minus => {
                self.advance();
                let t = self.advance();
                let kind = match t.tok {
                    Tok::Int(n) => PatKind::Int(-n),
                    Tok::Big(n) => match negated(ExprKind::Big(n)) {
                        ExprKind::Int(n) => PatKind::Int(n),
                        ExprKind::Big(n) => PatKind::Big(n),
                        _ => unreachable!(),
                    },
                    Tok::Float(f) => PatKind::Float(-f),
                    _ => return Err(self.unexpected_token(&t, t!("a number", "un nombre"))),
                };
                return Ok(Pattern { kind, span: span.to(t.span) });
            }
            Tok::True | Tok::False => {
                self.advance();
                PatKind::Bool(tok.tok == Tok::True)
            }
            Tok::Symbol(s) => {
                self.advance();
                PatKind::Symbol(s)
            }
            Tok::Str(parts) => {
                self.advance();
                if !parts.iter().all(|p| matches!(p, StrPart::Lit(_))) {
                    return Err(Diagnostic::error(
                        "E001",
                        t!("a pattern cannot contain `{{...}}`", "un motif ne peut pas contenir `{{...}}`"),
                    )
                    .at(span)
                    .help(t!(
                        "use a guard instead: `s if s == \"...\" -> ...`",
                        "utilisez plutôt une condition : `s if s == \"...\" -> ...`"
                    ))
                    .boxed());
                }
                PatKind::Text(Rc::from(parts.iter().map(|p| if let StrPart::Lit(s) = p { s.as_str() } else { "" }).collect::<String>()))
            }
            Tok::LParen => {
                self.advance();
                return self.nested(false, |p| {
                    if p.eat(&Tok::RParen) {
                        return Ok(Pattern { kind: PatKind::Unit, span: span.to(p.last) });
                    }
                    let inner = p.pattern()?;
                    if !p.eat(&Tok::RParen) {
                        return Err(p.unexpected(t!("`)`", "`)`")));
                    }
                    Ok(inner)
                });
            }
            Tok::LBracket => {
                self.advance();
                return self.nested(false, |p| {
                    let mut elems = Vec::new();
                    let mut rest = None;
                    loop {
                        if matches!(p.peek(), Tok::RBracket) {
                            break;
                        }
                        if p.eat(&Tok::DotDot) {
                            let name = match p.peek_token().tok {
                                Tok::Ident(s) => {
                                    let t = p.advance();
                                    if &*s.name() == "_" {
                                        None
                                    } else {
                                        Some((s, t.span))
                                    }
                                }
                                _ => None,
                            };
                            rest = Some(name);
                            p.eat(&Tok::Comma);
                            if !matches!(p.peek(), Tok::RBracket) {
                                let mut d = p.unexpected(t!("`]`", "`]`"));
                                d.helps.push(t!(
                                    "`..rest` must be the last element: `[first, ..rest]`",
                                    "`..reste` doit être le dernier élément : `[premier, ..reste]`"
                                ));
                                return Err(d);
                            }
                            break;
                        }
                        elems.push(p.pattern()?);
                        if !p.eat(&Tok::Comma) {
                            break;
                        }
                    }
                    if !p.eat(&Tok::RBracket) {
                        return Err(p.unexpected(t!("`,` or `]`", "`,` ou `]`")));
                    }
                    Ok(Pattern { kind: PatKind::List(elems, rest), span: span.to(p.last) })
                });
            }
            Tok::LBrace => {
                self.advance();
                let fields = self.comma_list(&Tok::RBrace, |p| {
                    if p.eat(&Tok::DotDot) {
                        if let Tok::Ident(_) = p.peek() {
                            let name = p.peek_token();
                            return Err(p.named_record_rest(name.span));
                        }
                        return Ok(None);
                    }
                    let t = p.advance();
                    let Some(key) = field_key(&t.tok) else {
                        return Err(p.unexpected_token(&t, t!("a field name", "un nom de champ")));
                    };
                    if p.eat(&Tok::Colon) {
                        return Ok(Some((key, p.pattern()?)));
                    }
                    match t.tok {
                        Tok::Ident(s) => Ok(Some((key, Pattern { kind: PatKind::Bind(s), span: t.span }))),
                        Tok::Reserved(s) => Err(p.reserved_error(s, t.span)),
                        _ => Err(p.unexpected(t!("`:` and a pattern", "`:` et un motif"))),
                    }
                })?;
                return Ok(Pattern { kind: PatKind::Record(fields.into_iter().flatten().collect()), span: span.to(self.last) });
            }
            Tok::Reserved(s) => return Err(self.reserved_error(s, span)),
            _ => {
                let mut d = self.unexpected(t!("a pattern", "un motif"));
                d.helps.push(t!(
                    "patterns look like: `x`, `_`, `0`, `\"text\"`, `:ok`, `Some(x)`, `[first, ..rest]`, `{{name}}`",
                    "les motifs ressemblent à : `x`, `_`, `0`, `\"texte\"`, `:ok`, `Some(x)`, `[premier, ..reste]`, `{{nom}}`"
                ));
                return Err(d);
            }
        };
        Ok(Pattern { kind, span })
    }
}

/// The name of a field in a record or a record pattern: a name, a keyword
/// (`type`, `class`...), or a text (`"full name"`).
fn field_key(tok: &Tok) -> Option<Rc<str>> {
    match tok {
        Tok::Ident(s) | Tok::Ctor(s) | Tok::Reserved(s) => Some(s.name()),
        Tok::Str(parts) if parts.iter().all(|p| matches!(p, StrPart::Lit(_))) => {
            Some(Rc::from(parts.iter().map(|p| if let StrPart::Lit(s) = p { s.as_str() } else { "" }).collect::<String>()))
        }
        other => lexer::keyword_text(other).map(Rc::from),
    }
}

/// `-n` for a whole number written in the code: `-9223372036854775808` is
/// the smallest `Int`, and the opposite of a larger number may fit in one.
fn negated(kind: ExprKind) -> ExprKind {
    let n = match kind {
        ExprKind::Int(n) => match n.checked_neg() {
            Some(m) => return ExprKind::Int(m),
            None => BigInt::from_i64(n),
        },
        ExprKind::Big(n) => (*n).clone(),
        other => return other,
    };
    match bigint::value(n.neg()) {
        Value::Int(m) => ExprKind::Int(m),
        Value::Big(m) => ExprKind::Big(m),
        _ => unreachable!(),
    }
}

/// `call(args)` followed by a block: the block is its last argument.
fn with_block(e: Expr, block: Expr) -> Expr {
    let span = e.span.to(block.span);
    let kind = match e.kind {
        ExprKind::Call(f, mut args) => {
            args.push(block);
            ExprKind::Call(f, args)
        }
        ExprKind::Method(recv, name, mut args, at) => {
            args.push(block);
            ExprKind::Method(recv, name, args, at)
        }
        // `xs.each do |x| ... end` is `xs.each(fn(x) = ...)`.
        ExprKind::Field(recv, name, at) => ExprKind::Method(recv, Sym::new(&name), vec![block], at),
        kind => ExprKind::Call(Box::new(Expr::new(kind, e.span)), vec![block]),
    };
    Expr::new(kind, span)
}

/// A number written in the code, `5` or `-2.5` (the parser folds the sign), as the bound of a range.
fn expr_num(e: &Expr) -> Option<Num> {
    match &e.kind {
        ExprKind::Int(n) => Some(Num::Int(*n)),
        ExprKind::Float(f) => Some(Num::Float(*f)),
        _ => None,
    }
}

/// A bound of a range pattern: a number written in the code.
fn range_bound(p: &Pattern) -> Fallible<Num> {
    match p.kind {
        PatKind::Int(n) => Ok(Num::Int(n)),
        PatKind::Float(f) => Ok(Num::Float(f)),
        PatKind::Big(_) => Err(Diagnostic::error(
            "E001",
            t!(
                "the ends of a range in a pattern are whole numbers of 64 bits at most",
                "les bouts d'un intervalle dans un motif sont des nombres entiers de 64 bits au plus"
            ),
        )
        .at(p.span)
        .label(t!("too large for a range", "trop grand pour un intervalle"))
        .help(t!(
            "compare with a guard instead: `n if n >= low and n <= high -> ...`; a range still matches large numbers between its ends, and a decimal end like `1.0e30` works too",
            "comparez plutôt avec une condition : `n if n >= bas and n <= haut -> ...` ; un intervalle accepte quand même les grands nombres entre ses bouts, et un bout décimal comme `1.0e30` marche aussi"
        ))
        .boxed()),
        _ => Err(Diagnostic::error(
            "E001",
            t!("a range in a pattern goes from a number to a number", "un intervalle dans un motif va d'un nombre à un nombre"),
        )
        .at(p.span)
        .label(t!("not a number", "pas un nombre"))
        .help(range_help())
        .boxed()),
    }
}

fn range_help() -> String {
    t!(
        "write the numbers themselves, like `1..9` or `0.0..0.5`; to compare with names, use a guard: `n if n >= low and n <= high -> ...`",
        "écrivez les nombres eux-mêmes, comme `1..9` ou `0.0..0.5` ; pour comparer avec des noms, utilisez une condition : `n if n >= bas and n <= haut -> ...`"
    )
}

fn prefix_help() -> String {
    t!(
        "a text that starts with a text is written `\"Mr \" + name`: `name` is the rest of the text",
        "un texte qui commence par un texte s'écrit `\"M. \" + nom` : `nom` est le reste du texte"
    )
}

fn binary(op: BinOp, l: Expr, r: Expr, op_span: Span) -> Expr {
    let span = l.span.to(r.span);
    Expr::new(ExprKind::Binary(op, Box::new(l), Box::new(r), op_span), span)
}

fn missing_end(at: Span, opener: Span, what: &str) -> Box<Diagnostic> {
    Diagnostic::error("E020", t!("missing `end` to close this `{what}`", "il manque `end` pour fermer ce `{what}`"))
        .at(opener)
        .label(t!("this `{what}` is never closed", "ce `{what}` n'est jamais fermé"))
        .secondary(at, t!("the code ends here", "le code se termine ici"))
        .help(t!(
            "add `end` on its own line after the last line of the block",
            "ajoutez `end` sur sa propre ligne après la dernière ligne du bloc"
        ))
        .incomplete()
        .boxed()
}

pub fn immutable_help() -> String {
    t!(
        "values never change in Rythme: give the new value a new name, like `total2 = total + 1`",
        "les valeurs ne changent jamais en Rythme : donnez un nouveau nom à la nouvelle valeur, comme `total2 = total + 1`"
    )
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}
