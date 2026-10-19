//! Charts drawn as text: `plot(ys)`, `plot(xs, ys)` and `histogram(xs)`
//! give a text to `print`, the same in a terminal, in the playground and in
//! a test. Points are placed on a grid of 50 columns and 11 lines; the
//! labels of the axes are lined up by their width on the screen (Chinese
//! digits too), and numbers that are not finite (`nan`, `inf`) are left out.
//! The same numbers always give the same drawing.

use super::ml::{same_length, vector};
use super::*;
use crate::diagnostic::display_width;

/// Columns and lines of the drawing area of `plot`.
const PLOT_WIDTH: usize = 50;
const PLOT_HEIGHT: usize = 11;
/// The longest bar of `histogram`, in columns.
const BAR_WIDTH: usize = 40;

/// A number on an axis, written with the decimals that a difference of
/// `step` between two marks needs: `0.25`, `170.5`, `1`, or `2.00e9` for
/// very large or very small numbers.
fn tick(x: f64, step: f64) -> String {
    if x == 0.0 {
        return "0".into();
    }
    let a = x.abs();
    if !a.is_finite() || a >= 1e6 || (a < 1e-4 && step < 1e-4) {
        return format!("{x:.2e}");
    }
    let decimals = if step.is_normal() { (1 - step.abs().log10().floor() as i32).clamp(0, 6) as usize } else { 2 };
    let s = format!("{x:.decimals$}");
    let s = if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s };
    if s == "-0" {
        "0".into()
    } else {
        s
    }
}

/// `s` padded on the left to `width` columns of a terminal.
fn right_aligned(s: &str, width: usize) -> String {
    format!("{}{s}", " ".repeat(width.saturating_sub(display_width(s))))
}

fn nothing_to_draw(f: &str, span: Span) -> Box<Diagnostic> {
    invalid(
        t!(
            "`{f}` has nothing to draw: the list is empty, or holds no finite number",
            "`{f}` n'a rien à dessiner : la liste est vide, ou ne contient aucun nombre fini"
        ),
        span,
    )
}

/// The smallest and largest of finite numbers, a little apart when equal.
fn range_of(xs: impl Iterator<Item = f64>) -> (f64, f64) {
    let (lo, hi) = xs.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| (lo.min(x), hi.max(x)));
    if lo == hi {
        let pad = if lo == 0.0 { 1.0 } else { lo.abs() / 2.0 };
        (lo - pad, hi + pad)
    } else {
        (lo, hi)
    }
}

/// The place (0 to `n - 1`) of `x` between `lo` and `hi`.
fn cell(x: f64, lo: f64, hi: f64, n: usize) -> usize {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    ((t * (n - 1) as f64).round() as usize).min(n - 1)
}

pub(super) fn plot(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (xs, ys) = if a.len() == 2 {
        list(a, 0, "plot", span)?;
        list(a, 1, "plot", span)?;
        let (xs, ys) = (vector(a, 0, "plot", span)?, vector(a, 1, "plot", span)?);
        same_length("plot", &xs, &ys, span)?;
        (xs, ys)
    } else {
        let ys = vector(a, 0, "plot", span)?;
        ((0..ys.len()).map(|i| i as f64).collect(), ys)
    };
    let points: Vec<(f64, f64)> = xs.into_iter().zip(ys).filter(|(x, y)| x.is_finite() && y.is_finite()).collect();
    if points.is_empty() {
        return Err(nothing_to_draw("plot", span));
    }
    let (x_lo, x_hi) = range_of(points.iter().map(|p| p.0));
    let (y_lo, y_hi) = range_of(points.iter().map(|p| p.1));
    let mut grid = vec![vec![' '; PLOT_WIDTH]; PLOT_HEIGHT];
    for &(x, y) in &points {
        let (c, r) = (cell(x, x_lo, x_hi, PLOT_WIDTH), cell(y, y_lo, y_hi, PLOT_HEIGHT));
        grid[PLOT_HEIGHT - 1 - r][c] = '•';
    }
    let y_step = (y_hi - y_lo) / (PLOT_HEIGHT - 1) as f64;
    let (top, middle, bottom) = (tick(y_hi, y_step), tick((y_lo + y_hi) / 2.0, y_step), tick(y_lo, y_step));
    let width = [&top, &middle, &bottom].iter().map(|s| display_width(s)).max().unwrap_or(0);
    let mut out = String::new();
    for (i, row) in grid.iter().enumerate() {
        let label = match i {
            0 => &top,
            _ if i == PLOT_HEIGHT / 2 => &middle,
            _ if i == PLOT_HEIGHT - 1 => &bottom,
            _ => "",
        };
        let axis = if label.is_empty() { '│' } else { '┤' };
        let line: String = row.iter().collect();
        out.push_str(&format!("{} {axis}{}\n", right_aligned(label, width), line.trim_end()));
    }
    out.push_str(&format!("{} └{}\n", " ".repeat(width), "─".repeat(PLOT_WIDTH)));
    let x_step = (x_hi - x_lo) / (PLOT_WIDTH - 1) as f64;
    let (left, right) = (tick(x_lo, x_step), tick(x_hi, x_step));
    let gap = PLOT_WIDTH.saturating_sub(display_width(&left) + display_width(&right)).max(1);
    out.push_str(&format!("{}  {left}{}{right}", " ".repeat(width), " ".repeat(gap)));
    Ok(text_value(out))
}

pub(super) fn histogram(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    list(a, 0, "histogram", span)?;
    let bins = if a.len() == 2 { size_arg(a, 1, "histogram", span)? } else { 10 };
    let xs: Vec<f64> = vector(a, 0, "histogram", span)?.into_iter().filter(|x| x.is_finite()).collect();
    if bins > 100 {
        return Err(invalid(
            t!("`histogram` draws at most 100 bars, not {bins}", "`histogram` dessine au plus 100 barres, pas {bins}"),
            span,
        ));
    }
    if xs.is_empty() {
        return Err(nothing_to_draw("histogram", span));
    }
    let (lo, hi) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| (lo.min(x), hi.max(x)));
    // All the same number: one bar.
    let bins = if lo == hi { 1 } else { bins };
    let width = (hi - lo) / bins as f64;
    let mut counts = vec![0usize; bins];
    for &x in &xs {
        let i = if width > 0.0 { ((x - lo) / width).floor() as usize } else { 0 };
        counts[i.min(bins - 1)] += 1;
    }
    let edges: Vec<String> = (0..=bins).map(|i| tick(if i == bins { hi } else { lo + width * i as f64 }, width)).collect();
    let left = edges[..bins].iter().map(|s| display_width(s)).max().unwrap_or(0);
    let right = edges[1..].iter().map(|s| display_width(s)).max().unwrap_or(0);
    let most = counts.iter().copied().max().unwrap_or(0).max(1);
    // A bar is drawn in eighths of a column: full blocks, then one of these.
    let eighths = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
    let mut lines = Vec::new();
    for (i, &n) in counts.iter().enumerate() {
        let length = n * BAR_WIDTH * 8 / most;
        let bar = format!("{}{}", "█".repeat(length / 8), eighths[length % 8]);
        let range = format!("{} – {}", right_aligned(&edges[i], left), right_aligned(&edges[i + 1], right));
        lines.push(format!("{range} │{bar} {n}"));
    }
    Ok(text_value(lines.join("\n")))
}
