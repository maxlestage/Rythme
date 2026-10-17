//! Interned names: comparing two names is comparing two integers.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sym(u32);

#[derive(Default)]
struct Interner {
    map: HashMap<Rc<str>, Sym>,
    names: Vec<Rc<str>>,
}

thread_local! {
    static INTERNER: RefCell<Interner> = RefCell::new(Interner::default());
}

impl Sym {
    pub fn new(name: &str) -> Sym {
        INTERNER.with(|i| {
            let mut i = i.borrow_mut();
            if let Some(&s) = i.map.get(name) {
                return s;
            }
            let s = Sym(i.names.len() as u32);
            let rc: Rc<str> = Rc::from(name);
            i.names.push(rc.clone());
            i.map.insert(rc, s);
            s
        })
    }

    pub fn name(self) -> Rc<str> {
        INTERNER.with(|i| i.borrow().names[self.0 as usize].clone())
    }

    /// Names starting with an uppercase letter are constructors (`Some`, `Circle`).
    pub fn is_ctor(self) -> bool {
        self.name().chars().next().is_some_and(|c| c.is_uppercase())
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}

impl fmt::Debug for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}`", self.name())
    }
}
