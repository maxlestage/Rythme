//! Types written in the code: `fn area(w: Int, h: Int) -> Int`,
//! `total: Float = 0.0`, `Circle(radius: Float)`.
//!
//! They are optional and never needed: running a program never looks at
//! them. The checker reads them as promises. It reports a value that
//! surely breaks one (E027): an argument, the value given to a name, the
//! result of a function. Inside a function, a parameter has the type written
//! for it, and a call gives the type written for the result, so that more
//! mistakes are found, and found where they are made. Like the rest of the
//! checker, it only speaks when it is sure: a value whose type is not known
//! is never reported.

use std::collections::HashMap;
use std::rc::Rc;

use super::{literal, needs, Checker, GKind, Res};
use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::source::Span;
use crate::symbol::Sym;
use crate::types::{DataTy, Ty, DATA, FLOAT, INT};

/// What the code writes of the types of a function or a constructor.
#[derive(Clone, Debug)]
pub struct Written {
    /// Its name, as in messages: `area`, `geo.area`, `Circle`.
    pub name: String,
    /// The name of each parameter (or field), with its written type.
    pub params: Vec<(String, Option<Rc<TypeExpr>>)>,
    pub ret: Option<Rc<TypeExpr>>,
    /// The name of the file it comes from, when that file is used with a
    /// name (`use geometry`): its types are known as `geometry.Shape`.
    pub module: Option<Sym>,
}

impl Written {
    /// What a function writes of its types, if it writes any.
    pub fn of_fn(def: &FnDef) -> Option<Written> {
        let params: Vec<(String, Option<Rc<TypeExpr>>)> =
            needs::param_names(def).into_iter().enumerate().map(|(i, n)| (n, def.param_type(i).cloned())).collect();
        let ret = def.ret_type().cloned();
        (ret.is_some() || params.iter().any(|(_, t)| t.is_some())).then(|| Written {
            name: def.name.map_or_else(|| "fn".to_string(), |n| n.to_string()),
            params,
            ret,
            module: None,
        })
    }

    /// What a constructor writes of the types of its fields, if it writes any.
    pub fn of_variant(v: &VariantDef) -> Option<Written> {
        v.types.iter().any(Option::is_some).then(|| Written {
            name: v.name.to_string(),
            params: v.fields.iter().zip(&v.types).map(|(f, t)| (f.to_string(), t.clone())).collect(),
            ret: None,
            module: None,
        })
    }
}

/// The types of the program and of Rythme that a written type may name,
/// with how many types each one takes between parentheses.
fn builtin_type(name: &str) -> Option<&'static [usize]> {
    Some(match name {
        "Int" | "Float" | "Text" | "Bool" | "Unit" | "Symbol" | "Record" | "Function" => &[0],
        "List" | "Set" | "Option" => &[0, 1],
        "Dict" | "Result" => &[0, 2],
        _ => return None,
    })
}

/// The name of a type in another language, and its name in Rythme.
fn other_name(name: &str) -> Option<&'static str> {
    Some(match name {
        "int" | "integer" | "Integer" | "i32" | "i64" | "u32" | "u64" | "usize" | "long" | "Long" | "BigInt" => "Int",
        "float" | "double" | "Double" | "f32" | "f64" | "real" | "decimal" | "Decimal" | "number" | "Number" => "Float",
        "string" | "str" | "String" | "char" | "Char" | "Str" => "Text",
        "bool" | "boolean" | "Boolean" => "Bool",
        "list" | "array" | "Array" | "Vec" | "vec" | "vector" | "Vector" | "ArrayList" | "tuple" | "Tuple" => "List",
        "dict" | "map" | "Map" | "HashMap" | "hash" | "Hash" | "object" | "Object" | "Dictionary" => "Dict",
        "set" | "HashSet" => "Set",
        "void" | "None" | "nil" | "null" | "unit" | "()" => "Unit",
        "Maybe" | "Optional" | "option" => "Option",
        "Either" | "result" => "Result",
        "symbol" => "Symbol",
        "record" => "Record",
        "func" | "function" | "Func" | "Fn" | "Callable" | "lambda" | "Lambda" | "Proc" => "fn(Int) -> Int",
        _ => return None,
    })
}

/// Is a value of type `actual` surely not of the type `written`? Only true
/// when it is certain: a type that is not known never conflicts.
pub(super) fn conflicts(actual: &Ty, written: &Ty) -> bool {
    if !actual.is_known() || !written.is_known() {
        return false;
    }
    if actual.never(written.kinds()) {
        return true;
    }
    match (actual, written) {
        (Ty::List(x), Ty::List(y)) | (Ty::Set(x), Ty::Set(y)) => conflicts(x, y),
        (Ty::Dict(k, v), Ty::Dict(l, u)) => conflicts(k, l) || conflicts(v, u),
        (Ty::Record(r), Ty::Record(s)) => s.fields.iter().any(|(k, t)| match r.get(k) {
            Some(x) => conflicts(x, t),
            None => r.exact,
        }),
        (Ty::Data(x), Ty::Data(y)) if x.name != y.name => {
            x.kind() == DATA && y.kind() == DATA && ![x.name, y.name].contains(&needs::any_data())
        }
        (Ty::Data(x), Ty::Data(y)) => match (&x.ctors, &y.ctors) {
            (Some(xs), Some(ys)) => xs.iter().any(|(c, fs)| {
                ys.iter()
                    .find(|(d, _)| d == c)
                    .is_some_and(|(_, gs)| fs.len() == gs.len() && fs.iter().zip(gs).any(|(f, g)| conflicts(f, g)))
            }),
            _ => false,
        },
        (Ty::Func(f), Ty::Func(g)) => {
            let arity = match (f.arity, g.arity) {
                (Some((lo, hi)), Some((n, _))) => n < lo || n > hi,
                _ => false,
            };
            arity || conflicts(&f.ret, &g.ret)
        }
        _ => false,
    }
}

/// Does a written type use a type variable (`a`)?
fn has_vars(t: &TypeExpr) -> bool {
    let mut found = false;
    t.each(&mut |x| found |= matches!(x.kind, TypeKind::Var(_)));
    found
}

/// The type variables of a call, each with the type it was found to be and where.
type Vars = HashMap<Sym, (Ty, Span)>;

/// How many times a type of the program with parameters is unfolded inside
/// itself (`Tree(Int)` holds two `Tree(Int)`): deeper, nothing is known.
const UNFOLD: usize = 2;

/// The help for a number written where the other kind is expected.
fn number_help(actual: &Ty, written: &Ty) -> Option<String> {
    match (actual.kinds(), written) {
        (INT, Ty::Float) => Some(t!(
            "a whole number is an Int, not a Float: write it with a decimal point (`2.0`), or convert it with `float(...)`",
            "un nombre entier est un Int, pas un Float : écrivez-le avec un point décimal (`2.0`), ou convertissez-le avec `float(...)`"
        )),
        (FLOAT, Ty::Int) => Some(t!(
            "a decimal number is a Float, not an Int: convert it with `int(...)`, `round(...)` or `floor(...)`",
            "un nombre décimal est un Float, pas un Int : convertissez-le avec `int(...)`, `round(...)` ou `floor(...)`"
        )),
        _ => None,
    }
}

fn written_help() -> String {
    t!(
        "change the value, or the type written for it; types written in the code are checked before running, and are never needed (see `rythme help types`)",
        "changez la valeur, ou le type écrit pour elle ; les types écrits dans le code sont vérifiés avant l'exécution, et ne sont jamais obligatoires (voir `rythme help types`)"
    )
}

impl Checker<'_> {
    /// Reports an E027 once: the same code may be checked several times.
    fn report(&mut self, d: Diagnostic) {
        if !self.diags.iter().any(|x| x.code == d.code && x.span == d.span && x.message == d.message) {
            self.diags.push(d);
        }
    }

    // ----- from written types to the checker's types ----------------------

    /// A type, as shown in a message: `Int`, `List(Text)`, `Option(Int)`,
    /// `Tree(Int)`, `{name: Text}`... (`_` for what is not known inside it).
    fn show(&self, t: &Ty) -> String {
        let inner = |t: &Ty| if t.is_known() { self.show(t) } else { "_".to_string() };
        match t {
            Ty::List(x) if x.is_known() => format!("List({})", self.show(x)),
            Ty::Set(x) if x.is_known() => format!("Set({})", self.show(x)),
            Ty::Dict(k, v) if k.is_known() || v.is_known() => format!("Dict({}, {})", inner(k), inner(v)),
            Ty::Record(r) if !r.fields.is_empty() => {
                let fs: Vec<String> = r.fields.iter().map(|(k, t)| format!("{k}: {}", inner(t))).collect();
                format!("{{{}}}", fs.join(", "))
            }
            Ty::Data(d) if matches!(&*d.name.name(), "Option" | "Result") => {
                let field = |c: &str| d.fields_of(c, 1).and_then(|f| f.into_iter().next()).unwrap_or(Ty::Any);
                if &*d.name.name() == "Option" {
                    let x = field("Some");
                    if x.is_known() {
                        return format!("Option({})", self.show(&x));
                    }
                } else {
                    let (ok, err) = (field("Ok"), field("Err"));
                    if ok.is_known() || err.is_known() {
                        return format!("Result({}, {})", inner(&ok), inner(&err));
                    }
                }
                t.name()
            }
            // `Tree(Int)`: the type variables of the type, as found in the value.
            Ty::Data(d) => match self.params_in(d) {
                Some((params, found)) if !found.is_empty() => {
                    let shown: Vec<String> = params.iter().map(|p| found.get(p).map_or("_".to_string(), |(t, _)| inner(t))).collect();
                    format!("{}({})", d.name, shown.join(", "))
                }
                _ => t.name(),
            },
            _ => t.name(),
        }
    }

    /// How a value is shown in an E027 message: like at run time for a value
    /// written in the code (`Int 0`), or else by its whole type (`List(Text)`).
    fn shown(&self, e: Option<&Expr>, t: &Ty) -> String {
        match e.and_then(literal) {
            Some(v) => v.describe(),
            None => t!("a value of type {}", "une valeur de type {}", self.show(t)),
        }
    }

    /// What the type variables of the type of a value of type `d` are
    /// (`type Tree(a)`), found in the fields of the constructors it may have
    /// been built with: a type of the program with parameters only.
    fn params_in(&self, d: &DataTy) -> Option<(&[Sym], Vars)> {
        let params = self.type_params.get(&d.name).filter(|_| self.is_program_type(d.name))?;
        let mut found = Vars::new();
        for (c, fields) in d.ctors.iter().flatten() {
            let Some(w) = self.written_ctor(*c).filter(|w| w.params.len() == fields.len()) else { continue };
            for ((_, ft), f) in w.params.iter().zip(fields) {
                if let Some(ft) = ft {
                    // A value that does not keep to its own type: its first one counts.
                    let _ = self.bind_vars(ft, f, Span::default(), &mut found, w.module);
                }
            }
        }
        Some((params, found))
    }

    /// The type a written type means. Type variables are what `vars` says,
    /// or anything; a type that does not exist (already reported) is anything.
    pub(super) fn written(&self, t: &TypeExpr, vars: &Vars, module: Option<Sym>) -> Ty {
        self.written_in(t, vars, module, 0)
    }

    /// `written`, `unfolded` levels deep inside types of the program with parameters.
    fn written_in(&self, t: &TypeExpr, vars: &Vars, module: Option<Sym>, unfolded: usize) -> Ty {
        let w = |t: &TypeExpr| self.written_in(t, vars, module, unfolded);
        match &t.kind {
            TypeKind::Var(v) => vars.get(v).map_or(Ty::Any, |(t, _)| t.clone()),
            TypeKind::List(x) => Ty::list(w(x)),
            TypeKind::Record(fields) => Ty::record(fields.iter().map(|(k, t)| (k.clone(), w(t))).collect(), false),
            TypeKind::Func(params, ret) => Ty::func(Some((params.len(), params.len())), w(ret), None),
            TypeKind::Named(n, args) => {
                let arg = |i: usize| args.get(i).map_or(Ty::Any, w);
                // A type of the program: maybe of a file used with a name.
                let own = own_type(*n, module);
                if self.is_program_type(own) {
                    return self.program_type(own, args, vars, module, unfolded);
                }
                match &*n.name() {
                    "Int" => Ty::Int,
                    "Float" => Ty::Float,
                    "Text" => Ty::Text,
                    "Bool" => Ty::Bool,
                    "Unit" => Ty::Unit,
                    "Symbol" => Ty::Symbol,
                    "List" => Ty::list(arg(0)),
                    "Set" => Ty::set(arg(0)),
                    "Dict" => Ty::dict(arg(0), arg(1)),
                    "Option" if !args.is_empty() => Ty::option(arg(0)),
                    "Result" if !args.is_empty() => Ty::result(arg(0), arg(1)),
                    "Option" | "Result" => Ty::data(*n, None),
                    "Record" => Ty::record(Vec::new(), false),
                    "Function" => Ty::func(None, Ty::Any, None),
                    _ => Ty::Any,
                }
            }
        }
    }

    /// A type of the program: `Tree(Int)`, with the types it takes, is a
    /// Tree whose constructors have the types written for their fields, its
    /// type variables (`type Tree(a)`) being these types. Without them, or
    /// for a type without parameters, nothing is known of its fields here.
    fn program_type(&self, own: Sym, args: &[TypeExpr], vars: &Vars, module: Option<Sym>, unfolded: usize) -> Ty {
        let (Some(params), Some(ctors)) = (self.type_params.get(&own), self.types.get(&own)) else { return Ty::data(own, None) };
        if args.len() != params.len() || unfolded >= UNFOLD || self.types_defined_again.contains(&own) {
            return Ty::data(own, None);
        }
        let given: Vars =
            params.iter().zip(args).map(|(p, a)| (*p, (self.written_in(a, vars, module, unfolded + 1), Span::default()))).collect();
        let ctors = ctors
            .iter()
            .map(|c| {
                let fields = match self.written_ctor(*c) {
                    Some(w) => w
                        .params
                        .iter()
                        .map(|(_, t)| t.as_ref().map_or(Ty::Any, |t| self.written_in(t, &given, w.module, unfolded + 1)))
                        .collect(),
                    None => vec![Ty::Any; self.ctors.get(c).map_or(0, |m| m.arity)],
                };
                (*c, fields)
            })
            .collect();
        Ty::data(own, Some(ctors)).limit()
    }

    /// What is written of the types of the fields of a constructor.
    fn written_ctor(&self, ctor: Sym) -> Option<&Rc<Written>> {
        self.own_written.get(&ctor).or_else(|| self.know.written.get(&ctor))
    }

    /// Finds what the type variables of `written` (written in the file used
    /// as `module`, if any) are in a value of type `actual` (given at `at`);
    /// gives a variable already found to be a type that `actual` surely is
    /// not: the variable, what it was, where, and what it is here.
    fn bind_vars(&self, written: &TypeExpr, actual: &Ty, at: Span, vars: &mut Vars, module: Option<Sym>) -> Option<(Sym, Ty, Span, Ty)> {
        if !actual.is_known() {
            return None;
        }
        let mut go = |w: &TypeExpr, a: &Ty| self.bind_vars(w, a, at, vars, module);
        match (&written.kind, actual) {
            (TypeKind::Var(v), _) => match vars.get(v) {
                Some((before, where_)) if conflicts(actual, before) => Some((*v, before.clone(), *where_, actual.clone())),
                Some(_) => None,
                None => {
                    vars.insert(*v, (actual.clone(), at));
                    None
                }
            },
            (TypeKind::List(x), Ty::List(e)) => go(x, e),
            (TypeKind::Named(n, args), Ty::Data(d))
                if !args.is_empty() && d.name == own_type(*n, module) && self.is_program_type(d.name) =>
            {
                // `Tree(a)`: what the type variables of `type Tree(t)` are in
                // the value, then `a`.
                let (params, found) = self.params_in(d).filter(|(ps, _)| ps.len() == args.len())?;
                params.iter().zip(args).find_map(|(p, a)| found.get(p).and_then(|(t, _)| go(a, t)))
            }
            (TypeKind::Named(n, args), _) => match (&*n.name(), args.as_slice(), actual) {
                ("List", [x], Ty::List(e)) | ("Set", [x], Ty::Set(e)) => go(x, e),
                ("Dict", [k, v], Ty::Dict(a, b)) => go(k, a).or_else(|| go(v, b)),
                ("Option", [x], Ty::Data(d)) if &*d.name.name() == "Option" => go(x, &d.payload()),
                ("Result", [x, y], Ty::Data(d)) if &*d.name.name() == "Result" => {
                    let err = d.fields_of("Err", 1).and_then(|f| f.into_iter().next()).unwrap_or(Ty::Never);
                    go(x, &d.payload()).or_else(|| go(y, &err))
                }
                _ => None,
            },
            (TypeKind::Record(fields), Ty::Record(r)) => fields.iter().find_map(|(k, t)| r.get(k).and_then(|a| go(t, a))),
            (TypeKind::Func(_, ret), Ty::Func(f)) => go(ret, &f.ret),
            _ => None,
        }
    }

    /// The type written for parameter `i` of a function, without type variables.
    pub(super) fn written_param(&self, def: &FnDef, i: usize) -> Option<Ty> {
        def.param_type(i).map(|t| self.written(t, &Vars::new(), None))
    }

    /// What is written of the types of the function or constructor called
    /// by `name` (resolved as `res`), or of `def`.
    pub(super) fn written_of(&self, name: Option<Sym>, res: &Res, def: Option<&FnDef>) -> Option<Rc<Written>> {
        if let Some(def) = def {
            return Written::of_fn(def).map(Rc::new);
        }
        let name = name?;
        match res {
            Res::Global(g) if matches!(g.kind, GKind::Ctor | GKind::Func) => {
                self.own_written.get(&name).or_else(|| self.know.written.get(&name)).cloned()
            }
            _ => None,
        }
    }

    /// The types of the fields of a constructor, as written (anything when not written).
    pub(super) fn written_fields(&self, ctor: Sym, n: usize) -> Option<Vec<Ty>> {
        let w = self.own_written.get(&ctor).or_else(|| self.know.written.get(&ctor))?;
        if w.params.len() != n {
            return None;
        }
        Some(w.params.iter().map(|(_, t)| t.as_ref().map_or(Ty::Any, |t| self.written(t, &Vars::new(), w.module))).collect())
    }

    // ----- checking values against written types ---------------------------

    /// `name: Type = value`: the value must be of the written type. Gives
    /// the type the names of the pattern get: the written one.
    pub(super) fn written_value(&mut self, p: &Pattern, e: &Expr, t: &Ty) -> Ty {
        let Some(written) = &p.ty else { return t.clone() };
        let wt = self.written(written, &Vars::new(), None);
        if conflicts(t, &wt) {
            let what = self.shown(Some(e), t);
            let w = written.to_string();
            let (msg, secondary) = match &p.kind {
                PatKind::Bind(name) => (
                    t!("`{name}` is written `{w}`, but its value is {what}", "`{name}` est écrit `{w}`, mais sa valeur est {what}"),
                    t!("the type written for `{name}`", "le type écrit pour `{name}`"),
                ),
                _ => (
                    t!(
                        "this value is {what}, but the type written for it is `{w}`",
                        "cette valeur est {what}, mais le type écrit pour elle est `{w}`"
                    ),
                    t!("the type written here", "le type écrit ici"),
                ),
            };
            let mut d =
                Diagnostic::error("E027", msg).at(result_span(e)).label(t!("not `{w}`", "pas `{w}`")).secondary(written.span, secondary);
            d = match number_help(t, &wt) {
                Some(h) => d.help(h),
                None => d.help(written_help()),
            };
            self.report(d);
        }
        if wt.is_known() {
            wt
        } else {
            t.clone()
        }
    }

    /// The arguments of a call to a function or a constructor that writes
    /// the types of its parameters. Reports those that surely do not fit
    /// (true if any), and gives what the type variables were found to be.
    pub(super) fn written_args(&mut self, w: &Written, exprs: &[&Expr], tys: &[Ty]) -> (bool, Vars) {
        let mut vars = Vars::new();
        // A function checked again with the arguments of a call looks for the
        // error it stops with when it runs: written types are not one.
        if self.needs.specializing > 0 {
            for ((_, written), (e, t)) in w.params.iter().zip(exprs.iter().zip(tys)) {
                if let Some(written) = written {
                    self.bind_vars(written, t, e.span, &mut vars, w.module);
                }
            }
            return (false, vars);
        }
        let mut reported = false;
        let f = &w.name;
        for ((param, written), (e, t)) in w.params.iter().zip(exprs.iter().zip(tys)) {
            let Some(written) = written else { continue };
            if let Some((v, before, at, now)) = self.bind_vars(written, t, e.span, &mut vars, w.module) {
                let (a, b) = (self.show(&before), self.show(&now));
                self.report(
                    Diagnostic::error(
                        "E027",
                        t!(
                            "`{v}` cannot be both {a} and {b} in this call to `{f}`",
                            "`{v}` ne peut pas être à la fois {a} et {b} dans cet appel à `{f}`"
                        ),
                    )
                    .at(e.span)
                    .label(t!("`{v}` is {b} here", "`{v}` est {b} ici"))
                    .secondary(at, t!("`{v}` is {a} here", "`{v}` est {a} ici"))
                    .note(t!(
                        "a type variable like `{v}` stands for the same type everywhere in the types written for `{f}`",
                        "une variable de type comme `{v}` désigne le même type partout dans les types écrits pour `{f}`"
                    )),
                );
                reported = true;
                continue;
            }
            let wt = self.written(written, &vars, w.module);
            if conflicts(t, &wt) {
                let what = self.shown(Some(e), t);
                // `Shape` of a file used with a name is `geometry.Shape` here.
                let ws = if w.module.is_some() && matches!(wt, Ty::Data(_)) { self.show(&wt) } else { written.to_string() };
                let mut d = Diagnostic::error(
                    "E027",
                    t!("`{f}` expects `{ws}` for `{param}`, but receives {what}", "`{f}` attend `{ws}` pour `{param}`, mais reçoit {what}"),
                )
                .at(e.span)
                .label(t!("not `{ws}`", "pas `{ws}`"))
                .secondary(written.span, t!("the type written for `{param}`", "le type écrit pour `{param}`"));
                d = match number_help(t, &wt) {
                    Some(h) => d.help(h),
                    None => d.help(written_help()),
                };
                self.report(d);
                reported = true;
            }
        }
        (reported, vars)
    }

    /// What a call gives, with the type written for the result: with type
    /// variables, the type found for them in the arguments.
    pub(super) fn written_result(&self, w: &Written, vars: &Vars, inferred: Ty) -> Ty {
        let Some(ret) = &w.ret else { return inferred };
        let wt = self.written(ret, vars, w.module);
        if wt.is_known() && (has_vars(ret) || !inferred.is_known() || conflicts(&inferred, &wt)) {
            wt
        } else {
            inferred
        }
    }

    /// A value given to a function written in place (a block given to `map`)
    /// whose parameter has a written type: it must fit.
    pub(super) fn written_received(&mut self, p: &Pattern, given: &Ty) {
        let Some(written) = &p.ty else { return };
        let wt = self.written(written, &Vars::new(), None);
        if !conflicts(given, &wt) {
            return;
        }
        let (what, w) = (self.shown(None, given), written.to_string());
        let param = match &p.kind {
            PatKind::Bind(s) => s.to_string(),
            _ => "_".to_string(),
        };
        let mut d = Diagnostic::error(
            "E027",
            t!(
                "this function receives {what}, but `{param}` is written `{w}`",
                "cette fonction reçoit {what}, mais `{param}` est écrit `{w}`"
            ),
        )
        .at(written.span)
        .label(t!("the type written for `{param}`", "le type écrit pour `{param}`"));
        d = match number_help(given, &wt) {
            Some(h) => d.help(h),
            None => d.help(written_help()),
        };
        self.report(d);
    }

    /// The result of a definition of a function with a written result type:
    /// it must fit.
    pub(super) fn written_body(&mut self, def: &FnDef, body: &Expr, t: &Ty) {
        let Some(ret) = def.ret_type() else { return };
        let wt = self.written(ret, &Vars::new(), None);
        if !conflicts(t, &wt) {
            return;
        }
        let at = result_expr(body);
        let (f, what, w) = (def.name.map_or_else(|| "fn".to_string(), |n| n.to_string()), self.shown(Some(at), t), ret.to_string());
        let mut d = Diagnostic::error(
            "E027",
            t!("`{f}` is written to give `{w}`, but gives {what}", "`{f}` est écrite pour donner `{w}`, mais donne {what}"),
        )
        .at(at.span)
        .label(t!("not `{w}`", "pas `{w}`"))
        .secondary(ret.span, t!("the type written for the result of `{f}`", "le type écrit pour le résultat de `{f}`"));
        d = match number_help(t, &wt) {
            Some(h) => d.help(h),
            None => d.help(written_help()),
        };
        self.report(d);
    }

    /// What `?` may give back from a function with a written result type: it must fit.
    pub(super) fn written_early(&mut self, def: &FnDef, early: &Ty) {
        let Some(ret) = def.ret_type() else { return };
        let wt = self.written(ret, &Vars::new(), None);
        if !conflicts(early, &wt) {
            return;
        }
        let (f, what, w) = (def.name.map_or_else(|| "fn".to_string(), |n| n.to_string()), self.shown(None, early), ret.to_string());
        self.report(
            Diagnostic::error(
                "E027",
                t!("`{f}` is written to give `{w}`, but its `?` may give back {what}", "`{f}` est écrite pour donner `{w}`, mais son `?` peut renvoyer {what}"),
            )
            .at(ret.span)
            .label(t!("the type written for the result of `{f}`", "le type écrit pour le résultat de `{f}`"))
            .help(t!(
                "`?` gives back the `None` or `Err(...)` it stops on: write the result as an Option or a Result, like `-> Option(Int)`",
                "`?` renvoie le `None` ou le `Err(...)` sur lequel il s'arrête : écrivez le résultat comme une Option ou un Result, comme `-> Option(Int)`"
            )),
        );
    }

    /// What a function gives, once checked, with the type written for its result.
    pub(super) fn with_written_result(&self, def: &FnDef, inferred: Ty) -> Ty {
        match def.ret_type() {
            Some(ret) => {
                let wt = self.written(ret, &Vars::new(), None);
                if wt.is_known() && (!inferred.is_known() || conflicts(&inferred, &wt)) {
                    wt
                } else {
                    inferred
                }
            }
            None => inferred,
        }
    }

    // ----- the written types themselves -----------------------------------

    /// Reports the written types that name no type, or take a wrong number
    /// of types: everywhere in the module, once.
    pub(super) fn check_written_types(&mut self, items: &[Item]) {
        let mut found: Vec<Rc<TypeExpr>> = Vec::new();
        for item in items {
            match item {
                Item::Stmt(s) => written_in_stmt(s, &mut found),
                Item::Type(t) => self.check_type_def(t),
                Item::Test(_, body, _) => written_in_expr(body, &mut found),
                Item::Use(_) => {}
            }
        }
        for t in found {
            t.each(&mut |x| self.check_written(x, None));
        }
    }

    /// The parameters of a type defined with `type` (`type Tree(a)`), and
    /// the types written for the fields of its constructors.
    fn check_type_def(&mut self, def: &TypeDef) {
        let mut seen: Vec<Sym> = Vec::new();
        for (p, span) in &def.params {
            let name = p.name();
            let ty = def.name;
            if seen.contains(p) {
                self.report(
                    Diagnostic::error(
                        "E027",
                        t!(
                            "the type variable `{name}` appears twice in `{ty}`",
                            "la variable de type `{name}` apparaît deux fois dans `{ty}`"
                        ),
                    )
                    .at(*span)
                    .help(t!("give each one its own letter: `type Pair(a, b)`", "donnez à chacune sa propre lettre : `type Paire(a, b)`")),
                );
            } else if name.chars().count() != 1 {
                let letter: String = name.chars().filter(char::is_ascii_lowercase).take(1).collect();
                let letter = if letter.is_empty() { "a".to_string() } else { letter };
                self.report(
                    Diagnostic::error(
                        "E027",
                        t!(
                            "`{name}` cannot be a type variable: a type variable is one lowercase letter",
                            "`{name}` ne peut pas être une variable de type : une variable de type est une seule lettre minuscule"
                        ),
                    )
                    .at(*span)
                    .help(t!(
                        "write one letter, like `{letter}`, here and in the fields",
                        "écrivez une lettre, comme `{letter}`, ici et dans les champs"
                    ))
                    .note(type_var_note()),
                );
            }
            seen.push(*p);
        }
        for v in &def.variants {
            for t in v.types.iter().flatten() {
                t.each(&mut |x| self.check_written(x, Some(def)));
            }
        }
    }

    /// Reports a written type that names no type, or takes a wrong number of
    /// types; `def`: the type whose field it is written for, if any.
    fn check_written(&mut self, t: &TypeExpr, def: Option<&TypeDef>) {
        match &t.kind {
            // In a type with parameters, the type variables are its parameters.
            TypeKind::Var(v) if def.is_some_and(|d| d.params.iter().any(|(p, _)| p == v)) => {}
            TypeKind::Var(v) if def.is_some_and(|d| !d.params.is_empty()) && v.name().chars().count() == 1 => {
                let Some(d) = def else { return };
                let (ty, header) = (d.name, d.header());
                let mut params: Vec<String> = d.params.iter().map(|(p, _)| p.to_string()).collect();
                params.push(v.to_string());
                let with = format!("{ty}({})", params.join(", "));
                self.report(
                    Diagnostic::error("E027", t!("`{v}` is not a parameter of `{ty}`", "`{v}` n'est pas un paramètre de `{ty}`"))
                        .at(t.span)
                        .label(t!("not among `{header}`", "pas parmi `{header}`"))
                        .help(t!(
                            "add it to the parameters of the type: `type {with} = ...`",
                            "ajoutez-la aux paramètres du type : `type {with} = ...`"
                        )),
                );
            }
            TypeKind::Var(v) => {
                let name = v.name();
                if name.chars().count() == 1 {
                    return;
                }
                let mut d = Diagnostic::error("E027", t!("unknown type `{name}`", "type inconnu `{name}`")).at(t.span);
                if let Some(best) = other_name(&name) {
                    d = d.help(t!("in Rythme, write `{best}`", "en Rythme, écrivez `{best}`"));
                    d = crate::fix::guess(d, t.span, &name, best);
                    d.fixes.iter_mut().for_each(|f| f.safe = true);
                } else if let Some(best) = crate::suggest::closest(&capitalized(&name), self.type_names().iter().map(|s| &**s)) {
                    d = crate::fix::guess(d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?")), t.span, &name, &best);
                }
                self.report(d.note(type_var_note()));
            }
            TypeKind::Named(n, args) => {
                let name = n.name();
                if self.is_program_type(*n) {
                    // Alone, a type with parameters may hold anything (like `List`).
                    let count = self.type_params.get(n).map_or(0, Vec::len);
                    if args.is_empty() || args.len() == count || self.params_unknown.contains(n) {
                        return;
                    }
                    let example = format!(
                        "{name}({})",
                        ["Int", "Text", "Bool", "Float"].iter().cycle().take(count).copied().collect::<Vec<_>>().join(", ")
                    );
                    let d = match count {
                        0 => Diagnostic::error(
                            "E027",
                            t!("`{name}` takes no type between parentheses", "`{name}` ne prend pas de type entre parenthèses"),
                        )
                        .help(t!("write `{name}` alone", "écrivez `{name}` seul")),
                        1 => Diagnostic::error(
                            "E027",
                            t!("`{name}` takes one type: `{example}`", "`{name}` prend un seul type : `{example}`"),
                        ),
                        2 => Diagnostic::error(
                            "E027",
                            t!("`{name}` takes two types: `{example}`", "`{name}` prend deux types : `{example}`"),
                        ),
                        _ => Diagnostic::error(
                            "E027",
                            t!("`{name}` takes {count} types: `{example}`", "`{name}` prend {count} types : `{example}`"),
                        ),
                    };
                    self.report(d.at(t.span));
                    return;
                }
                let Some(counts) = builtin_type(&name) else {
                    let mut d = Diagnostic::error("E027", t!("unknown type `{name}`", "type inconnu `{name}`")).at(t.span);
                    if let Some(best) = other_name(&name) {
                        d = d.help(t!("in Rythme, write `{best}`", "en Rythme, écrivez `{best}`"));
                        d = crate::fix::guess(d, t.span, &name, best);
                        d.fixes.iter_mut().for_each(|f| f.safe = true);
                    } else if let Some(best) = crate::suggest::closest(&name, self.type_names().iter().map(|s| &**s)) {
                        d = crate::fix::guess(d.help(t!("did you mean `{best}`?", "vouliez-vous dire `{best}` ?")), t.span, &name, &best);
                    } else {
                        d = d.help(t!(
                            "a type is one of Rythme's (`Int`, `Float`, `Text`, `Bool`, `List`, `Dict`...) or one defined with `type`",
                            "un type est l'un de ceux de Rythme (`Int`, `Float`, `Text`, `Bool`, `List`, `Dict`...) ou un type défini avec `type`"
                        ));
                    }
                    self.report(d);
                    return;
                };
                if counts.contains(&args.len()) {
                    return;
                }
                let example = match &*name {
                    "List" => "List(Int)",
                    "Set" => "Set(Text)",
                    "Option" => "Option(Int)",
                    "Dict" => "Dict(Text, Int)",
                    "Result" => "Result(Int, Text)",
                    _ => "",
                };
                let d = match counts {
                    [0] => Diagnostic::error(
                        "E027",
                        t!("`{name}` takes no type between parentheses", "`{name}` ne prend pas de type entre parenthèses"),
                    )
                    .help(t!("write `{name}` alone", "écrivez `{name}` seul")),
                    [0, 1] => {
                        Diagnostic::error("E027", t!("`{name}` takes one type: `{example}`", "`{name}` prend un seul type : `{example}`"))
                    }
                    _ => Diagnostic::error("E027", t!("`{name}` takes two types: `{example}`", "`{name}` prend deux types : `{example}`")),
                };
                self.report(d.at(t.span));
            }
            _ => {}
        }
    }

    /// Is this a type defined with `type` (by this file or another one)?
    /// `Option` and `Result` are Rythme's, and take types.
    fn is_program_type(&self, name: Sym) -> bool {
        self.types.contains_key(&name) && !matches!(&*name.name(), "Option" | "Result")
    }

    /// The names of the types a written type may name.
    fn type_names(&self) -> Vec<Rc<str>> {
        let mut names: Vec<Rc<str>> =
            ["Int", "Float", "Text", "Bool", "Unit", "Symbol", "List", "Set", "Dict", "Option", "Result", "Record", "Function"]
                .iter()
                .map(|n| Rc::from(*n))
                .collect();
        names.extend(self.types.keys().map(|t| t.name()));
        names
    }
}

fn type_var_note() -> String {
    t!(
        "the names of types start with an uppercase letter; a type variable, which stands for any type, is one lowercase letter, like `a`",
        "les noms de types commencent par une majuscule ; une variable de type, qui désigne n'importe quel type, est une seule lettre minuscule, comme `a`"
    )
}

/// The name of a type written in the file used as `module` (if any), as the
/// checker knows it: `Shape` of `use geometry` is `geometry.Shape`.
fn own_type(n: Sym, module: Option<Sym>) -> Sym {
    match module {
        Some(m) if !n.name().contains('.') => Sym::new(&format!("{m}.{n}")),
        _ => n,
    }
}

/// The expression that gives the value of a body: the last line of a block.
fn result_expr(body: &Expr) -> &Expr {
    match &body.kind {
        ExprKind::Block(stmts) => match stmts.last() {
            Some(Stmt::Expr(e)) => result_expr(e),
            _ => body,
        },
        _ => body,
    }
}

/// Where the value of an expression is given, for a message.
fn result_span(e: &Expr) -> Span {
    result_expr(e).span
}

fn written_in_def(def: &FnDef, out: &mut Vec<Rc<TypeExpr>>) {
    for c in &def.clauses {
        out.extend(c.params.iter().filter_map(|p| p.ty.clone()));
        out.extend(c.ret.clone());
        if let Some(g) = &c.guard {
            written_in_expr(g, out);
        }
        written_in_expr(&c.body, out);
    }
}

fn written_in_stmt(s: &Stmt, out: &mut Vec<Rc<TypeExpr>>) {
    match s {
        Stmt::Let(p, e) => {
            out.extend(p.ty.clone());
            written_in_expr(e, out);
        }
        Stmt::Expr(e) => written_in_expr(e, out),
        Stmt::Fn(def) => written_in_def(def, out),
    }
}

fn written_in_expr(e: &Expr, out: &mut Vec<Rc<TypeExpr>>) {
    match &e.kind {
        ExprKind::Lambda(def) => return written_in_def(def, out),
        ExprKind::Block(stmts) => return stmts.iter().for_each(|s| written_in_stmt(s, out)),
        _ => {}
    }
    super::each_inner(e, &mut |x| written_in_expr(x, out));
}

/// `text` for `Text`.
fn capitalized(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}
