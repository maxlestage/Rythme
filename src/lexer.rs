//! Turns source text into tokens.

use std::collections::HashMap;

use crate::bigint::{self, BigInt};
use crate::diagnostic::{Diagnostic, Fallible};
use crate::source::Span;
use crate::suggest;
use crate::symbol::Sym;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Int(i64),
    /// A whole number too large for 64 bits.
    Big(std::rc::Rc<BigInt>),
    Float(f64),
    Str(Vec<StrPart>),
    /// A lowercase name: `x`, `print`, `_`.
    Ident(Sym),
    /// An uppercase name: a constructor such as `Some` or `Circle`.
    Ctor(Sym),
    /// A keyword from another language (`for`, `return`...), reserved to give better help.
    Reserved(Sym),
    /// A symbol: `:ok`, `:error` (the name, without the `:`).
    Symbol(Sym),
    // Keywords
    Fn,
    If,
    Then,
    Else,
    Match,
    End,
    Do,
    Type,
    Test,
    Use,
    And,
    Or,
    Not,
    True,
    False,
    // Punctuation
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Dot,
    DotDot,
    Colon,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,
    Percent,
    StarStar,
    Bar,
    PipeGt,
    Arrow,
    Question,
    // Tokens that do not exist in Rythme but get a helpful message
    FatArrow,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    AmpAmp,
    BarBar,
    Bang,
    Newline,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrPart {
    Lit(String),
    /// Tokens of an interpolated `{expression}`, ending with `Eof`.
    Code(Vec<Token>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
}

pub struct Lexed {
    pub tokens: Vec<Token>,
    /// Documentation comments (`## ...`), keyed by the index of the token they precede.
    pub docs: HashMap<usize, String>,
}

pub const KEYWORDS: &[&str] =
    &["fn", "if", "then", "else", "match", "end", "do", "type", "test", "use", "and", "or", "not", "true", "false"];

/// Keywords of other languages. Using them gives a message explaining the Rythme way.
pub const RESERVED: &[&str] = &[
    "for", "while", "return", "break", "continue", "let", "var", "const", "def", "function", "class", "elif", "elsif", "import", "try",
    "catch", "except", "raise", "throw", "null", "nil", "lambda", "switch",
];

fn keyword(s: &str) -> Option<Tok> {
    Some(match s {
        "fn" => Tok::Fn,
        "if" => Tok::If,
        "then" => Tok::Then,
        "else" => Tok::Else,
        "match" => Tok::Match,
        "end" => Tok::End,
        "do" => Tok::Do,
        "type" => Tok::Type,
        "test" => Tok::Test,
        "use" => Tok::Use,
        "and" => Tok::And,
        "or" => Tok::Or,
        "not" => Tok::Not,
        "true" => Tok::True,
        "false" => Tok::False,
        _ => return None,
    })
}

/// The keyword text of a token, if it is a keyword (used for record keys like `{type: 1}`).
pub fn keyword_text(tok: &Tok) -> Option<&'static str> {
    Some(match tok {
        Tok::Fn => "fn",
        Tok::If => "if",
        Tok::Then => "then",
        Tok::Else => "else",
        Tok::Match => "match",
        Tok::End => "end",
        Tok::Do => "do",
        Tok::Type => "type",
        Tok::Test => "test",
        Tok::Use => "use",
        Tok::And => "and",
        Tok::Or => "or",
        Tok::Not => "not",
        Tok::True => "true",
        Tok::False => "false",
        _ => return None,
    })
}

pub fn lex(src: &str, file: u32) -> Fallible<Lexed> {
    // Some editors (Notepad) start files with an invisible byte order mark.
    let pos = if src.starts_with('\u{FEFF}') { '\u{FEFF}'.len_utf8() } else { 0 };
    let mut lx = Lexer { src, pos, file, docs: HashMap::new(), pending_doc: Vec::new(), depth: 0 };
    let tokens = lx.tokens(None)?;
    Ok(Lexed { tokens, docs: lx.docs })
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    file: u32,
    docs: HashMap<usize, String>,
    pending_doc: Vec<String>,
    /// How many texts are being read inside `{...}` of other texts.
    depth: usize,
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Can `:` and this text be written as a symbol (`:name`)?
pub fn is_symbol_name(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(is_ident_start) && chars.all(is_ident_continue)
}

impl<'a> Lexer<'a> {
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn peek_at(&self, n: usize) -> Option<char> {
        self.src[self.pos..].chars().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// The text of the current line, from its start up to `pos`.
    fn line_before(&self, pos: usize) -> &str {
        let line_start = self.src[..pos].rfind('\n').map_or(0, |i| i + 1);
        &self.src[line_start..pos]
    }

    fn span(&self, start: usize) -> Span {
        Span::new(self.file, start, self.pos)
    }

    fn error(&self, code: &'static str, start: usize, msg: String) -> Diagnostic {
        Diagnostic::error(code, msg).at(Span::new(self.file, start, self.pos.max(start + 1)))
    }

    /// Lexes tokens until the end of the text, or (inside `{...}` in a text,
    /// `brace` being the position of the `{`) until the closing brace.
    fn tokens(&mut self, brace: Option<usize>) -> Fallible<Vec<Token>> {
        let interpolation = brace.is_some();
        let mut toks: Vec<Token> = Vec::new();
        let mut depth = 0usize;
        loop {
            while let Some(c) = self.peek() {
                if c == ' ' || c == '\t' || c == '\r' {
                    self.bump();
                } else {
                    break;
                }
            }
            let start = self.pos;
            let Some(c) = self.bump() else {
                if let Some(brace) = brace {
                    return Err(self.unclosed_brace(brace).boxed());
                }
                toks.push(Token { tok: Tok::Eof, span: Span::new(self.file, start, start) });
                return Ok(toks);
            };
            let tok = match c {
                '\n' | ';' => {
                    // A blank line separates a `##` comment from what follows.
                    if c == '\n' && self.line_before(start).trim().is_empty() {
                        self.pending_doc.clear();
                    }
                    if !matches!(toks.last(), None | Some(Token { tok: Tok::Newline, .. })) {
                        toks.push(Token { tok: Tok::Newline, span: self.span(start) });
                    }
                    continue;
                }
                '#' => {
                    // `##` documents the next definition only when it starts its line.
                    let doc = self.peek() == Some('#') && self.line_before(start).trim().is_empty();
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                    if doc && !interpolation {
                        let text = self.src[start..self.pos].trim_start_matches('#');
                        self.pending_doc.push(text.strip_prefix(' ').unwrap_or(text).trim_end().to_string());
                    }
                    continue;
                }
                '0'..='9' => self.number(start)?,
                '"' => self.string(start)?,
                c if is_ident_start(c) => {
                    while self.peek().is_some_and(is_ident_continue) {
                        self.bump();
                    }
                    let word = &self.src[start..self.pos];
                    if let Some(k) = keyword(word) {
                        k
                    } else if RESERVED.contains(&word) {
                        Tok::Reserved(Sym::new(word))
                    } else if c.is_uppercase() {
                        Tok::Ctor(Sym::new(word))
                    } else {
                        Tok::Ident(Sym::new(word))
                    }
                }
                '(' => Tok::LParen,
                ')' => Tok::RParen,
                '[' => Tok::LBracket,
                ']' => Tok::RBracket,
                '{' => {
                    depth += 1;
                    Tok::LBrace
                }
                '}' => {
                    if interpolation && depth == 0 {
                        toks.push(Token { tok: Tok::Eof, span: self.span(start) });
                        return Ok(toks);
                    }
                    depth = depth.saturating_sub(1);
                    Tok::RBrace
                }
                ',' => Tok::Comma,
                ':' if self.peek().is_some_and(is_ident_start) && !key_before(&toks, interpolation) => {
                    while self.peek().is_some_and(is_ident_continue) {
                        self.bump();
                    }
                    Tok::Symbol(Sym::new(&self.src[start + 1..self.pos]))
                }
                ':' => Tok::Colon,
                '?' => Tok::Question,
                '%' => Tok::Percent,
                '.' => {
                    if self.peek() == Some('.') {
                        self.bump();
                        if self.peek() == Some('.') {
                            self.bump();
                            return Err(self
                                .error("E018", start, t!("`...` is not an operator in Rythme", "`...` n'est pas un opérateur en Rythme"))
                                .help(t!(
                                    "write `1..10` for a range (10 included), or `[first, ..rest]` in a list",
                                    "écrivez `1..10` pour un intervalle (10 inclus), ou `[premier, ..reste]` dans une liste"
                                ))
                                .boxed());
                        }
                        Tok::DotDot
                    } else {
                        Tok::Dot
                    }
                }
                '=' => match self.peek() {
                    Some('=') => {
                        self.bump();
                        Tok::EqEq
                    }
                    Some('>') => {
                        self.bump();
                        Tok::FatArrow
                    }
                    _ => Tok::Eq,
                },
                '!' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        Tok::NotEq
                    } else {
                        Tok::Bang
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        Tok::Le
                    } else {
                        Tok::Lt
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.bump();
                        Tok::Ge
                    } else {
                        Tok::Gt
                    }
                }
                '+' => match self.peek() {
                    Some('=') => {
                        self.bump();
                        Tok::PlusEq
                    }
                    Some('+') => {
                        self.bump();
                        return Err(self
                            .error("E018", start, t!("`++` is not an operator in Rythme", "`++` n'est pas un opérateur en Rythme"))
                            .help(t!(
                                "to add one, write `x + 1` and give the result a new name (values never change);\nto join two lists or two texts, use `+`: `[1] + [2]`",
                                "pour ajouter un, écrivez `x + 1` et donnez un nouveau nom au résultat (les valeurs ne changent jamais) ;\npour joindre deux listes ou deux textes, utilisez `+` : `[1] + [2]`"
                            ))
                            .boxed());
                    }
                    _ => Tok::Plus,
                },
                '-' => match self.peek() {
                    Some('>') => {
                        self.bump();
                        Tok::Arrow
                    }
                    Some('=') => {
                        self.bump();
                        Tok::MinusEq
                    }
                    _ => Tok::Minus,
                },
                '*' => match self.peek() {
                    Some('*') => {
                        self.bump();
                        Tok::StarStar
                    }
                    Some('=') => {
                        self.bump();
                        Tok::StarEq
                    }
                    _ => Tok::Star,
                },
                '/' => match self.peek() {
                    Some('/') => {
                        self.bump();
                        Tok::SlashSlash
                    }
                    Some('=') => {
                        self.bump();
                        Tok::SlashEq
                    }
                    _ => Tok::Slash,
                },
                '|' => match self.peek() {
                    Some('>') => {
                        self.bump();
                        Tok::PipeGt
                    }
                    Some('|') => {
                        self.bump();
                        Tok::BarBar
                    }
                    _ => Tok::Bar,
                },
                '&' => {
                    if self.peek() == Some('&') {
                        self.bump();
                        Tok::AmpAmp
                    } else {
                        return Err(self
                            .error("E018", start, t!("unexpected character `&`", "caractère inattendu `&`"))
                            .help(t!("use `and` to combine two conditions", "utilisez `and` pour combiner deux conditions"))
                            .boxed());
                    }
                }
                other => return Err(self.unexpected_char(other, start).boxed()),
            };
            if !interpolation && !self.pending_doc.is_empty() {
                self.docs.insert(toks.len(), self.pending_doc.join("\n"));
                self.pending_doc.clear();
            }
            toks.push(Token { tok, span: self.span(start) });
        }
    }

    /// The `{` at `brace`, which starts a value inserted in a text, is never closed.
    fn unclosed_brace(&self, brace: usize) -> Diagnostic {
        Diagnostic::error("E002", t!("this `{{` in a text is never closed", "cette `{{` dans un texte n'est jamais fermée"))
            .at(Span::new(self.file, brace, brace + 1))
            .label(t!("starts a value inserted in the text", "commence une valeur insérée dans le texte"))
            .help(t!(
                "close it with `}}`, or write `\\{{` to get a literal brace: \"\\{{\"",
                "fermez-la avec `}}`, ou écrivez `\\{{` pour obtenir une accolade : \"\\{{\""
            ))
            .incomplete()
    }

    fn unexpected_char(&self, c: char, start: usize) -> Diagnostic {
        // Spaces and marks that cannot be seen, often copied from a document or a web page.
        let space = matches!(c, '\u{a0}' | '\u{2000}'..='\u{200a}' | '\u{202f}' | '\u{205f}' | '\u{3000}');
        let invisible =
            c.is_control() || matches!(c, '\u{ad}' | '\u{200b}'..='\u{200f}' | '\u{2028}'..='\u{202e}' | '\u{2060}' | '\u{feff}');
        if space || invisible {
            let code = format!("U+{:04X}", c as u32);
            let d = self.error("E018", start, t!("unexpected invisible character {code}", "caractère invisible inattendu {code}"));
            return d.help(if space {
                t!(
                    "this is a special space (often copied from a document or a web page): replace it with a normal space",
                    "c'est une espace spéciale (souvent copiée depuis un document ou une page web) : remplacez-la par une espace normale"
                )
            } else {
                t!(
                    "this character cannot be seen (it is often copied from a document or a web page): delete it",
                    "ce caractère ne se voit pas (il est souvent copié depuis un document ou une page web) : supprimez-le"
                )
            });
        }
        let d = self.error("E018", start, t!("unexpected character `{c}`", "caractère inattendu `{c}`"));
        let instead = |s: &str| d.clone().help(t!("write `{s}` instead", "écrivez plutôt `{s}`"));
        match c {
            '\'' | '‘' | '’' | '‚' => {
                d.help(t!("texts use double quotes: \"hello\"", "les textes s'écrivent entre guillemets droits : \"bonjour\""))
            }
            '^' => d.help(t!("use `**` for powers: `2 ** 10`", "utilisez `**` pour les puissances : `2 ** 10`")),
            '`' => d.help(t!("texts use double quotes: \"hello\"", "les textes s'écrivent entre guillemets droits : \"bonjour\"")),
            '“' | '”' | '„' | '«' | '»' => {
                d.help(t!("use straight double quotes `\"` for texts", "utilisez des guillemets droits `\"` pour les textes"))
            }
            '−' | '–' | '—' => instead("-"),
            '×' | '·' | '∗' => instead("*"),
            '÷' | '∕' => instead("/"),
            '≠' => instead("!="),
            '≤' => instead("<="),
            '≥' => instead(">="),
            '→' => instead("->"),
            // Full-width forms of `,` `(` `)`..., typed with an Asian keyboard layout.
            '\u{ff01}'..='\u{ff5e}' => instead(&char::from_u32(c as u32 - 0xfee0).unwrap_or(c).to_string()),
            '$' => d.help(t!(
                "to insert a value in a text, use braces: \"total: {{x}}\"",
                "pour insérer une valeur dans un texte, utilisez des accolades : \"total : {{x}}\""
            )),
            _ => d,
        }
    }

    fn number(&mut self, start: usize) -> Fallible<Tok> {
        let first = self.src.as_bytes()[start];
        if first == b'0' && matches!(self.peek(), Some('x') | Some('b')) {
            let radix = if self.bump() == Some('x') { 16 } else { 2 };
            let digits_start = self.pos;
            while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
                self.bump();
            }
            let digits: String = self.src[digits_start..self.pos].chars().filter(|&c| c != '_').collect();
            let text = &self.src[start..self.pos];
            if !digits.is_empty() && digits.chars().all(|c| c.is_digit(radix)) {
                if let Ok(n) = i64::from_str_radix(&digits, radix) {
                    return Ok(Tok::Int(n));
                }
                return big_literal(&digits, radix, self.span(start));
            }
            return Err(self
                .error("E018", start, t!("invalid number `{text}`", "nombre invalide `{text}`"))
                .help(t!(
                    "hexadecimal numbers use the digits 0 to 9 and a to f, like `0xff`; binary numbers use 0 and 1, like `0b101`",
                    "les nombres hexadécimaux utilisent les chiffres 0 à 9 et a à f, comme `0xff` ; les binaires 0 et 1, comme `0b101`"
                ))
                .boxed());
        }
        while self.peek().is_some_and(|c| c.is_ascii_digit() || c == '_') {
            self.bump();
        }
        let mut float = false;
        if self.peek() == Some('.') && self.peek_at(1).is_some_and(|c| c.is_ascii_digit()) {
            float = true;
            self.bump();
            while self.peek().is_some_and(|c| c.is_ascii_digit() || c == '_') {
                self.bump();
            }
        }
        if matches!(self.peek(), Some('e') | Some('E')) {
            let next = self.peek_at(1);
            let exp = next.is_some_and(|c| c.is_ascii_digit())
                || (matches!(next, Some('+') | Some('-')) && self.peek_at(2).is_some_and(|c| c.is_ascii_digit()));
            if exp {
                float = true;
                self.bump();
                self.bump();
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.bump();
                }
            }
        }
        if let Some(c) = self.peek() {
            if is_ident_start(c) {
                let num = self.src[start..self.pos].to_string();
                let word_start = self.pos;
                while self.peek().is_some_and(is_ident_continue) {
                    self.bump();
                }
                let word = &self.src[word_start..self.pos];
                return Err(self
                    .error(
                        "E018",
                        start,
                        t!(
                            "a name cannot start with a digit: `{}`",
                            "un nom ne peut pas commencer par un chiffre : `{}`",
                            &self.src[start..self.pos]
                        ),
                    )
                    .help(t!(
                        "to multiply, write it explicitly: `{num} * {word}`",
                        "pour multiplier, écrivez-le explicitement : `{num} * {word}`"
                    ))
                    .boxed());
            }
        }
        let text: String = self.src[start..self.pos].chars().filter(|&c| c != '_').collect();
        if float {
            Ok(Tok::Float(text.parse().unwrap_or(f64::INFINITY)))
        } else if let Ok(n) = text.parse::<i64>() {
            Ok(Tok::Int(n))
        } else {
            big_literal(&text, 10, self.span(start))
        }
    }

    fn string(&mut self, start: usize) -> Fallible<Tok> {
        if self.depth >= 32 {
            return Err(self
                .error(
                    "E001",
                    start,
                    t!("texts are nested too deeply inside `{{...}}`", "les textes sont imbriqués trop profondément dans des `{{...}}`"),
                )
                .boxed());
        }
        self.depth += 1;
        let r = self.string_inner(start);
        self.depth -= 1;
        r
    }

    fn string_inner(&mut self, start: usize) -> Fallible<Tok> {
        let mut parts = Vec::new();
        let mut cur = String::new();
        loop {
            let at = self.pos;
            let Some(c) = self.bump() else {
                return Err(Diagnostic::error("E002", t!("this text is never closed", "ce texte n'est jamais fermé"))
                    .at(Span::new(self.file, start, start + 1))
                    .label(t!("the text starts here", "le texte commence ici"))
                    .help(t!("add a `\"` at the end of the text", "ajoutez un `\"` à la fin du texte"))
                    .incomplete()
                    .boxed());
            };
            match c {
                '"' => break,
                '\\' => {
                    let esc = self.bump();
                    match esc {
                        Some('n') => cur.push('\n'),
                        Some('t') => cur.push('\t'),
                        Some('r') => cur.push('\r'),
                        Some('0') => cur.push('\0'),
                        Some('\\') => cur.push('\\'),
                        Some('"') => cur.push('"'),
                        Some('{') => cur.push('{'),
                        Some('}') => cur.push('}'),
                        // `\#{x}` is written as it is, as in Ruby.
                        Some('#') => {
                            cur.push('#');
                            if self.peek() == Some('{') {
                                self.bump();
                                cur.push('{');
                            }
                        }
                        Some('u') if self.peek() == Some('{') => {
                            self.bump();
                            let hex_start = self.pos;
                            while self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
                                self.bump();
                            }
                            let hex = &self.src[hex_start..self.pos];
                            let ch = u32::from_str_radix(hex, 16).ok().and_then(char::from_u32);
                            match (ch, self.bump()) {
                                (Some(ch), Some('}')) => cur.push(ch),
                                _ => {
                                    return Err(self
                                        .error("E018", at, t!("invalid unicode escape", "code unicode invalide"))
                                        .help(t!("write it like `\\u{{e9}}` for é", "écrivez-le comme `\\u{{e9}}` pour é"))
                                        .boxed())
                                }
                            }
                        }
                        _ => {
                            let mut d = self.error(
                                "E018",
                                at,
                                t!("unknown escape sequence in text", "séquence d'échappement inconnue dans le texte"),
                            );
                            // `"\d+"` for a regex: the backslash must be written twice.
                            if let Some(c) = esc.filter(|c| "dDwWsSbB.()[]*+?|^$-/".contains(*c)) {
                                d = d.help(t!(
                                    "in a regex, write the backslash twice: \"\\\\{c}\" is the text \\{c}",
                                    "dans une regex, écrivez la barre oblique inverse deux fois : \"\\\\{c}\" est le texte \\{c}"
                                ));
                            }
                            return Err(d
                                .help(t!(
                                    "valid escapes: \\n (new line), \\t (tab), \\r (carriage return), \\0 (null character), \\\\ (backslash),\n\\\" (quote), \\{{ and \\}} (braces), \\# (a `#` before a brace), \\u{{e9}} (a character by its Unicode number: é)",
                                    "échappements valides : \\n (nouvelle ligne), \\t (tabulation), \\r (retour chariot), \\0 (caractère nul), \\\\ (barre oblique),\n\\\" (guillemet), \\{{ et \\}} (accolades), \\# (un `#` devant une accolade), \\u{{e9}} (un caractère par son numéro Unicode : é)"
                                ))
                                .boxed());
                        }
                    }
                }
                // `#{x}`, as in Ruby, is the same as `{x}`.
                '{' | '#' if c == '{' || self.peek() == Some('{') => {
                    if c == '#' {
                        self.bump();
                    }
                    if !cur.is_empty() {
                        parts.push(StrPart::Lit(std::mem::take(&mut cur)));
                    }
                    let code = match self.tokens(Some(at)) {
                        // A text started inside `{...}` reached the end of the code: the
                        // `{` was probably meant literally, as in "{" or "{name: ".
                        Err(d) if d.incomplete && d.span.is_some_and(|s| s.start as usize != at) => {
                            let mut e = self.unclosed_brace(at);
                            if let Some(inner) = d.span {
                                e = e.secondary(
                                    inner,
                                    t!("inside it, this text is never closed", "à l'intérieur, ce texte n'est jamais fermé"),
                                );
                            }
                            return Err(e.boxed());
                        }
                        Err(mut d) => {
                            // (An unclosed `{` already says how to write a brace.)
                            if d.span.is_some_and(|s| s.start as usize != at) {
                                add_literal_brace_help(&mut d);
                            }
                            return Err(d);
                        }
                        Ok(code) => code,
                    };
                    if code.len() == 1 {
                        return Err(self
                            .error("E001", at, t!("empty `{{}}` in a text", "`{{}}` vide dans un texte"))
                            .help(t!(
                                "put a value inside, like \"{{name}}\", or write `\\{{` for a literal brace",
                                "mettez une valeur à l'intérieur, comme \"{{nom}}\", ou écrivez `\\{{` pour une accolade"
                            ))
                            .boxed());
                    }
                    parts.push(StrPart::Code(code));
                }
                // Windows line ends inside a text become plain new lines.
                '\r' if self.peek() == Some('\n') => {}
                c => cur.push(c),
            }
        }
        if !cur.is_empty() || parts.is_empty() {
            parts.push(StrPart::Lit(cur));
        }
        Ok(Tok::Str(parts))
    }
}

/// Is a `:` read now the one between a field name and its value, as in
/// `{name:value}`? It is when it follows a name (or a keyword, or a text)
/// that itself follows `{` or `,`. Otherwise, `:name` is a symbol: `:ok`.
fn key_before(toks: &[Token], interpolation: bool) -> bool {
    let key = |t: &Tok| matches!(t, Tok::Ident(_) | Tok::Ctor(_) | Tok::Reserved(_) | Tok::Str(_)) || keyword_text(t).is_some();
    // Records may be written on several lines.
    let mut before = toks.iter().rev().filter(|t| t.tok != Tok::Newline).map(|t| &t.tok);
    match (before.next(), before.next()) {
        (Some(last), Some(before)) => key(last) && matches!(before, Tok::LBrace | Tok::Comma),
        // `"{name:value}"`: the `{` of the text comes before.
        (Some(last), None) => key(last) && interpolation,
        _ => false,
    }
}

/// For a mistake inside the `{...}` of a text: maybe the brace was meant literally.
pub fn add_literal_brace_help(d: &mut Diagnostic) {
    let help = t!(
        "inside a text, `{{` starts an inserted value: to write braces, use `\\{{` and `\\}}`, like \"\\{{\\\"a\\\": 1\\}}\"",
        "dans un texte, `{{` commence une valeur insérée : pour écrire des accolades, utilisez `\\{{` et `\\}}`, comme \"\\{{\\\"a\\\": 1\\}}\""
    );
    if !d.helps.contains(&help) {
        d.helps.push(help);
    }
}

/// A whole number written in the code that does not fit in 64 bits.
fn big_literal(digits: &str, radix: u32, span: Span) -> Fallible<Tok> {
    match BigInt::parse(digits, radix) {
        Some(Ok(n)) => Ok(Tok::Big(std::rc::Rc::new(n))),
        _ => Err(bigint::too_large(span)),
    }
}

/// Human description of a token for error messages.
pub fn describe(tok: &Tok) -> String {
    match tok {
        Tok::Int(n) => t!("the number `{n}`", "le nombre `{n}`"),
        Tok::Big(n) => t!("the number `{n}`", "le nombre `{n}`"),
        Tok::Float(f) => t!("the number `{f}`", "le nombre `{f}`"),
        Tok::Str(_) => t!("a text", "un texte"),
        Tok::Ident(s) | Tok::Ctor(s) | Tok::Reserved(s) => t!("the name `{s}`", "le nom `{s}`"),
        Tok::Symbol(s) => t!("the symbol `:{s}`", "le symbole `:{s}`"),
        Tok::Newline => t!("the end of the line", "la fin de la ligne"),
        Tok::Eof => t!("the end of the code", "la fin du code"),
        other => {
            if let Some(k) = keyword_text(other) {
                return t!("the keyword `{k}`", "le mot-clé `{k}`");
            }
            format!("`{}`", symbol_text(other))
        }
    }
}

pub fn symbol_text(tok: &Tok) -> &'static str {
    match tok {
        Tok::LParen => "(",
        Tok::RParen => ")",
        Tok::LBracket => "[",
        Tok::RBracket => "]",
        Tok::LBrace => "{",
        Tok::RBrace => "}",
        Tok::Comma => ",",
        Tok::Dot => ".",
        Tok::DotDot => "..",
        Tok::Colon => ":",
        Tok::Eq => "=",
        Tok::EqEq => "==",
        Tok::NotEq => "!=",
        Tok::Lt => "<",
        Tok::Le => "<=",
        Tok::Gt => ">",
        Tok::Ge => ">=",
        Tok::Plus => "+",
        Tok::Minus => "-",
        Tok::Star => "*",
        Tok::Slash => "/",
        Tok::SlashSlash => "//",
        Tok::Percent => "%",
        Tok::StarStar => "**",
        Tok::Bar => "|",
        Tok::PipeGt => "|>",
        Tok::Arrow => "->",
        Tok::Question => "?",
        Tok::FatArrow => "=>",
        Tok::PlusEq => "+=",
        Tok::MinusEq => "-=",
        Tok::StarEq => "*=",
        Tok::SlashEq => "/=",
        Tok::AmpAmp => "&&",
        Tok::BarBar => "||",
        Tok::Bang => "!",
        _ => "?",
    }
}

/// Help for a keyword borrowed from another language.
pub fn reserved_help(word: &str) -> String {
    suggest::foreign_keyword_hint(word).unwrap_or_default()
}
