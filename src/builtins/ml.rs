//! Numbers for machine learning, for a class: vectors are lists of numbers,
//! matrices are lists of rows (lists of numbers of the same length), and
//! every function gives back new values, always Floats.
//!
//! - math: `tanh`, `sigmoid`, `relu`;
//! - vectors and matrices: `dot`, `norm`, `vector_add`, `vector_sub`,
//!   `vector_mul`, `vector_scale`, `zip_with`, `matmul`, `transpose`,
//!   `zeros`, `ones`, `identity_matrix`, `shape`;
//! - statistics: `stats` (a record: count, mean, median...), `variance`, `std_dev`, `percentile`, `argmax`,
//!   `argmin`, `softmax`, `normalize`;
//! - chance, following `random_seed`: `random_normal`,
//!   `random_choice_weighted`, `train_test_split`.
//!
//! The slopes (`derivative`, `gradient`) are in `grad.rs`, the charts as
//! text (`plot`, `histogram`) in `plot.rs`.
//!
//! The work is done on `f64`s taken out of the lists, then put back into
//! new lists: the values stay ordinary lists, printed and compared as usual.
//! A mistake of shape names both shapes (`a 2×3 matrix and a vector of 2
//! numbers`); no input makes them panic (`nan`, empty or ragged lists).

use super::*;

/// Lists nested deeper than this are not vectors nor matrices: the
/// functions that walk any nesting (`vector_add`, `shape`) stop there.
const MAX_NESTING: usize = 64;

/// The most multiplications `matmul` does (a 2000×2000 by 2000×2000 product
/// is 8 billion: several seconds). Beyond, E023.
const MAX_WORK: f64 = 1e10;

// ----- reading vectors and matrices -----------------------------------------

/// Argument `i` of `f` as a vector: a list of numbers.
pub(super) fn vector(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Vec<f64>> {
    let l = list(a, i, f, span)?;
    numbers_of(l.as_slice(), f, i, span)
}

/// The numbers of a list (argument `i` of `f`).
fn numbers_of(items: &[Value], f: &str, i: usize, span: Span) -> Fallible<Vec<f64>> {
    items.iter().enumerate().map(|(k, v)| as_number(v).ok_or_else(|| not_a_number(f, i, k, v, span))).collect()
}

pub(super) fn as_number(v: &Value) -> Option<f64> {
    match v {
        Value::Int(n) => Some(*n as f64),
        Value::Big(n) => Some(n.to_f64()),
        Value::Float(x) => Some(*x),
        _ => None,
    }
}

fn not_a_number(f: &str, i: usize, k: usize, v: &Value, span: Span) -> Box<Diagnostic> {
    let pos = i + 1;
    let got = v.describe();
    let mut d = Diagnostic::error(
        "E005",
        t!(
            "`{f}` expects a list of numbers as argument {pos}, but its element {k} is {got}",
            "`{f}` attend une liste de nombres comme argument {pos}, mais son élément {k} est {got}"
        ),
    )
    .at(span)
    .help(usage(f));
    if matches!(v, Value::Text(_)) {
        d = d.help(t!(
            "a text is not a number: `parse_float(text)` reads one",
            "un texte n'est pas un nombre : `parse_float(texte)` en lit un"
        ));
    }
    d.boxed()
}

/// A matrix: its numbers row after row, and its numbers of rows and columns.
struct Matrix {
    cells: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn at(&self, r: usize, c: usize) -> f64 {
        self.cells[r * self.cols + c]
    }

    fn value(&self) -> Value {
        if self.cols == 0 {
            return Value::list(vec![Value::list(Vec::new()); self.rows]);
        }
        Value::list(self.cells.chunks(self.cols).map(floats_value).collect())
    }
}

/// Argument `i` of `f` as a matrix: a list of rows of the same length.
fn matrix(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Matrix> {
    let l = list(a, i, f, span)?;
    let pos = i + 1;
    let mut cells = Vec::new();
    let mut cols = 0;
    for (r, row) in l.iter().enumerate() {
        let Value::List(row) = row else {
            let got = row.describe();
            return Err(Diagnostic::error(
                "E005",
                t!(
                    "`{f}` expects a matrix (a list of rows, each a list of numbers) as argument {pos}, but its row {r} is {got}",
                    "`{f}` attend une matrice (une liste de lignes, chacune une liste de nombres) comme argument {pos}, mais sa ligne {r} est {got}"
                ),
            )
            .at(span)
            .help(t!("a matrix of 2 rows and 3 columns: [[1, 2, 3], [4, 5, 6]]", "une matrice de 2 lignes et 3 colonnes : [[1, 2, 3], [4, 5, 6]]"))
            .boxed());
        };
        if r == 0 {
            cols = row.len();
        } else if row.len() != cols {
            let n = row.len();
            return Err(Diagnostic::error(
                "E023",
                t!(
                    "`{f}`: argument {pos} is not a matrix: its row 0 has {cols} number(s), but its row {r} has {n}",
                    "`{f}` : l'argument {pos} n'est pas une matrice : sa ligne 0 a {cols} nombre(s), mais sa ligne {r} en a {n}"
                ),
            )
            .at(span)
            .help(t!("every row of a matrix has the same length", "toutes les lignes d'une matrice ont la même longueur"))
            .boxed());
        }
        for (k, v) in row.iter().enumerate() {
            let x = as_number(v).ok_or_else(|| {
                let got = v.describe();
                Diagnostic::error(
                    "E005",
                    t!(
                        "`{f}` expects a matrix of numbers as argument {pos}, but its row {r} holds {got} at position {k}",
                        "`{f}` attend une matrice de nombres comme argument {pos}, mais sa ligne {r} contient {got} à la position {k}"
                    ),
                )
                .at(span)
                .boxed()
            })?;
            cells.push(x);
        }
    }
    Ok(Matrix { cells, rows: l.len(), cols })
}

/// The sum of numbers, `0.0` for none (Rust's `sum` gives `-0.0`).
fn total(xs: impl Iterator<Item = f64>) -> f64 {
    xs.fold(0.0, |acc, x| acc + x)
}

fn floats_value(xs: &[f64]) -> Value {
    Value::list(xs.iter().map(|&x| Value::Float(x)).collect())
}

/// The shape of a value, in words: "a number", "a vector of 3 numbers",
/// "a 2×3 matrix", "a list of 2 elements".
fn shape_words(v: &Value) -> String {
    match v {
        Value::Int(_) | Value::Big(_) | Value::Float(_) => t!("a number", "un nombre"),
        Value::List(l) => {
            let n = l.len();
            if l.iter().all(|x| as_number(x).is_some()) {
                return t!("a vector of {n} number(s)", "un vecteur de {n} nombre(s)");
            }
            if let Some(Value::List(first)) = l.get(0) {
                let c = first.len();
                if l.iter().all(|row| matches!(row, Value::List(r) if r.len() == c && r.iter().all(|x| as_number(x).is_some()))) {
                    return t!("a {n}×{c} matrix", "une matrice {n}×{c}");
                }
            }
            t!("a list of {n} element(s)", "une liste de {n} élément(s)")
        }
        v => v.describe(),
    }
}

fn shape_mismatch(f: &str, x: &Value, y: &Value, span: Span) -> Box<Diagnostic> {
    let (a, b) = (shape_words(x), shape_words(y));
    Diagnostic::error(
        "E023",
        t!(
            "`{f}` needs two values of the same shape, but got {a} and {b}",
            "`{f}` a besoin de deux valeurs de même forme, mais a reçu {a} et {b}"
        ),
    )
    .at(span)
    .help(t!(
        "`shape(x)` gives the shape of a vector or a matrix: [3] or [2, 3]",
        "`shape(x)` donne la forme d'un vecteur ou d'une matrice : [3] ou [2, 3]"
    ))
    .boxed()
}

fn too_deep(f: &str, span: Span) -> Box<Diagnostic> {
    invalid(
        t!(
            "`{f}`: these lists are nested more than {MAX_NESTING} levels deep: they are not a vector nor a matrix",
            "`{f}` : ces listes sont imbriquées sur plus de {MAX_NESTING} niveaux : ce n'est ni un vecteur ni une matrice"
        ),
        span,
    )
}

/// Numbers, vectors or matrices (lists nested at any depth) of the same
/// shape, combined number by number with `op`.
fn combine(f: &str, a: &[Value], op: fn(f64, f64) -> f64, span: Span) -> Fallible<Value> {
    combine_in(f, &a[0], &a[1], op, 0, a, span)
}

/// `combine` on the parts `x` and `y` of the arguments `top` (named when the shapes differ).
fn combine_in(f: &str, x: &Value, y: &Value, op: fn(f64, f64) -> f64, depth: usize, top: &[Value], span: Span) -> Fallible<Value> {
    if depth > MAX_NESTING {
        return Err(too_deep(f, span));
    }
    match (x, y) {
        (Value::List(a), Value::List(b)) => {
            if a.len() != b.len() {
                return Err(shape_mismatch(f, &top[0], &top[1], span));
            }
            let mut out = Vec::with_capacity(a.len());
            for (p, q) in a.iter().zip(b.iter()) {
                out.push(combine_in(f, p, q, op, depth + 1, top, span)?);
            }
            Ok(Value::list(out))
        }
        _ => match (as_number(x), as_number(y)) {
            (Some(p), Some(q)) => Ok(Value::Float(op(p, q))),
            _ if !matches!(x, Value::List(_)) && as_number(x).is_none() => Err(not_numeric(f, x, span)),
            _ if !matches!(y, Value::List(_)) && as_number(y).is_none() => Err(not_numeric(f, y, span)),
            _ => Err(shape_mismatch(f, &top[0], &top[1], span)),
        },
    }
}

fn not_numeric(f: &str, v: &Value, span: Span) -> Box<Diagnostic> {
    let got = v.describe();
    Diagnostic::error(
        "E005",
        t!(
            "`{f}` works on vectors and matrices of numbers, but found {got} in them",
            "`{f}` travaille sur des vecteurs et des matrices de nombres, mais y a trouvé {got}"
        ),
    )
    .at(span)
    .help(usage(f))
    .boxed()
}

/// Every number of a vector or a matrix (any nesting), changed by `op`.
fn each_number(f: &str, x: &Value, op: &dyn Fn(f64) -> f64, depth: usize, span: Span) -> Fallible<Value> {
    if depth > MAX_NESTING {
        return Err(too_deep(f, span));
    }
    match x {
        Value::List(l) => {
            let mut out = Vec::with_capacity(l.len());
            for v in l.iter() {
                out.push(each_number(f, v, op, depth + 1, span)?);
            }
            Ok(Value::list(out))
        }
        v => match as_number(v) {
            Some(n) => Ok(Value::Float(op(n))),
            None => Err(not_numeric(f, v, span)),
        },
    }
}

// ----- math ---------------------------------------------------------------------

pub(super) fn tanh(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(number(a, 0, "tanh", span)?.tanh()))
}

pub(super) fn sigmoid(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "sigmoid", span)?;
    // Written so that neither side overflows: exp of a negative number only.
    let y = if x >= 0.0 { 1.0 / (1.0 + (-x).exp()) } else { x.exp() / (1.0 + x.exp()) };
    Ok(Value::Float(y))
}

pub(super) fn relu(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = number(a, 0, "relu", span)?;
    Ok(Value::Float(if x > 0.0 || x.is_nan() { x } else { 0.0 }))
}

// ----- vectors and matrices -----------------------------------------------------

pub(super) fn same_length(f: &str, a: &[f64], b: &[f64], span: Span) -> Fallible<()> {
    if a.len() != b.len() {
        let (n, m) = (a.len(), b.len());
        return Err(Diagnostic::error(
            "E023",
            t!(
                "`{f}` needs two vectors of the same length, but got a vector of {n} number(s) and a vector of {m}",
                "`{f}` a besoin de deux vecteurs de même longueur, mais a reçu un vecteur de {n} nombre(s) et un vecteur de {m}"
            ),
        )
        .at(span)
        .boxed());
    }
    Ok(())
}

pub(super) fn dot(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    list(a, 0, "dot", span)?;
    list(a, 1, "dot", span)?;
    let (x, y) = (vector(a, 0, "dot", span)?, vector(a, 1, "dot", span)?);
    same_length("dot", &x, &y, span)?;
    Ok(Value::Float(total(x.iter().zip(&y).map(|(p, q)| p * q))))
}

pub(super) fn norm(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let x = vector(a, 0, "norm", span)?;
    Ok(Value::Float(total(x.iter().map(|p| p * p)).sqrt()))
}

pub(super) fn vector_add(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    combine("vector_add", a, |p, q| p + q, span)
}

pub(super) fn vector_sub(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    combine("vector_sub", a, |p, q| p - q, span)
}

pub(super) fn vector_mul(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    combine("vector_mul", a, |p, q| p * q, span)
}

pub(super) fn vector_scale(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if !matches!(a[0], Value::List(_)) {
        return Err(wrong("vector_scale", 0, Expected::List, &a[0], span));
    }
    let k = number(a, 1, "vector_scale", span)?;
    each_number("vector_scale", &a[0], &|x| x * k, 0, span)
}

pub(super) fn zip_with(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (xs, ys) = (list(a, 0, "zip_with", span)?, list(a, 1, "zip_with", span)?);
    let f = func(a, 2, "zip_with", span)?;
    if xs.len() != ys.len() {
        let (n, m) = (xs.len(), ys.len());
        return Err(invalid(
            t!(
                "`zip_with` needs two lists of the same length, but got {n} and {m} element(s)",
                "`zip_with` a besoin de deux listes de même longueur, mais a reçu {n} et {m} élément(s)"
            ),
            span,
        ));
    }
    let mut out = Vec::with_capacity(xs.len());
    for (x, y) in xs.iter().zip(ys.iter()) {
        out.push(call_given2(it, "zip_with", &f, x.clone(), y.clone(), span)?);
    }
    Ok(Value::list(out))
}

/// Whether a list is a vector (its elements are numbers) rather than a matrix.
fn is_vector(l: &List) -> bool {
    !l.is_empty() && l.iter().all(|v| as_number(v).is_some())
}

pub(super) fn matmul(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    list(a, 0, "matmul", span)?;
    list(a, 1, "matmul", span)?;
    let m = matrix(a, 0, "matmul", span)?;
    let by_vector = matches!(&a[1], Value::List(l) if is_vector(l));
    let n = if by_vector {
        let v = vector(a, 1, "matmul", span)?;
        let len = v.len();
        Matrix { cells: v, rows: len, cols: 1 }
    } else {
        matrix(a, 1, "matmul", span)?
    };
    if m.cols != n.rows {
        let (r, c, nr, nc) = (m.rows, m.cols, n.rows, n.cols);
        let first = t!("a {r}×{c} matrix", "une matrice {r}×{c}");
        let second = if by_vector {
            t!("a vector of {nr} number(s)", "un vecteur de {nr} nombre(s)")
        } else {
            t!("a {nr}×{nc} matrix", "une matrice {nr}×{nc}")
        };
        return Err(Diagnostic::error(
            "E023",
            t!(
                "`matmul` cannot multiply {first} by {second}: the first one has {c} column(s), so the second one needs {c} row(s), not {nr}",
                "`matmul` ne peut pas multiplier {first} par {second} : la matrice de gauche a {c} colonne(s), il faut donc {c} ligne(s) à droite, pas {nr}"
            ),
        )
        .at(span)
        .help(t!(
            "`shape(x)` gives the shape of each one; `transpose(m)` swaps rows and columns",
            "`shape(x)` donne la forme de chacun ; `transpose(m)` échange les lignes et les colonnes"
        ))
        .boxed());
    }
    if m.rows.saturating_mul(n.cols) > MAX_LIST {
        return Err(too_big(true, span));
    }
    if m.rows as f64 * m.cols as f64 * n.cols as f64 > MAX_WORK {
        return Err(invalid(
            t!(
                "`matmul`: these matrices are too large to multiply here (more than {} multiplications)",
                "`matmul` : ces matrices sont trop grandes pour être multipliées ici (plus de {} multiplications)",
                MAX_WORK as u64
            ),
            span,
        ));
    }
    let mut cells = vec![0.0; m.rows * n.cols];
    for r in 0..m.rows {
        for k in 0..m.cols {
            let x = m.at(r, k);
            let row = &n.cells[k * n.cols..(k + 1) * n.cols];
            for (cell, y) in cells[r * n.cols..(r + 1) * n.cols].iter_mut().zip(row) {
                *cell += x * y;
            }
        }
    }
    if by_vector {
        return Ok(floats_value(&cells));
    }
    Ok(Matrix { cells, rows: m.rows, cols: n.cols }.value())
}

pub(super) fn transpose(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let m = matrix(a, 0, "transpose", span)?;
    let mut cells = Vec::with_capacity(m.cells.len());
    for c in 0..m.cols {
        for r in 0..m.rows {
            cells.push(m.at(r, c));
        }
    }
    Ok(Matrix { cells, rows: m.cols, cols: m.rows }.value())
}

/// A vector of `a[0]` numbers, or a matrix of `a[0]` rows and `a[1]` columns, all `x`.
fn filled(a: &[Value], f: &str, x: f64, span: Span) -> Fallible<Value> {
    let rows = count_arg(a, 0, f, span)?;
    if a.len() == 1 {
        if rows > MAX_LIST {
            return Err(too_big(true, span));
        }
        return Ok(Value::list(vec![Value::Float(x); rows]));
    }
    let cols = count_arg(a, 1, f, span)?;
    if rows.saturating_mul(cols.max(1)) > MAX_LIST {
        return Err(too_big(true, span));
    }
    let row = Value::list(vec![Value::Float(x); cols]);
    Ok(Value::list(vec![row; rows]))
}

pub(super) fn zeros(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    filled(a, "zeros", 0.0, span)
}

pub(super) fn ones(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    filled(a, "ones", 1.0, span)
}

pub(super) fn identity_matrix(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let n = count_arg(a, 0, "identity_matrix", span)?;
    if n.saturating_mul(n) > MAX_LIST {
        return Err(too_big(true, span));
    }
    let rows = (0..n).map(|r| Value::list((0..n).map(|c| Value::Float(if r == c { 1.0 } else { 0.0 })).collect())).collect();
    Ok(Value::list(rows))
}

pub(super) fn shape(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    if !matches!(a[0], Value::List(_)) {
        return Err(wrong("shape", 0, Expected::List, &a[0], span));
    }
    /// The sizes of `v` at each depth, or the first place where its parts differ.
    fn walk(v: &Value, depth: usize, span: Span) -> Fallible<Vec<usize>> {
        if depth > MAX_NESTING {
            return Err(too_deep("shape", span));
        }
        match v {
            Value::List(l) => {
                let mut inner: Option<Vec<usize>> = None;
                for (i, x) in l.iter().enumerate() {
                    let s = walk(x, depth + 1, span)?;
                    match &inner {
                        None => inner = Some(s),
                        Some(first) if *first != s => {
                            let (a, b) = (shape_words(&l.as_slice()[0]), shape_words(x));
                            return Err(Diagnostic::error(
                                "E023",
                                t!(
                                    "`shape`: the elements of this list do not all have the same shape: element 0 is {a}, element {i} is {b}",
                                    "`shape` : les éléments de cette liste n'ont pas tous la même forme : l'élément 0 est {a}, l'élément {i} est {b}"
                                ),
                            )
                            .at(span)
                            .boxed());
                        }
                        Some(_) => {}
                    }
                }
                let mut out = vec![l.len()];
                out.extend(inner.unwrap_or_default());
                Ok(out)
            }
            v if as_number(v).is_some() => Ok(Vec::new()),
            v => Err(not_numeric("shape", v, span)),
        }
    }
    let sizes = walk(&a[0], 0, span)?;
    Ok(Value::list(sizes.into_iter().map(|n| Value::Int(n as i64)).collect()))
}

// ----- statistics -----------------------------------------------------------------

/// The numbers of argument 0, at least one.
fn sample(a: &[Value], f: &str, span: Span) -> Fallible<Vec<f64>> {
    let xs = vector(a, 0, f, span)?;
    if xs.is_empty() {
        return Err(invalid(t!("`{f}` of an empty list", "`{f}` d'une liste vide"), span));
    }
    Ok(xs)
}

fn mean_of(xs: &[f64]) -> f64 {
    total(xs.iter().copied()) / xs.len() as f64
}

fn variance_of(xs: &[f64]) -> f64 {
    let m = mean_of(xs);
    total(xs.iter().map(|x| (x - m) * (x - m))) / xs.len() as f64
}

pub(super) fn stats(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut xs = sample(a, "stats", span)?;
    let (mean, sd) = (mean_of(&xs), variance_of(&xs).sqrt());
    xs.sort_by(f64::total_cmp);
    let mut r = Record::new();
    r.set(Rc::from("count"), Value::Int(xs.len() as i64));
    r.set(Rc::from("mean"), Value::Float(mean));
    r.set(Rc::from("median"), Value::Float(quantile(&xs, 0.5)));
    r.set(Rc::from("std_dev"), Value::Float(sd));
    r.set(Rc::from("min"), Value::Float(xs[0]));
    r.set(Rc::from("max"), Value::Float(xs[xs.len() - 1]));
    Ok(Value::Record(Rc::new(r)))
}

/// The number at the share `q` (0 to 1) of sorted numbers, between the two
/// closest ones in proportion (as spreadsheets do).
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let rank = q * (sorted.len() - 1) as f64;
    let (lo, hi) = (rank.floor() as usize, rank.ceil() as usize);
    if lo == hi {
        sorted[lo]
    } else {
        sorted[lo] + (sorted[hi] - sorted[lo]) * (rank - lo as f64)
    }
}

pub(super) fn variance(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(variance_of(&sample(a, "variance", span)?)))
}

pub(super) fn std_dev(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Float(variance_of(&sample(a, "std_dev", span)?).sqrt()))
}

pub(super) fn percentile(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    list(a, 0, "percentile", span)?;
    let p = number(a, 1, "percentile", span)?;
    let mut xs = sample(a, "percentile", span)?;
    if !(0.0..=100.0).contains(&p) {
        return Err(invalid(
            t!("`percentile` needs a percentage from 0 to 100, not {p}", "`percentile` a besoin d'un pourcentage de 0 à 100, pas {p}"),
            span,
        ));
    }
    xs.sort_by(f64::total_cmp);
    Ok(Value::Float(quantile(&xs, p / 100.0)))
}

/// The position of the first number that `better` prefers to all the others.
fn arg_best(a: &[Value], f: &str, better: fn(f64, f64) -> bool, span: Span) -> Fallible<Value> {
    let xs = sample(a, f, span)?;
    let mut best = 0;
    for (i, &x) in xs.iter().enumerate() {
        // `nan` is never preferred, but a list of `nan` has a first one.
        if better(x, xs[best]) || xs[best].is_nan() && !x.is_nan() {
            best = i;
        }
    }
    Ok(Value::Int(best as i64))
}

pub(super) fn argmax(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    arg_best(a, "argmax", |x, best| x > best, span)
}

pub(super) fn argmin(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    arg_best(a, "argmin", |x, best| x < best, span)
}

pub(super) fn softmax(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let xs = vector(a, 0, "softmax", span)?;
    // Subtracting the largest number changes nothing but keeps `exp` finite.
    let top = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if top == f64::INFINITY {
        // Infinite scores share everything.
        let k = xs.iter().filter(|x| **x == f64::INFINITY).count() as f64;
        return Ok(floats_value(&xs.iter().map(|x| if *x == f64::INFINITY { 1.0 / k } else { 0.0 }).collect::<Vec<_>>()));
    }
    let top = if top.is_finite() { top } else { 0.0 };
    let es: Vec<f64> = xs.iter().map(|x| (x - top).exp()).collect();
    let sum = total(es.iter().copied());
    Ok(floats_value(&es.iter().map(|e| e / sum).collect::<Vec<_>>()))
}

pub(super) fn normalize(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let xs = vector(a, 0, "normalize", span)?;
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let width = hi - lo;
    let ys: Vec<f64> = xs.iter().map(|x| if width > 0.0 && width.is_finite() { (x - lo) / width } else { 0.0 }).collect();
    Ok(floats_value(&ys))
}

// ----- chance -----------------------------------------------------------------------

/// A Float in [0, 1), from the generator of `random`.
fn uniform(it: &mut Interp) -> f64 {
    (it.random_u64() >> 11) as f64 / (1u64 << 53) as f64
}

pub(super) fn random_normal(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mean = number(a, 0, "random_normal", span)?;
    let sd = number(a, 1, "random_normal", span)?;
    if sd.is_nan() || sd < 0.0 {
        return Err(invalid(
            t!(
                "`random_normal` needs a standard deviation of 0 or more, not {sd}",
                "`random_normal` a besoin d'un écart type positif ou nul, pas {sd}"
            ),
            span,
        ));
    }
    // Box-Muller: two uniform draws give one normal draw.
    let u1 = 1.0 - uniform(it);
    let u2 = uniform(it);
    let z = (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos();
    Ok(Value::Float(mean + sd * z))
}

pub(super) fn random_choice_weighted(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let xs = list(a, 0, "random_choice_weighted", span)?;
    list(a, 1, "random_choice_weighted", span)?;
    let ws = vector(a, 1, "random_choice_weighted", span)?;
    if xs.len() != ws.len() {
        let (n, m) = (xs.len(), ws.len());
        return Err(invalid(
            t!(
                "`random_choice_weighted` needs one weight per element: {n} element(s), but {m} weight(s)",
                "`random_choice_weighted` a besoin d'un poids par élément : {n} élément(s), mais {m} poids"
            ),
            span,
        ));
    }
    if let Some((i, w)) = ws.iter().enumerate().find(|(_, w)| !w.is_finite() || **w < 0.0) {
        return Err(invalid(
            t!(
                "`random_choice_weighted`: the weight {i} is {w}; a weight is a number of 0 or more",
                "`random_choice_weighted` : le poids {i} vaut {w} ; un poids est un nombre positif ou nul"
            ),
            span,
        ));
    }
    let total = total(ws.iter().copied());
    if !(total > 0.0 && total.is_finite()) {
        return Err(invalid(
            t!(
                "`random_choice_weighted` needs at least one weight above 0",
                "`random_choice_weighted` a besoin d'au moins un poids supérieur à 0"
            ),
            span,
        ));
    }
    let mut r = uniform(it) * total;
    for (x, w) in xs.iter().zip(&ws) {
        if r < *w {
            return Ok(x.clone());
        }
        r -= w;
    }
    // Rounding may leave a tiny rest: the last element of weight above 0.
    let last = ws.iter().rposition(|w| *w > 0.0).unwrap_or(0);
    Ok(xs.as_slice()[last].clone())
}

pub(super) fn train_test_split(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let mut v = list(a, 0, "train_test_split", span)?.as_slice().to_vec();
    let ratio = number(a, 1, "train_test_split", span)?;
    if !(0.0..=1.0).contains(&ratio) {
        return Err(invalid(
            t!(
                "`train_test_split` needs a share from 0 to 1 for the training part (like 0.8), not {ratio}",
                "`train_test_split` a besoin d'une part de 0 à 1 pour l'entraînement (comme 0.8), pas {ratio}"
            ),
            span,
        ));
    }
    // The same Fisher-Yates as `shuffle`, then a cut.
    for i in (1..v.len()).rev() {
        let j = (it.random_u64() % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
    let cut = ((v.len() as f64 * ratio).round() as usize).min(v.len());
    let test = v.split_off(cut);
    let mut r = Record::new();
    r.set(Rc::from("train"), Value::list(v));
    r.set(Rc::from("test"), Value::list(test));
    Ok(Value::Record(Rc::new(r)))
}
