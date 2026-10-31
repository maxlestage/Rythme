//! Option and Result: a default computed only when needed
//! (`unwrap_or_else`), unwrapping with your own message (`expect`), and a
//! list of Results made into one Result (`all_ok`).

use super::*;

pub(super) fn unwrap_or_else(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let kind = option_or_result(it, &a[0], "unwrap_or_else", span)?;
    let f = func(a, 1, "unwrap_or_else", span)?;
    match kind {
        Wrap::Some | Wrap::Ok => Ok(Interp::payload(&a[0])),
        // `None` gives nothing to the function; `Err(e)` gives it `e`.
        Wrap::None => {
            given_arity("unwrap_or_else", &f, 0, span)?;
            it.call(f, Vec::new(), span)
        }
        _ => call_given1(it, "unwrap_or_else", &f, Interp::payload(&a[0]), span),
    }
}

pub(super) fn expect(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let kind = option_or_result(it, &a[0], "expect", span)?;
    let message = text(a, 1, "expect", span)?;
    match kind {
        Wrap::Some | Wrap::Ok => Ok(Interp::payload(&a[0])),
        Wrap::None => Err(Diagnostic::error("E013", message.to_string())
            .at(span)
            .note(t!("`expect` found `None`: there is no value", "`expect` a trouvé `None` : il n'y a pas de valeur"))
            .boxed()),
        _ => Err(Diagnostic::error("E013", message.to_string())
            .at(span)
            .note(t!("`expect` found an error: {}", "`expect` a trouvé une erreur : {}", Interp::payload(&a[0]).display()))
            .boxed()),
    }
}

pub(super) fn all_ok(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let l = list(a, 0, "all_ok", span)?;
    let mut out = Vec::with_capacity(l.len());
    for (k, r) in l.iter().enumerate() {
        match it.wrap(r) {
            Wrap::Ok => out.push(Interp::payload(r)),
            Wrap::Err => return Ok(r.clone()),
            _ => {
                return Err(Diagnostic::error(
                    "E005",
                    t!(
                        "`all_ok` needs a list of Results (Ok or Err), but its element {k} is {}",
                        "`all_ok` a besoin d'une liste de Results (Ok ou Err), mais son élément {k} est {}",
                        r.describe()
                    ),
                )
                .at(span)
                .help(t!(
                    "to keep the values of a list of Options, use `filter_map(list, fn(x) = x)`",
                    "pour garder les valeurs d'une liste d'Options, utilisez `filter_map(liste, fn(x) = x)`"
                ))
                .boxed())
            }
        }
    }
    Ok(it.ok(Value::list(out)))
}
