//! The value stack of the interpreter, which holds the registers of the
//! running functions (see the module `vm`).
//!
//! It is a vector that does not give back the room above its length: the
//! values there stay, as long as they hold no memory of their own (whole
//! numbers, `()`...), and a frame that needs more registers takes them back
//! without writing them. A function given to `map` is called a million
//! times; each call makes its frame and those of the functions it calls,
//! then removes them: with a plain vector, every call wrote all of these
//! registers again (and freed them one by one).

pub(crate) use crate::value::is_plain;
use crate::value::Value;

/// A stack of values (see the module). Every value of `vals` above `len`
/// holds no memory of its own (`is_plain`).
#[derive(Default)]
pub struct Stack {
    vals: Vec<Value>,
    len: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack::default()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// How many values it holds, the room above its length included.
    pub fn capacity(&self) -> usize {
        self.vals.capacity()
    }

    /// The first value. Pointers into the stack stay valid while it does not grow.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut Value {
        self.vals.as_mut_ptr()
    }

    #[inline(always)]
    pub fn push(&mut self, v: Value) {
        if self.len < self.vals.len() {
            self.vals[self.len] = v;
        } else {
            self.vals.push(v);
        }
        self.len += 1;
    }

    pub fn extend(&mut self, vs: impl IntoIterator<Item = Value>) {
        for v in vs {
            self.push(v);
        }
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(std::mem::replace(&mut self.vals[self.len], Value::Unit))
    }

    /// Makes the stack `len` values long, if it is shorter. The values it
    /// takes back are what was left there (`()`, or numbers): registers are
    /// written before they are read.
    #[inline(always)]
    pub fn grow_to(&mut self, len: usize) {
        if self.len < len {
            if self.vals.len() < len {
                self.make_room(len);
            }
            self.len = len;
        }
    }

    #[inline(never)]
    fn make_room(&mut self, len: usize) {
        self.vals.resize_with(len, || Value::Unit);
    }

    /// Frees the values `from..to` that hold memory (those below the length).
    #[inline(always)]
    pub fn reset(&mut self, from: usize, to: usize) {
        let to = to.min(self.len);
        if let Some(vals) = self.vals.get_mut(from..to) {
            for v in vals {
                if !is_plain(v) {
                    *v = Value::Unit;
                }
            }
        }
    }

    /// Removes the values above `len`, freeing those that hold memory.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            self.reset(len, self.len);
            self.len = len;
        }
    }

    /// Removes the values above `len`, which hold no memory of their own
    /// (the frames above it were freed when their calls ended).
    #[inline(always)]
    pub fn drop_plain(&mut self, len: usize) {
        debug_assert!(self.vals[len.min(self.len)..self.len].iter().all(is_plain), "values that hold memory above the stack");
        self.len = self.len.min(len);
    }
}

impl Stack {
    /// Swaps two values below the length.
    #[inline(always)]
    pub fn swap(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.len && b < self.len);
        self.vals.swap(a, b);
    }
}

// Indexing checks the position against the values written (the room above
// the length holds values too): one comparison instead of two, on the paths
// of every call.
impl std::ops::Index<usize> for Stack {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        debug_assert!(i < self.len);
        &self.vals[i]
    }
}

impl std::ops::IndexMut<usize> for Stack {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        debug_assert!(i < self.len);
        &mut self.vals[i]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Stack {
    type Output = [Value];

    #[inline(always)]
    fn index(&self, r: std::ops::Range<usize>) -> &[Value] {
        debug_assert!(r.end <= self.len);
        &self.vals[r]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Stack {
    #[inline(always)]
    fn index_mut(&mut self, r: std::ops::Range<usize>) -> &mut [Value] {
        debug_assert!(r.end <= self.len);
        &mut self.vals[r]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for Stack {
    type Output = [Value];

    #[inline(always)]
    fn index(&self, r: std::ops::RangeFrom<usize>) -> &[Value] {
        &self.vals[r.start..self.len]
    }
}

impl std::ops::IndexMut<std::ops::RangeFrom<usize>> for Stack {
    #[inline(always)]
    fn index_mut(&mut self, r: std::ops::RangeFrom<usize>) -> &mut [Value] {
        &mut self.vals[r.start..self.len]
    }
}

impl std::ops::Deref for Stack {
    type Target = [Value];

    #[inline(always)]
    fn deref(&self) -> &[Value] {
        &self.vals[..self.len]
    }
}

impl std::ops::DerefMut for Stack {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [Value] {
        &mut self.vals[..self.len]
    }
}
