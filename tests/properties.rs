//! Properties of the parts around the interpreter, checked on many random
//! inputs: versions and their ranges (`semver`), `rythme.toml` and the files
//! of the package index read from odd TOML, and the checksum.

use rythme::manifest;
use rythme::semver::{Range, Version};

struct Rng(u64);

impl Rng {
    /// splitmix64.
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[self.below(items.len())]
    }
}

/// A text made of pieces that matter to the reader under test.
fn soup(rng: &mut Rng, pieces: &[&str], max: usize) -> String {
    (0..rng.below(max + 1)).map(|_| rng.pick(pieces)).collect()
}

// ----- versions and ranges ----------------------------------------------------

const VERSION_PIECES: &[&str] = &[
    "0",
    "1",
    "2",
    "10",
    "01",
    ".",
    ".",
    "-",
    "+",
    "alpha",
    "beta",
    "rc",
    "x",
    "X",
    "*",
    "^",
    "~",
    ">",
    "<",
    "=",
    ">=",
    "<=",
    ",",
    " ",
    "é",
    "18446744073709551615",
    "18446744073709551616",
    "",
    "-0",
    "a-b",
];

/// A version: a release (`1.2.3`) or a pre-release (`1.2.3-beta.2`).
fn some_version(rng: &mut Rng) -> Version {
    let n = |rng: &mut Rng| rng.below(3).to_string();
    let mut s = format!("{}.{}.{}", n(rng), n(rng), n(rng));
    if rng.below(3) == 0 {
        s.push('-');
        s.push_str(rng.pick(&["alpha", "beta", "rc", "1", "alpha.1", "beta.2", "0"]));
    }
    Version::parse(&s).unwrap_or_else(|| panic!("{s}"))
}

/// Reading never panics; what is read is shown as it was written.
#[test]
fn versions_and_ranges_read_from_any_text() {
    let mut rng = Rng(1);
    for _ in 0..100_000 {
        let s = soup(&mut rng, VERSION_PIECES, 8);
        if let Some(v) = Version::parse(&s) {
            assert_eq!(v.to_string(), s);
            assert_eq!(Version::parse(&v.to_string()).as_ref(), Some(&v));
        }
        if let Some(r) = Range::parse(&s) {
            assert_eq!(r.to_string(), s.trim());
            let _ = r.matches(&some_version(&mut rng));
        }
    }
}

/// The order of versions is a total order, and the ranges mean what they say.
#[test]
fn ranges_follow_the_order_of_versions() {
    let mut rng = Rng(2);
    for _ in 0..50_000 {
        let (a, b, c) = (some_version(&mut rng), some_version(&mut rng), some_version(&mut rng));
        // A total order: antisymmetric, transitive.
        assert_eq!(a.cmp(&b), b.cmp(&a).reverse(), "{a} {b}");
        if a <= b && b <= c {
            assert!(a <= c, "{a} {b} {c}");
        }
        let range = |op: &str| Range::parse(&format!("{op}{a}")).unwrap_or_else(|| panic!("{op}{a}"));
        // A pre-release is only taken by a range that names a pre-release of the same version.
        let taken = !b.is_prerelease() || (a.is_prerelease() && (a.major, a.minor, a.patch) == (b.major, b.minor, b.patch));
        assert_eq!(range("=").matches(&b), a == b, "={a} on {b}");
        assert_eq!(range(">=").matches(&b), b >= a && taken, ">={a} on {b}");
        assert_eq!(range(">").matches(&b), b > a && taken, ">{a} on {b}");
        assert_eq!(range("<=").matches(&b), b <= a && taken, "<={a} on {b}");
        assert_eq!(range("<").matches(&b), b < a && taken, "<{a} on {b}");
        // `^a`: from a, without the next number that may break (major, or the first one that is not 0).
        let compatible = if a.major > 0 {
            b.major == a.major
        } else if a.minor > 0 {
            b.major == 0 && b.minor == a.minor
        } else {
            (b.major, b.minor, b.patch) == (0, 0, a.patch)
        };
        assert_eq!(range("^").matches(&b), b >= a && compatible && taken, "^{a} on {b}");
        assert_eq!(range("").matches(&b), range("^").matches(&b), "{a} on {b}");
        // `~a`: from a, the same major and minor.
        assert_eq!(range("~").matches(&b), b >= a && b.major == a.major && b.minor == a.minor && taken, "~{a} on {b}");
        // Two conditions: both.
        let both = Range::parse(&format!(">={a}, <{c}")).unwrap();
        if !b.is_prerelease() {
            assert_eq!(both.matches(&b), b >= a && b < c, ">={a}, <{c} on {b}");
        }
    }
}

#[test]
fn version_edge_cases() {
    for bad in [
        "",
        "1",
        "1.2",
        "1.2.3.4",
        "01.2.3",
        "1.2.3-",
        "1.2.3+",
        "1.2.3-é",
        "1..3",
        "-1.2.3",
        " 1.2.3",
        "1.2.3-01",
        "18446744073709551616.0.0",
    ] {
        assert!(Version::parse(bad).is_none(), "{bad}");
    }
    for bad in
        ["", " ", ",", "1.2.3,", "^", ">=", "1.2.3-beta+b", "1.x.3", "*.1", "x", ">=1.*", "1.2-beta", "01", "^1.2.3.4", "1.0.0 <2.0.0"]
    {
        assert!(Range::parse(bad).is_none(), "{bad}");
    }
    let v = |s: &str| Version::parse(s).unwrap();
    let r = |s: &str| Range::parse(s).unwrap();
    // 0.x: the minor number may break, 0.0.x: every version may.
    assert!(r("^0.2.3").matches(&v("0.2.9")) && !r("^0.2.3").matches(&v("0.3.0")));
    assert!(r("^0.0.3").matches(&v("0.0.3")) && !r("^0.0.3").matches(&v("0.0.4")));
    assert!(r("^0").matches(&v("0.9.0")) && !r("^0").matches(&v("1.0.0")));
    assert!(r("^0.0").matches(&v("0.0.7")) && !r("^0.0").matches(&v("0.1.0")));
    // Pre-releases: only when asked, and only of that version.
    assert!(!r("^1.0.0").matches(&v("1.1.0-beta")));
    assert!(r("^1.0.0-beta").matches(&v("1.0.0-beta.2")) && !r("^1.0.0-beta").matches(&v("1.0.1-beta")));
    assert!(r("^1.0.0-beta").matches(&v("1.0.0")) && r("^1.0.0-beta").matches(&v("1.5.0")));
    assert!(v("1.0.0-alpha") < v("1.0.0-alpha.1") && v("1.0.0-alpha.1") < v("1.0.0-alpha.beta") && v("1.0.0-rc.1") < v("1.0.0"));
    assert!(v("1.0.0-2") < v("1.0.0-10") && v("1.0.0-10") < v("1.0.0-a"));
    // Wildcards.
    assert!(r("1.*").matches(&v("1.9.9")) && !r("1.*").matches(&v("2.0.0")) && r("*").matches(&v("7.0.0")));
    assert!(r("1.2.x").matches(&v("1.2.9")) && !r("1.2.x").matches(&v("1.3.0")));
}

// ----- rythme.toml, rythme.lock and the files of the index ---------------------------------

const TOML_PIECES: &[&str] = &[
    "name",
    "version",
    "entry",
    "description",
    "[dependencies]",
    "[package]",
    "[[package]]",
    "[[version]]",
    "[",
    "]",
    "[[",
    "]]",
    "=",
    " = ",
    "\"",
    "\"\"",
    "\"0.1.0\"",
    "\"^1.2\"",
    "\"src/main.ry\"",
    "{",
    "}",
    "{ git = \"https://x/y\", tag = \"v1\" }",
    "{ path = \"../a\" }",
    "{ version = \"1\" }",
    "[\"a\", \"b\"]",
    ",",
    "#",
    "# comment",
    "\n",
    "\r\n",
    "\r",
    "\t",
    " ",
    "\\",
    "\\\"",
    "\\u{1F600}",
    "\\u00e9",
    "é",
    "👍",
    "日本",
    "'",
    "'''",
    "\"\"\"",
    "1",
    "true",
    "-",
    "--upload-pack=x",
    "../../etc",
    "a.b",
    "\u{0}",
    "\u{feff}",
    "mathlib",
    "tools",
    "commit = \"abcd\"",
    "tag = \"-x\"",
    "git = \"-x\"",
    "keywords",
    "authors",
];

/// `rythme.toml` and `rythme.lock`: reading never panics, a message points
/// inside the file, and a text written with `quote` reads back the same.
#[test]
fn odd_toml_never_breaks_the_reader() {
    let mut rng = Rng(3);
    let dir = std::env::temp_dir().join(format!("rythme-properties-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..20_000 {
        let text = soup(&mut rng, TOML_PIECES, 30);
        let file = rythme::source::add("rythme.toml", &text);
        if let Err(d) = manifest::parse(&text, file) {
            if let Some(sp) = d.span {
                let (s, e) = (sp.start as usize, sp.end as usize);
                assert!(s <= e && e <= text.len() && text.is_char_boundary(s) && text.is_char_boundary(e), "{text:?}: {:?}", d.span);
            }
            let _ = d.render(false);
        }
        // The whole project and its lock file, as `rythme install` reads them.
        if i % 10 == 0 {
            std::fs::write(dir.join("rythme.toml"), &text).unwrap();
            std::fs::write(dir.join("rythme.lock"), soup(&mut rng, TOML_PIECES, 30)).unwrap();
            let _ = manifest::load(&dir).map_err(|d| d.render(false));
            let _ = manifest::load_lock(&dir).map_err(|d| d.render(false));
        }
        let quoted = manifest::quote(&soup(&mut rng, TOML_PIECES, 6).replace(['\n', '\r'], ""));
        let line = format!("name = {quoted}\n");
        let doc = manifest::parse(&line, rythme::source::add("q.toml", &line));
        match doc.ok().and_then(|d| d.top.into_iter().next()) {
            Some(manifest::Entry { value: manifest::Value::Text(t), .. }) => assert_eq!(manifest::quote(&t), quoted),
            _ => panic!("`{line}` does not read back"),
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The file of a package in the index: never a panic, whatever is written.
#[test]
fn odd_entries_of_the_index_never_break_the_reader() {
    let mut rng = Rng(4);
    let dir = std::env::temp_dir().join(format!("rythme-properties-entry-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("tools.toml");
    let pieces: Vec<&str> = TOML_PIECES
        .iter()
        .copied()
        .chain([
            "repository = \"https://github.com/a/tools\"",
            "license = \"MIT\"",
            "commit = \"0123456789abcdef0123456789abcdef01234567\"",
            "checksum = \"sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\"",
            "yanked = \"bug\"",
            "version = \"1.0.0\"",
            "version = \"1.0.0-beta\"",
            "version = \"x\"",
        ])
        .collect();
    for i in 0..5_000 {
        let text = soup(&mut rng, &pieces, 40);
        std::fs::write(&path, &text).unwrap();
        for strict in [false, true] {
            if let Err(d) = rythme::index::read_entry(&path, "tools", strict) {
                assert!(!d.message.is_empty(), "case {i}: {text:?}");
                let _ = d.render(false);
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}
