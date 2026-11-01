//! What the checker knows of the built-in functions added to the library
//! after 1.0 (text cases, `format`, `tally`, combinations, `dig`, `update`,
//! `all_ok`...). Like the others: an argument is reported only when it can
//! never be accepted, checked in the order the function checks it when it
//! runs, so that the message is the one it would stop with.

use super::*;

/// What a call to `name` gives (see [`super::builtin_call`]); `Err(Gives)`
/// for a function described elsewhere, or not at all.
pub(super) fn call(name: &str, a: &[Ty]) -> Result<Ty, Outcome> {
    let arg = |i: usize| a.get(i).cloned().unwrap_or(Ty::Any);
    let n = a.len();
    Ok(match name {
        "inspect" => Ty::Text,
        "print_error" => Ty::Unit,
        // Any values that can be compared; the result is one of the three.
        "constrain" => arg(0).join(&arg(1)).join(&arg(2)),
        "divmod" => {
            need(a, 0, Expected::Number)?;
            need(a, 1, Expected::Number)?;
            Ty::list(match (arg(0), arg(1)) {
                (Ty::Int, Ty::Int) => Ty::Int,
                (Ty::Float, _) | (_, Ty::Float) => Ty::Float,
                _ => Ty::Num,
            })
        }
        "binomial" => {
            need(a, 0, Expected::Int)?;
            need(a, 1, Expected::Int)?;
            Ty::Int
        }
        "digits" => {
            need(a, 0, Expected::Int)?;
            need(a, 1, Expected::Int)?;
            Ty::list(Ty::Int)
        }
        "to_base" => {
            need(a, 0, Expected::Int)?;
            need(a, 1, Expected::Int)?;
            Ty::Text
        }
        "from_base" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Int)?;
            Ty::result(Ty::Int, Ty::Text)
        }
        "is_positive" | "is_negative" | "is_zero" | "is_nan" | "is_finite" => {
            need(a, 0, Expected::Number)?;
            Ty::Bool
        }
        // Any value is centered as `print` shows it.
        "center" => {
            need(a, 1, Expected::Int)?;
            need(a, 2, Expected::Text)?;
            Ty::Text
        }
        "format" => {
            need(a, 1, Expected::Text)?;
            Ty::Text
        }
        "replace_first" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Text)?;
            need(a, 2, Expected::Text)?;
            Ty::Text
        }
        "occurrences" | "last_index_of" => {
            need(a, 0, Expected::ListOrText)?;
            // In a text, what is looked for is a text.
            if arg(0).only(TEXT) {
                need(a, 1, Expected::Text)?;
            }
            if name == "occurrences" {
                Ty::Int
            } else {
                Ty::option(Ty::Int)
            }
        }
        "remove_prefix" | "remove_suffix" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Text)?;
            Ty::Text
        }
        "capitalize_words" | "snake_case" | "camel_case" | "kebab_case" | "swap_case" => {
            need(a, 0, Expected::Text)?;
            Ty::Text
        }
        "is_upper" | "is_lower" => {
            need(a, 0, Expected::Text)?;
            Ty::Bool
        }
        "bytes" => {
            need(a, 0, Expected::Text)?;
            Ty::list(Ty::Int)
        }
        "split_once" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Text)?;
            Ty::option(Ty::list(Ty::Text))
        }
        "word_wrap" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Int)?;
            Ty::Text
        }
        "edit_distance" => {
            need(a, 0, Expected::Text)?;
            need(a, 1, Expected::Text)?;
            Ty::Int
        }
        "find_index" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Function)?;
            Ty::option(Ty::Int)
        }
        "update_at" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Int)?;
            need(a, 2, Expected::Function)?;
            Ty::list(arg(0).elem().join(&arg(2).ret()))
        }
        "rotate_by" => {
            need(a, 0, Expected::ListOrText)?;
            need(a, 1, Expected::Int)?;
            same_list_or_text(&arg(0))
        }
        "intersperse" => {
            need(a, 0, Expected::List)?;
            Ty::list(arg(0).elem().join(&arg(1)))
        }
        "interleave" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::List)?;
            Ty::list(arg(0).elem().join(&arg(1).elem()))
        }
        "tally" => {
            need(a, 0, Expected::List)?;
            Ty::dict(arg(0).elem(), Ty::Int)
        }
        "count_by" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Function)?;
            Ty::dict(arg(1).ret(), Ty::Int)
        }
        "most_common" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Int)?;
            Ty::list(Ty::list(arg(0).elem().join(&Ty::Int)))
        }
        "sort_desc" => {
            need(a, 0, Expected::List)?;
            Ty::list(arg(0).elem())
        }
        "unique_by" | "sort_by_desc" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Function)?;
            Ty::list(arg(0).elem())
        }
        "chunk_by" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Function)?;
            Ty::list(Ty::list(arg(0).elem()))
        }
        "split_at" => {
            need(a, 0, Expected::ListOrText)?;
            need(a, 1, Expected::Int)?;
            Ty::list(same_list_or_text(&arg(0)))
        }
        "combinations" | "permutations" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Int)?;
            Ty::list(Ty::list(arg(0).elem()))
        }
        "cartesian_product" => {
            for i in 0..n {
                need(a, i, Expected::List)?;
            }
            Ty::list(Ty::list(a.iter().fold(Ty::Never, |acc, t| acc.join(&t.elem()))))
        }
        "unzip" => {
            need(a, 0, Expected::List)?;
            Ty::list(Ty::list(arg(0).elem().elem()))
        }
        "iterate" => {
            need(a, 1, Expected::Int)?;
            need(a, 2, Expected::Function)?;
            Ty::list(arg(0).join(&arg(2).ret()))
        }
        "unfold" => {
            need(a, 1, Expected::Function)?;
            Ty::list(Ty::Any)
        }
        "filter_map" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::Function)?;
            Ty::list(match arg(1).ret() {
                Ty::Data(d) if d.kind() == OPTION => d.payload(),
                _ => Ty::Any,
            })
        }
        "dig" => {
            need(a, 1, Expected::List)?;
            Ty::option(Ty::Any)
        }
        "merge_with" => {
            need(a, 0, Expected::RecordOrDict)?;
            if arg(0).only(RECORD) {
                need(a, 1, Expected::Record)?;
            }
            if arg(0).only(DICT) {
                need(a, 1, Expected::Dict)?;
            }
            need(a, 2, Expected::Function)?;
            match (arg(0), arg(1)) {
                (Ty::Dict(k, v), Ty::Dict(l, w)) => Ty::dict(k.join(&l), v.join(&w).join(&arg(2).ret())),
                (t, _) if t.only(RECORD) => Ty::record(Vec::new(), false),
                _ => Ty::Any,
            }
        }
        "invert" => {
            need(a, 0, Expected::RecordOrDict)?;
            match arg(0) {
                Ty::Dict(k, v) => Ty::dict((*v).clone(), (*k).clone()),
                Ty::Record(r) if r.exact => Ty::dict(r.fields.iter().fold(Ty::Never, |acc, (_, t)| acc.join(t)), Ty::Text),
                t if t.only(RECORD) => Ty::dict(Ty::Any, Ty::Text),
                _ => Ty::dict(Ty::Any, Ty::Any),
            }
        }
        "select_keys" | "reject_keys" => {
            need(a, 0, Expected::RecordOrDict)?;
            need(a, 1, Expected::ListOrSet)?;
            match arg(0) {
                t @ Ty::Dict(..) => t,
                t if t.only(RECORD) => Ty::record(Vec::new(), false),
                _ => Ty::Any,
            }
        }
        "map_keys" => {
            need(a, 0, Expected::RecordOrDict)?;
            need(a, 1, Expected::Function)?;
            match arg(0) {
                Ty::Dict(_, v) => Ty::dict(arg(1).ret(), (*v).clone()),
                t if t.only(RECORD) => Ty::record(Vec::new(), false),
                _ => Ty::Any,
            }
        }
        "update" => {
            need(a, 0, Expected::RecordOrDict)?;
            if arg(0).only(RECORD) {
                need(a, 1, Expected::Text)?;
            }
            need(a, 3, Expected::Function)?;
            match arg(0) {
                Ty::Dict(k, v) => Ty::dict(k.join(&arg(1)), v.join(&arg(2)).join(&arg(3).ret())),
                t if t.only(RECORD) => Ty::record(Vec::new(), false),
                _ => Ty::Any,
            }
        }
        "symmetric_difference" => {
            need(a, 0, Expected::Set)?;
            need(a, 1, Expected::Set)?;
            match (arg(0), arg(1)) {
                (Ty::Set(x), Ty::Set(y)) => Ty::set(x.join(&y)),
                _ => Ty::Any,
            }
        }
        "is_disjoint" => {
            need(a, 0, Expected::Set)?;
            need(a, 1, Expected::Set)?;
            Ty::Bool
        }
        "unwrap_or_else" => {
            need(a, 0, Expected::OptionOrResult)?;
            need(a, 1, Expected::Function)?;
            payload(&arg(0)).join(&arg(1).ret())
        }
        "expect" => {
            need(a, 0, Expected::OptionOrResult)?;
            need(a, 1, Expected::Text)?;
            payload(&arg(0))
        }
        "all_ok" => {
            need(a, 0, Expected::List)?;
            match arg(0).elem() {
                Ty::Data(d) if d.kind() == RESULT => Ty::result(Ty::list(d.payload()), error_of(&d)),
                _ => Ty::result(Ty::list(Ty::Any), Ty::Any),
            }
        }
        "add_months" => {
            need(a, 0, Expected::Date)?;
            need(a, 1, Expected::Int)?;
            date_ty(false, false)
        }
        "day_of_year" => {
            need(a, 0, Expected::Date)?;
            Ty::Int
        }
        "linspace" => {
            need(a, 0, Expected::Number)?;
            need(a, 1, Expected::Number)?;
            need(a, 2, Expected::Int)?;
            Ty::list(Ty::Float)
        }
        "correlation" | "covariance" => {
            need(a, 0, Expected::List)?;
            need(a, 1, Expected::List)?;
            Ty::Float
        }
        "base64_encode" | "url_encode" => {
            need(a, 0, Expected::Text)?;
            Ty::Text
        }
        "base64_decode" | "url_decode" => {
            need(a, 0, Expected::Text)?;
            Ty::result(Ty::Text, Ty::Text)
        }
        _ => return Err(Outcome::Gives(Ty::Any)),
    })
}

/// How these functions call the function they are given (see [`super::callback`]).
pub(super) fn callback(name: &str, index: usize, a: &[Ty]) -> Option<Callback> {
    let first = a.first().cloned().unwrap_or(Ty::Any);
    let elem = first.elem();
    let (params, returns) = match (name, index) {
        ("find_index", 1) => (vec![elem], Some(BOOL)),
        ("update_at", 2) => (vec![elem], None),
        ("count_by" | "unique_by" | "sort_by_desc" | "chunk_by", 1) => (vec![elem], None),
        ("filter_map", 1) => (vec![elem], Some(OPTION)),
        ("iterate", 2) => (vec![Ty::Any], None),
        ("unfold", 1) => (vec![Ty::Any], Some(OPTION)),
        ("merge_with", 2) => match (&first, a.get(1)) {
            (Ty::Dict(_, v), Some(Ty::Dict(_, w))) => (vec![(**v).clone(), (**w).clone()], None),
            // Only the names both records have are merged: with none, the function is never called.
            (Ty::Record(x), Some(Ty::Record(y))) if x.exact && y.exact && !x.fields.iter().any(|(k, _)| y.get(k).is_some()) => {
                (vec![Ty::Never, Ty::Never], None)
            }
            _ => (vec![Ty::Any, Ty::Any], None),
        },
        // A record without any field: the function is never called.
        ("map_keys", 1) if matches!(&first, Ty::Record(r) if r.exact && r.fields.is_empty()) => (vec![Ty::Never], None),
        // A record's names are texts, and must stay texts.
        ("map_keys", 1) => match &first {
            Ty::Dict(k, _) => (vec![(**k).clone()], None),
            t if t.only(RECORD) => (vec![Ty::Text], Some(TEXT)),
            _ => (vec![Ty::Any], None),
        },
        ("update", 3) => match &first {
            Ty::Dict(_, v) => (vec![(**v).clone()], None),
            _ => (vec![Ty::Any], None),
        },
        // `None` gives the function nothing, `Err(e)` gives it `e`.
        ("unwrap_or_else", 1) => match &first {
            // Never `None`: the function is never called.
            Ty::Data(d) if d.kind() == OPTION && d.fields_of("None", 0).is_none() => (vec![Ty::Never], None),
            Ty::Data(d) if d.kind() == OPTION => (Vec::new(), None),
            Ty::Data(d) if d.kind() == RESULT => (vec![error_of(d)], None),
            _ if a.is_empty() => (vec![Ty::Any], None),
            _ => return None,
        },
        _ => return None,
    };
    Some(Callback { params, returns })
}
