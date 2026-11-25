//! Versions (`1.2.3`, `1.0.0-beta.2`) and the ranges of versions a
//! dependency accepts (`^1.2.0`, `~1.2`, `>=1.0.0, <2.0.0`), as in
//! semantic versioning (https://semver.org) and with the meanings Cargo and
//! npm give them: `^1.2.0` (or `1.2.0`) takes 1.2.0 and every newer version
//! that keeps compatibility, that is up to 2.0.0 excluded.

use std::cmp::Ordering;

/// One part of a pre-release (`beta`, `2`).
#[derive(Clone, Debug, PartialEq, Eq)]
enum Ident {
    Number(u64),
    Word(String),
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Ident::Number(a), Ident::Number(b)) => a.cmp(b),
            (Ident::Number(_), Ident::Word(_)) => Ordering::Less,
            (Ident::Word(_), Ident::Number(_)) => Ordering::Greater,
            (Ident::Word(a), Ident::Word(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A pre-release (`-beta.2`): none comes after every pre-release (1.0.0 > 1.0.0-beta).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Pre(Vec<Ident>);

impl Ord for Pre {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_empty(), other.0.is_empty()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self.0.cmp(&other.0),
        }
    }
}

impl PartialOrd for Pre {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn number(s: &str) -> Option<u64> {
    // No leading zero (`01`), as semantic versioning asks.
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0')) {
        return None;
    }
    s.parse().ok()
}

fn pre(s: &str) -> Option<Pre> {
    let mut out = Vec::new();
    for part in s.split('.') {
        if part.is_empty() || !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return None;
        }
        out.push(match part.chars().all(|c| c.is_ascii_digit()) {
            true => Ident::Number(number(part)?),
            false => Ident::Word(part.to_string()),
        });
    }
    Some(Pre(out))
}

/// A version: three numbers, major.minor.patch, and maybe a pre-release
/// and build information (ignored when versions are compared).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pre: Pre,
    /// As written.
    text: String,
}

impl Version {
    pub fn parse(s: &str) -> Option<Version> {
        let (rest, build) = match s.split_once('+') {
            Some((r, b)) => (r, Some(b)),
            None => (s, None),
        };
        if let Some(b) = build {
            if b.is_empty() || !b.split('.').all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')) {
                return None;
            }
        }
        let (core, pre_text) = match rest.split_once('-') {
            Some((c, p)) => (c, Some(p)),
            None => (rest, None),
        };
        let mut parts = core.split('.');
        let (major, minor, patch) = (number(parts.next()?)?, number(parts.next()?)?, number(parts.next()?)?);
        if parts.next().is_some() {
            return None;
        }
        let pre = match pre_text {
            Some(p) => pre(p)?,
            None => Pre::default(),
        };
        Some(Version { major, minor, patch, pre, text: s.to_string() })
    }

    pub fn is_prerelease(&self) -> bool {
        !self.pre.0.is_empty()
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)).then_with(|| self.pre.cmp(&other.pre))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Caret,
    Tilde,
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
}

/// One condition of a range: `^1.2`, `>=1.0.0`, `1.*`...
#[derive(Clone, Debug, PartialEq, Eq)]
struct Comparator {
    op: Op,
    major: u64,
    minor: Option<u64>,
    patch: Option<u64>,
    pre: Pre,
}

/// The versions a dependency accepts: conditions separated by commas, all
/// of which must hold. `*` accepts every version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Range {
    comparators: Vec<Comparator>,
    text: String,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

fn comparator(s: &str) -> Option<Comparator> {
    let s = s.trim();
    let (op, rest) = [
        (">=", Op::GreaterEq),
        ("<=", Op::LessEq),
        (">", Op::Greater),
        ("<", Op::Less),
        ("=", Op::Exact),
        ("^", Op::Caret),
        ("~", Op::Tilde),
    ]
    .iter()
    .find_map(|(p, op)| s.strip_prefix(p).map(|r| (*op, r.trim_start())))
    .unwrap_or((Op::Caret, s));
    let wild = |p: &str| matches!(p, "*" | "x" | "X");
    let (rest, pre_text) = match rest.split_once('-') {
        Some((r, p)) => (r, Some(p)),
        None => (rest, None),
    };
    let parts: Vec<&str> = rest.split('.').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let major = number(parts[0])?;
    let mut op = op;
    let mut field = |i: usize| -> Option<Option<u64>> {
        match parts.get(i) {
            None => Some(None),
            Some(p) if wild(p) => {
                // `1.*` is `1`; a wildcard needs no operator, or `=`.
                if !matches!(op, Op::Caret | Op::Exact) || parts[i + 1..].iter().any(|q| !wild(q)) {
                    return None;
                }
                op = Op::Exact;
                Some(None)
            }
            Some(p) => Some(Some(number(p)?)),
        }
    };
    let minor = field(1)?;
    let patch = field(2)?;
    if minor.is_none() && patch.is_some() {
        return None;
    }
    let pre = match pre_text {
        Some(p) if patch.is_some() => pre(p)?,
        Some(_) => return None,
        None => Pre::default(),
    };
    Some(Comparator { op, major, minor, patch, pre })
}

impl Range {
    /// Reads a range; `None` when it is not one.
    pub fn parse(s: &str) -> Option<Range> {
        let text = s.trim();
        if text.is_empty() {
            return None;
        }
        if text == "*" {
            return Some(Range { comparators: Vec::new(), text: text.to_string() });
        }
        let comparators = text.split(',').map(comparator).collect::<Option<Vec<_>>>()?;
        Some(Range { comparators, text: text.to_string() })
    }

    /// Does this version belong to the range? A pre-release (`1.2.0-beta`)
    /// belongs to it only when the range names a pre-release of the same
    /// three numbers: `^1.0.0` never takes a beta by surprise.
    pub fn matches(&self, v: &Version) -> bool {
        if !self.comparators.iter().all(|c| c.matches(v)) {
            return false;
        }
        !v.is_prerelease()
            || self
                .comparators
                .iter()
                .any(|c| c.major == v.major && c.minor == Some(v.minor) && c.patch == Some(v.patch) && !c.pre.0.is_empty())
    }
}

impl Comparator {
    fn matches(&self, v: &Version) -> bool {
        match self.op {
            Op::Exact => self.exact(v),
            Op::Greater => self.greater(v),
            Op::GreaterEq => self.exact(v) || self.greater(v),
            Op::Less => self.less(v),
            Op::LessEq => self.exact(v) || self.less(v),
            Op::Tilde => self.tilde(v),
            Op::Caret => self.caret(v),
        }
    }

    fn exact(&self, v: &Version) -> bool {
        v.major == self.major && self.minor.is_none_or(|m| v.minor == m) && self.patch.is_none_or(|p| v.patch == p) && v.pre == self.pre
    }

    fn greater(&self, v: &Version) -> bool {
        if v.major != self.major {
            return v.major > self.major;
        }
        let Some(minor) = self.minor else { return false };
        if v.minor != minor {
            return v.minor > minor;
        }
        let Some(patch) = self.patch else { return false };
        if v.patch != patch {
            return v.patch > patch;
        }
        v.pre > self.pre
    }

    fn less(&self, v: &Version) -> bool {
        if v.major != self.major {
            return v.major < self.major;
        }
        let Some(minor) = self.minor else { return false };
        if v.minor != minor {
            return v.minor < minor;
        }
        let Some(patch) = self.patch else { return false };
        if v.patch != patch {
            return v.patch < patch;
        }
        v.pre < self.pre
    }

    fn tilde(&self, v: &Version) -> bool {
        if v.major != self.major || self.minor.is_some_and(|m| v.minor != m) {
            return false;
        }
        if let Some(patch) = self.patch {
            if v.patch != patch {
                return v.patch > patch;
            }
        }
        v.pre >= self.pre
    }

    fn caret(&self, v: &Version) -> bool {
        if v.major != self.major {
            return false;
        }
        let Some(minor) = self.minor else { return true };
        let Some(patch) = self.patch else {
            return if self.major > 0 { v.minor >= minor } else { v.minor == minor };
        };
        if self.major > 0 {
            if v.minor != minor {
                return v.minor > minor;
            }
            if v.patch != patch {
                return v.patch > patch;
            }
        } else if minor > 0 {
            if v.minor != minor {
                return false;
            }
            if v.patch != patch {
                return v.patch > patch;
            }
        } else if v.minor != minor || v.patch != patch {
            return false;
        }
        v.pre >= self.pre
    }
}

#[cfg(test)]
mod tests {
    use super::{Range, Version};

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap_or_else(|| panic!("{s}"))
    }

    #[track_caller]
    fn check(range: &str, yes: &[&str], no: &[&str]) {
        let r = Range::parse(range).unwrap_or_else(|| panic!("{range}"));
        for y in yes {
            assert!(r.matches(&v(y)), "{range} should take {y}");
        }
        for n in no {
            assert!(!r.matches(&v(n)), "{range} should not take {n}");
        }
    }

    #[test]
    fn versions_are_read_and_ordered() {
        assert!(v("1.0.0") > v("1.0.0-rc.1"));
        assert!(v("1.0.0-rc.1") > v("1.0.0-beta.11"));
        assert!(v("1.0.0-beta.11") > v("1.0.0-beta.2"));
        assert!(v("1.0.0-beta") > v("1.0.0-alpha.1"));
        assert!(v("1.0.0-alpha.1") > v("1.0.0-alpha"));
        assert!(v("2.0.0") > v("1.10.0") && v("1.10.0") > v("1.9.9"));
        assert_eq!(v("1.0.0+build.5").cmp(&v("1.0.0")), std::cmp::Ordering::Equal);
        for bad in ["1.0", "1", "01.0.0", "1.0.0-", "1.0.0+", "a.b.c", "1.0.0.0", "1.0.0-be ta", "", "-1.0.0"] {
            assert!(Version::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn ranges_mean_what_cargo_and_npm_say() {
        check("^1.2.3", &["1.2.3", "1.2.9", "1.9.0"], &["1.2.2", "2.0.0", "1.3.0-beta"]);
        check("1.2.3", &["1.2.3", "1.4.0"], &["2.0.0", "1.2.0"]);
        check("^1.2", &["1.2.0", "1.9.9"], &["1.1.9", "2.0.0"]);
        check("^1", &["1.0.0", "1.9.9"], &["0.9.0", "2.0.0"]);
        check("^0.2.3", &["0.2.3", "0.2.9"], &["0.3.0", "0.2.2"]);
        check("^0.0.3", &["0.0.3"], &["0.0.4", "0.0.2"]);
        check("^0.2", &["0.2.0", "0.2.7"], &["0.3.0"]);
        check("~1.2.3", &["1.2.3", "1.2.8"], &["1.3.0", "1.2.2"]);
        check("~1.2", &["1.2.0", "1.2.8"], &["1.3.0"]);
        check("~1", &["1.0.0", "1.8.0"], &["2.0.0"]);
        check("=1.2.3", &["1.2.3"], &["1.2.4"]);
        check(">=1.2.0, <1.5.0", &["1.2.0", "1.4.9"], &["1.5.0", "1.1.0"]);
        check(">1.2", &["1.3.0"], &["1.2.9"]);
        check("<=1.2", &["1.2.9", "0.1.0"], &["1.3.0"]);
        check("<1", &["0.9.9"], &["1.0.0"]);
        check("*", &["0.0.1", "9.9.9"], &["1.0.0-beta"]);
        check("1.*", &["1.0.0", "1.9.0"], &["2.0.0"]);
        check("1.2.x", &["1.2.0", "1.2.9"], &["1.3.0"]);
        check("^1.0.0-beta.2", &["1.0.0-beta.2", "1.0.0-beta.10", "1.0.0", "1.5.0"], &["1.0.0-beta.1", "1.1.0-beta.1"]);
        check("=1.0.0-rc.1", &["1.0.0-rc.1"], &["1.0.0"]);
        for bad in ["", "abc", "^", "1.2.3.4", ">=", "1.*.3", "~1.*", "1.2-beta", "^01.0", ",", "1.0, "] {
            assert!(Range::parse(bad).is_none(), "{bad:?}");
        }
    }
}
