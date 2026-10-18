//! Memory shared by the texts and the lists built one from another, so that
//! adding at the end (`acc + "x"`, `push(xs, x)`, `[..xs, x]`) takes constant
//! time on average instead of copying everything each time.
//!
//! A buffer holds elements one after the other, and room for more. Each
//! value made from it sees a part of the elements written so far (its
//! slice). The elements written never change: adding to a value whose slice
//! ends where the written elements end writes the new elements after them,
//! in the spare room, and the new value sees one more element. The other
//! values do not see the new elements, so for them nothing changed. Adding
//! to any other value (or when the room is full) copies its slice to a new,
//! larger buffer, as before.

use std::alloc::Layout;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::rc::Rc;

/// Elements written one after the other, with room for more (see the module).
pub struct Buf<T> {
    ptr: NonNull<T>,
    cap: usize,
    /// How many elements are written; they never change until the buffer is freed.
    used: Cell<usize>,
    /// Whether the buffer was made by adding to a value: one that grows once
    /// will likely grow again, so the next copy leaves room.
    grows: bool,
    owns: PhantomData<T>,
}

impl<T> Buf<T> {
    /// The elements of a vector, and the room it has after them.
    pub fn from_vec(v: Vec<T>) -> Buf<T> {
        Buf::with_growth(v, false)
    }

    fn with_growth(v: Vec<T>, grows: bool) -> Buf<T> {
        let mut v = ManuallyDrop::new(v);
        // SAFETY: the pointer of a vector is never null.
        let ptr = unsafe { NonNull::new_unchecked(v.as_mut_ptr()) };
        Buf { ptr, cap: v.capacity(), used: Cell::new(v.len()), grows, owns: PhantomData }
    }

    /// How many elements are written.
    #[inline(always)]
    pub fn used(&self) -> usize {
        self.used.get()
    }

    /// The elements `from..to`, which must be written.
    #[inline(always)]
    pub fn slice(&self, from: usize, to: usize) -> &[T] {
        debug_assert!(from <= to && to <= self.used.get());
        // SAFETY: the first `used` elements are written and never change
        // while the buffer lives (only the room after them is written).
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr().add(from), to - from) }
    }

    /// The room after the written elements.
    #[inline(always)]
    pub fn room(&self) -> usize {
        self.cap - self.used.get()
    }

    /// Writes clones of `items` after the written elements, if `end` is where
    /// they end and there is room for them.
    #[inline]
    pub fn extend_at(&self, end: usize, items: &[T]) -> bool
    where
        T: Clone,
    {
        let used = self.used.get();
        if end != used || self.cap - used < items.len() {
            return false;
        }
        for (i, x) in items.iter().enumerate() {
            // SAFETY: `used + i < cap`: the room is allocated, and no value
            // sees it (`items` is among the written elements, or elsewhere).
            unsafe { self.ptr.as_ptr().add(used + i).write(x.clone()) };
            self.used.set(used + i + 1);
        }
        true
    }

    /// Takes all the elements out, leaving the buffer empty.
    pub fn take(&mut self) -> Vec<T> {
        let used = self.used.replace(0);
        let cap = std::mem::replace(&mut self.cap, 0);
        let ptr = std::mem::replace(&mut self.ptr, NonNull::dangling());
        // SAFETY: the buffer came from a vector of this capacity, whose first
        // `used` elements are written; it no longer owns them.
        unsafe { Vec::from_raw_parts(ptr.as_ptr(), used, cap) }
    }
}

impl<T> Drop for Buf<T> {
    fn drop(&mut self) {
        drop(self.take());
    }
}

/// A new buffer with `head` then `tail`, and room after them if `grows`.
fn joined<T: Clone>(head: &[T], tail: &[T], grows: bool) -> Buf<T> {
    let len = head.len() + tail.len();
    let cap = if grows { len.max(8).saturating_mul(2) } else { len };
    let mut v = Vec::with_capacity(cap);
    v.extend_from_slice(head);
    v.extend_from_slice(tail);
    Buf::with_growth(v, true)
}

/// The slice `start..end` of `buf` followed by `tail`: written in place
/// (`None`) if `in_place` allows it, the slice ends where the written
/// elements end and there is room, else in a new buffer, with room if this
/// one already grew.
pub fn append<T: Clone>(buf: &Rc<Buf<T>>, start: usize, end: usize, tail: &[T], in_place: bool) -> Option<Rc<Buf<T>>> {
    if in_place && buf.extend_at(end, tail) {
        return None;
    }
    let grows = buf.grows && end == buf.used();
    Some(Rc::new(joined(buf.slice(start, end), tail, grows)))
}

// ----- texts ------------------------------------------------------------

/// An immutable text. Texts made by adding to another one share its memory
/// (see the module). Like `Rc<str>`, a text is one block of memory: a count
/// of the texts that share it, the bytes written and the room, then the
/// bytes.
pub struct Text {
    ptr: NonNull<TextHead>,
    len: usize,
}

/// The start of the memory of texts, before the bytes.
struct TextHead {
    /// How many texts share this memory.
    refs: Cell<usize>,
    /// How many bytes are written; they never change until the memory is freed.
    used: Cell<usize>,
    /// How many bytes fit.
    cap: usize,
    /// Made by adding to a text (see `Buf::grows`).
    grows: bool,
}

/// Where the bytes start, after the head (whose size is a multiple of its alignment).
const HEAD: usize = std::mem::size_of::<TextHead>();

/// The memory of texts of `cap` bytes.
fn text_layout(cap: usize) -> Layout {
    Layout::from_size_align(HEAD.checked_add(cap).expect("a text too large"), std::mem::align_of::<TextHead>()).expect("a text too large")
}

impl Text {
    /// A new text with `parts` one after the other, with room for `cap` bytes in all.
    fn with(parts: [&[u8]; 2], cap: usize, grows: bool) -> Text {
        let len = parts[0].len() + parts[1].len();
        debug_assert!(len <= cap);
        let layout = text_layout(cap);
        // SAFETY: the layout has a size (the head at least).
        let Some(ptr) = NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<TextHead>()) else {
            std::alloc::handle_alloc_error(layout)
        };
        // SAFETY: the memory is allocated for the head and `cap` bytes after it.
        unsafe {
            ptr.as_ptr().write(TextHead { refs: Cell::new(1), used: Cell::new(len), cap, grows });
            let data = ptr.as_ptr().cast::<u8>().add(HEAD);
            std::ptr::copy_nonoverlapping(parts[0].as_ptr(), data, parts[0].len());
            std::ptr::copy_nonoverlapping(parts[1].as_ptr(), data.add(parts[0].len()), parts[1].len());
        }
        Text { ptr, len }
    }

    #[inline(always)]
    fn head(&self) -> &TextHead {
        // SAFETY: the head lives as long as one text shares the memory.
        unsafe { self.ptr.as_ref() }
    }

    #[inline(always)]
    fn data(&self) -> *mut u8 {
        // SAFETY: the bytes come right after the head, in the same memory.
        unsafe { self.ptr.as_ptr().cast::<u8>().add(HEAD) }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: the first `len` bytes are written and never change; they
        // are whole UTF-8 texts written one after the other.
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.data(), self.len)) }
    }

    /// This text followed by `tail`: in the room after it when it ends where
    /// the written bytes end, so that adding at the end again and again is fast.
    pub fn concat(&self, tail: &str) -> Text {
        let head = self.head();
        let used = head.used.get();
        let len = self.len + tail.len();
        if used == self.len && head.cap - used >= tail.len() {
            // SAFETY: the room is allocated and no text sees it; `tail` is
            // among the written bytes, or elsewhere.
            unsafe { std::ptr::copy_nonoverlapping(tail.as_ptr(), self.data().add(used), tail.len()) };
            head.used.set(len);
            let mut t = self.clone();
            t.len = len;
            return t;
        }
        let grows = head.grows && used == self.len;
        let cap = if grows { len.max(16).saturating_mul(2) } else { len };
        Text::with([self.as_bytes(), tail.as_bytes()], cap, true)
    }
}

impl Clone for Text {
    #[inline]
    fn clone(&self) -> Text {
        let refs = &self.head().refs;
        refs.set(refs.get().checked_add(1).unwrap_or_else(|| std::process::abort()));
        Text { ptr: self.ptr, len: self.len }
    }
}

impl Drop for Text {
    #[inline]
    fn drop(&mut self) {
        let head = self.head();
        let refs = head.refs.get() - 1;
        head.refs.set(refs);
        if refs == 0 {
            let layout = text_layout(head.cap);
            // SAFETY: no other text shares the memory, allocated with this layout.
            unsafe { std::alloc::dealloc(self.ptr.as_ptr().cast::<u8>(), layout) }
        }
    }
}

impl std::ops::Deref for Text {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::borrow::Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for Text {
    fn from(s: String) -> Text {
        Text::from(s.as_str())
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Text {
        Text::with([s.as_bytes(), &[]], s.len(), false)
    }
}

impl From<std::borrow::Cow<'_, str>> for Text {
    fn from(s: std::borrow::Cow<'_, str>) -> Text {
        Text::from(s.into_owned())
    }
}

impl From<&String> for Text {
    fn from(s: &String) -> Text {
        Text::from(s.as_str())
    }
}

impl From<Rc<str>> for Text {
    fn from(s: Rc<str>) -> Text {
        Text::from(&*s)
    }
}

impl From<&Rc<str>> for Text {
    fn from(s: &Rc<str>) -> Text {
        Text::from(&**s)
    }
}

impl From<char> for Text {
    fn from(c: char) -> Text {
        Text::from(c.to_string())
    }
}

impl From<&Text> for Rc<str> {
    fn from(t: &Text) -> Rc<str> {
        Rc::from(t.as_str())
    }
}

impl From<Text> for Rc<str> {
    fn from(t: Text) -> Rc<str> {
        Rc::from(t.as_str())
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Text) -> bool {
        self.len == other.len && (self.ptr == other.ptr || self.as_str() == other.as_str())
    }
}

impl Eq for Text {}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<Rc<str>> for Text {
    fn eq(&self, other: &Rc<str>) -> bool {
        self.as_str() == &**other
    }
}

impl PartialEq<Text> for Rc<str> {
    fn eq(&self, other: &Text) -> bool {
        &**self == other.as_str()
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Text) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Text) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_grow_in_place_and_stay_unchanged() {
        let a = Text::from("ab");
        let b = a.concat("cd");
        let c = b.concat("ef");
        let d = c.concat("gh");
        // `d` grew in the room `c` left, without changing `c`.
        assert_eq!(c.ptr, d.ptr);
        let e = c.concat("XY");
        assert_ne!(c.ptr, e.ptr);
        assert_eq!((&*a, &*b, &*c, &*d, &*e), ("ab", "abcd", "abcdef", "abcdefgh", "abcdefXY"));
        let f = d.concat(&d);
        assert_eq!(&*f, "abcdefghabcdefgh");
        assert_eq!(&*d, "abcdefgh");
    }

    #[test]
    fn a_shared_buffer_frees_its_elements_once() {
        let x = Rc::new(5);
        let buf = Rc::new(Buf::from_vec(vec![x.clone()]));
        assert!(append(&buf, 0, 1, &[x.clone(), x.clone()], true).is_some());
        // The first copy is exact, the next ones leave room.
        let grown = append(&buf, 0, 1, std::slice::from_ref(&x), true).unwrap();
        let roomy = append(&grown, 0, 2, std::slice::from_ref(&x), true).unwrap();
        assert!(append(&roomy, 0, 3, std::slice::from_ref(&x), true).is_none());
        assert!(append(&roomy, 0, 3, std::slice::from_ref(&x), false).is_some());
        assert_eq!(roomy.used(), 4);
        drop((buf, grown, roomy));
        assert_eq!(Rc::strong_count(&x), 1);
    }
}
