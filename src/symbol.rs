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

    /// Names starting with an uppercase letter are constructors (`Some`,
    /// `Circle`, and `geo.Circle` for one of the file used as `geo`).
    pub fn is_ctor(self) -> bool {
        let name = self.name();
        let own = name.rsplit('.').next().unwrap_or(&name);
        own.chars().next().is_some_and(|c| c.is_uppercase())
    }

    /// `geo.area`: the name of a file used with `use ... as geo`, and the
    /// name inside it.
    pub fn split_module(self) -> Option<(Sym, Sym)> {
        let name = self.name();
        if name.starts_with('#') {
            return None;
        }
        let (m, n) = name.split_once('.')?;
        Some((Sym::new(m), Sym::new(n)))
    }

    /// The name inside its file: `Circle` for `geo.Circle`.
    pub fn own_name(self) -> Sym {
        match self.split_module() {
            Some((_, n)) => n,
            None => self,
        }
    }

    /// How the interpreter stores a top-level name that must not meet the
    /// names of other files: `#3.area` for `area` in the third file used with
    /// a name. Such names cannot be written in a program.
    pub fn stored(prefix: &str, name: Sym) -> Sym {
        Sym::new(&format!("#{prefix}.{name}"))
    }

    /// Is it a name made by `stored`?
    pub fn is_stored(self) -> bool {
        self.name().starts_with('#')
    }

    /// The name as the program wrote it: `area` for `#3.area`.
    pub fn shown(self) -> Sym {
        let name = self.name();
        match name.strip_prefix('#').and_then(|n| n.split_once('.')) {
            Some((_, n)) => Sym::new(n),
            None => self,
        }
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
