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

/// A fast hasher for maps keyed by names (a name is just a small integer).
#[derive(Default, Clone, Copy)]
pub struct SymHasher(u64);

impl std::hash::Hasher for SymHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }

    fn write_u32(&mut self, n: u32) {
        self.0 = (n as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

pub type SymMap<V> = HashMap<Sym, V, std::hash::BuildHasherDefault<SymHasher>>;

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
