//! CSV, the tables of spreadsheets: `csv_parse`, `csv_records` and
//! `csv_format`, as RFC 4180 describes them. A field may be written between
//! quotes, and then holds separators, line breaks and quotes (written twice);
//! lines end with `\n` or `\r\n`; the last line may end with a line break or
//! not; empty lines are skipped. A table that cannot be read is an `Err`
//! whose text says where (line and column) and why: never a crash.

use super::*;

/// The separator when none is given.
pub const COMMA: char = ',';

/// Why a CSV text cannot be read, and where (`line` and `column` count from 1,
/// in characters; `column` is 0 for a mistake about a whole line).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsvError {
    pub line: usize,
    pub column: usize,
    pub kind: CsvErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CsvErrorKind {
    /// The quote that opens a field at this place is never closed.
    Unclosed,
    /// A quote in a field that does not start with one.
    QuoteInside,
    /// Something else than a separator or the end of the line after the quote that closes a field.
    AfterQuote(char),
    /// A name of column given twice in the header (`csv_records`).
    SameColumn(String),
    /// A line without as many fields as the header (`csv_records`): it has `.0`, the header `.1`.
    Fields(usize, usize),
    /// More rows or fields than a list can hold.
    TooLarge,
}

impl CsvError {
    /// The text of the error, in the language of the messages.
    pub fn message(&self, sep: char) -> String {
        let (line, column) = (self.line, self.column);
        let sep = shown(sep);
        match &self.kind {
            CsvErrorKind::Unclosed => t!(
                "line {line}, column {column}: the quote that starts this field is never closed (a quote inside a field is written twice: \"\")",
                "ligne {line}, colonne {column} : le guillemet qui commence ce champ n'est jamais fermé (un guillemet dans un champ s'écrit deux fois : \"\")"
            ),
            CsvErrorKind::QuoteInside => t!(
                "line {line}, column {column}: a quote in the middle of a field: put the whole field between quotes, and write each quote inside it twice (\"5\"\" screen\")",
                "ligne {line}, colonne {column} : un guillemet au milieu d'un champ : mettez tout le champ entre guillemets, et écrivez deux fois chaque guillemet qu'il contient (\"écran 5\"\"\")"
            ),
            CsvErrorKind::AfterQuote(c) => {
                let c = shown(*c);
                t!(
                    "line {line}, column {column}: after the quote that closes a field comes {sep} or the end of the line, not {c}",
                    "ligne {line}, colonne {column} : après le guillemet qui ferme un champ vient {sep} ou la fin de la ligne, pas {c}"
                )
            }
            CsvErrorKind::SameColumn(name) => t!(
                "line {line}: the header names the column \"{name}\" twice",
                "ligne {line} : l'en-tête nomme deux fois la colonne \"{name}\""
            ),
            CsvErrorKind::Fields(got, wanted) => t!(
                "line {line}: {got} field(s), but the header has {wanted} column(s)",
                "ligne {line} : {got} champ(s), mais l'en-tête a {wanted} colonne(s)"
            ),
            CsvErrorKind::TooLarge => t!(
                "line {line}: this table has too many rows or fields (more than {MAX_LIST})",
                "ligne {line} : ce tableau a trop de lignes ou de champs (plus de {MAX_LIST})"
            ),
        }
    }
}

/// A character in a message: `;`, or its name when it cannot be seen.
fn shown(c: char) -> String {
    match c {
        '\t' => t!("a tab", "une tabulation"),
        ' ' => t!("a space", "un espace"),
        c if c.is_control() => format!("`{}`", c.escape_default()),
        c => format!("`{c}`"),
    }
}

/// A row of a table, with the line where it starts.
pub struct Row {
    pub line: usize,
    pub fields: Vec<String>,
}

/// Reads a CSV text with the separator `sep` (never a quote or a line break).
pub fn parse(s: &str, sep: char) -> Result<Vec<Row>, CsvError> {
    // Some programs start their files with an invisible mark (BOM).
    let s = s.strip_prefix('\u{feff}').unwrap_or(s);
    let mut it = s.chars().peekable();
    let (mut line, mut column) = (1, 1);
    let mut rows = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut row_line = 1;
    loop {
        if row.is_empty() {
            row_line = line;
        }
        let mut field = String::new();
        let quoted = it.peek() == Some(&'"');
        if quoted {
            let (open_line, open_column) = (line, column);
            it.next();
            column += 1;
            loop {
                match it.next() {
                    None => return Err(CsvError { line: open_line, column: open_column, kind: CsvErrorKind::Unclosed }),
                    Some('"') => {
                        column += 1;
                        if it.peek() == Some(&'"') {
                            it.next();
                            column += 1;
                            field.push('"');
                        } else {
                            break;
                        }
                    }
                    Some('\n') => {
                        field.push('\n');
                        (line, column) = (line + 1, 1);
                    }
                    Some('\r') => {
                        field.push('\r');
                        if it.peek() == Some(&'\n') {
                            it.next();
                            field.push('\n');
                        }
                        (line, column) = (line + 1, 1);
                    }
                    Some(c) => {
                        field.push(c);
                        column += 1;
                    }
                }
            }
            match it.peek() {
                None | Some('\n' | '\r') => {}
                Some(&c) if c == sep => {}
                Some(&c) => return Err(CsvError { line, column, kind: CsvErrorKind::AfterQuote(c) }),
            }
        } else {
            loop {
                match it.peek() {
                    None | Some('\n' | '\r') => break,
                    Some(&c) if c == sep => break,
                    Some('"') => return Err(CsvError { line, column, kind: CsvErrorKind::QuoteInside }),
                    Some(&c) => {
                        field.push(c);
                        it.next();
                        column += 1;
                    }
                }
            }
        }
        // An empty line is no row; a line with only `""` is a row of one empty field.
        let blank = row.is_empty() && field.is_empty() && !quoted;
        row.push(field);
        if row.len() > MAX_LIST || rows.len() >= MAX_LIST {
            return Err(CsvError { line: row_line, column: 0, kind: CsvErrorKind::TooLarge });
        }
        let next = it.next();
        match next {
            Some(c) if c == sep => {
                column += 1;
                continue;
            }
            Some('\r') => {
                if it.peek() == Some(&'\n') {
                    it.next();
                }
                (line, column) = (line + 1, 1);
            }
            Some('\n') => (line, column) = (line + 1, 1),
            _ => {}
        }
        let fields = std::mem::take(&mut row);
        if !blank {
            rows.push(Row { line: row_line, fields });
        }
        if next.is_none() {
            return Ok(rows);
        }
    }
}

/// Reads a table whose first row names the columns: the names, all
/// different, and the other rows, each with one field for each column.
pub fn parse_records(s: &str, sep: char) -> Result<(Vec<String>, Vec<Row>), CsvError> {
    let mut rows = parse(s, sep)?.into_iter();
    let Some(header) = rows.next() else { return Ok((Vec::new(), Vec::new())) };
    let mut seen = HashSet::new();
    for name in &header.fields {
        if !seen.insert(name.as_str()) {
            return Err(CsvError { line: header.line, column: 0, kind: CsvErrorKind::SameColumn(name.clone()) });
        }
    }
    let rows: Vec<Row> = rows.collect();
    for r in &rows {
        if r.fields.len() != header.fields.len() {
            return Err(CsvError { line: r.line, column: 0, kind: CsvErrorKind::Fields(r.fields.len(), header.fields.len()) });
        }
    }
    Ok((header.fields, rows))
}

/// Writes one field: between quotes only when it needs them (a separator, a
/// quote or a line break inside; or alone and empty on its line, which
/// would otherwise be an empty line).
pub fn write_field(field: &str, sep: char, alone: bool, out: &mut String) {
    let needs = (alone && field.is_empty()) || field.chars().any(|c| c == sep || matches!(c, '"' | '\n' | '\r'));
    if needs {
        out.push('"');
        out.push_str(&field.replace('"', "\"\""));
        out.push('"');
    } else {
        out.push_str(field);
    }
}

/// Writes rows: each one on its line, ended by `\n`.
pub fn format<S: AsRef<str>>(rows: &[Vec<S>], sep: char) -> String {
    let mut out = String::new();
    for row in rows {
        for (i, field) in row.iter().enumerate() {
            if i > 0 {
                out.push(sep);
            }
            write_field(field.as_ref(), sep, row.len() == 1, &mut out);
        }
        out.push('\n');
    }
    out
}

/// The separator given to `f` as argument `i`, or a comma: one character,
/// neither a quote nor a line break.
fn separator(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<char> {
    if a.len() <= i {
        return Ok(COMMA);
    }
    let s = text(a, i, f, span)?;
    let mut chars = s.chars();
    let msg = match (chars.next(), chars.next()) {
        (Some(c), None) if !matches!(c, '"' | '\n' | '\r') => return Ok(c),
        (Some(c), None) => {
            let c = shown(c);
            t!(
                "`{f}` cannot use {c} as separator: it has another meaning in CSV",
                "`{f}` ne peut pas prendre {c} comme séparateur : il a un autre sens en CSV"
            )
        }
        _ => t!("the separator of `{f}` is one character, not \"{s}\"", "le séparateur de `{f}` est un seul caractère, pas \"{s}\""),
    };
    Err(Diagnostic::error("E023", msg)
        .at(span)
        .help(t!(
            "the usual separators: \",\" (by default), \";\" (French spreadsheets), \"\\t\" (a tab)",
            "les séparateurs habituels : \",\" (par défaut), \";\" (tableurs en français), \"\\t\" (une tabulation)"
        ))
        .boxed())
}

pub(super) fn csv_parse(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "csv_parse", span)?;
    let sep = separator(a, 1, "csv_parse", span)?;
    Ok(match parse(s, sep) {
        Ok(rows) => it.ok(Value::list(rows.into_iter().map(|r| Value::list(r.fields.into_iter().map(text_value).collect())).collect())),
        Err(e) => it.err(text_value(e.message(sep))),
    })
}

pub(super) fn csv_records(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "csv_records", span)?;
    let sep = separator(a, 1, "csv_records", span)?;
    Ok(match parse_records(s, sep) {
        Ok((header, rows)) => {
            // Every record shares the names of the header.
            let shape = Rc::new(Shape::new(header.into_iter().map(Rc::from).collect()));
            let records = rows
                .into_iter()
                .map(|r| Value::Record(Rc::new(Record::with_shape(shape.clone(), r.fields.into_iter().map(text_value).collect()))))
                .collect();
            it.ok(Value::list(records))
        }
        Err(e) => it.err(text_value(e.message(sep))),
    })
}

/// The text of a field given to `csv_format`: a text, a number or a Bool.
fn field_text(v: &Value, row: usize, column: usize, span: Span) -> Fallible<String> {
    match v {
        Value::Text(s) => Ok(s.to_string()),
        Value::Int(_) | Value::Big(_) | Value::Float(_) | Value::Bool(_) => Ok(v.display()),
        _ => {
            let got = v.describe();
            Err(Diagnostic::error(
                "E005",
                t!(
                    "`csv_format`: the field {column} of row {row} must be a text, a number or a Bool, but it is {got}",
                    "`csv_format` : le champ {column} de la ligne {row} doit être un texte, un nombre ou un Bool, mais c'est {got}"
                ),
            )
            .at(span)
            .help(t!(
                "turn it into a text first, with `text(x)` or `unwrap_or(x, \"\")`",
                "transformez-le d'abord en texte, avec `text(x)` ou `unwrap_or(x, \"\")`"
            ))
            .boxed())
        }
    }
}

pub(super) fn csv_format(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let f = "csv_format";
    let rows = list(a, 0, f, span)?;
    let sep = separator(a, 1, f, span)?;
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut size = 0;
    // Records: a header with the names of the fields of the first one, then their values.
    let header: Option<Rc<Record>> = match rows.as_slice().first() {
        Some(Value::Record(r)) => Some(r.clone()),
        _ => None,
    };
    if let Some(first) = &header {
        table.push(first.keys().iter().map(|k| k.to_string()).collect());
    }
    for (n, row) in rows.iter().enumerate() {
        let number = n + 1;
        let fields: Vec<String> = match (row, &header) {
            (Value::List(l), None) => l.iter().enumerate().map(|(i, v)| field_text(v, number, i + 1, span)).collect::<Fallible<_>>()?,
            (Value::Record(r), Some(first)) => {
                let same = r.len() == first.len() && first.keys().iter().all(|k| r.get(k).is_some());
                if !same {
                    let names = |r: &Record| r.keys().iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", ");
                    let (these, wanted) = (names(r), names(first));
                    return Err(invalid(
                        t!(
                            "`csv_format`: record {number} has the fields {{{these}}}, but the first one has {{{wanted}}}: every record must have the same fields",
                            "`csv_format` : l'enregistrement {number} a les champs {{{these}}}, mais le premier a {{{wanted}}} : tous les enregistrements doivent avoir les mêmes champs"
                        ),
                        span,
                    ));
                }
                let mut fields = Vec::with_capacity(first.len());
                for (i, k) in first.keys().iter().enumerate() {
                    fields.push(field_text(r.get(k).unwrap_or(&Value::Unit), number, i + 1, span)?);
                }
                fields
            }
            (v, _) => {
                let got = v.describe();
                let wanted = if header.is_some() {
                    t!("a record, like the first row", "un enregistrement, comme la première ligne")
                } else {
                    t!("a list of fields", "une liste de champs")
                };
                return Err(Diagnostic::error(
                    "E005",
                    t!(
                        "`csv_format`: row {number} must be {wanted}, but it is {got}",
                        "`csv_format` : la ligne {number} doit être {wanted}, mais c'est {got}"
                    ),
                )
                .at(span)
                .help(t!(
                    "`csv_format` takes a list of rows: lists of fields ([[\"name\", \"age\"], [\"Ada\", 36]]) or records ([{{name: \"Ada\", age: 36}}])",
                    "`csv_format` prend une liste de lignes : des listes de champs ([[\"nom\", \"age\"], [\"Ada\", 36]]) ou des enregistrements ([{{nom: \"Ada\", age: 36}}])"
                ))
                .boxed());
            }
        };
        size += fields.iter().map(|f| f.len() * 2 + 3).sum::<usize>() + 1;
        if size > MAX_TEXT {
            return Err(too_big(false, span));
        }
        table.push(fields);
    }
    Ok(text_value(format(&table, sep)))
}
