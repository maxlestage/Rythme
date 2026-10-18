//! CSV (`csv_parse`, `csv_records`, `csv_format`) and the seed of `random`
//! (`random_seed`, `rythme run --seed N`).
//!
//! The reader is checked on the cases of RFC 4180, on its mistakes (each one
//! with its line and column), and on thousands of random tables: writing a
//! table then reading it gives the same table back, and no text, however
//! broken, makes it crash.

mod common;

use common::{fails, ok, rythme, rythme_lang};
use rythme::builtins::csv::{format, parse, parse_records, CsvErrorKind};

// ----- reading -----------------------------------------------------------------

#[test]
fn reading_tables() {
    ok("print(csv_parse(\"name,age\\nAda,36\\nAlan,41\"))", "Ok([[\"name\", \"age\"], [\"Ada\", \"36\"], [\"Alan\", \"41\"]])");
    // Quotes: separators, line breaks and doubled quotes inside; CRLF; a final line break.
    ok(
        "print(csv_parse(\"a,\\\"b, c\\\",\\\"say \\\"\\\"hi\\\"\\\"\\\"\\r\\n\\\"two\\r\\nlines\\\",x,\\r\\n\"))",
        "Ok([[\"a\", \"b, c\", \"say \\\"hi\\\"\"], [\"two\\r\\nlines\", \"x\", \"\"]])",
    );
    // Empty texts and empty lines give no rows; a line with "" is a row of one empty field.
    ok("print(csv_parse(\"\"), csv_parse(\"\\n\\n\"), csv_parse(\"a\\n\\n\\\"\\\"\\nb\"))", "Ok([]) Ok([]) Ok([[\"a\"], [\"\"], [\"b\"]])");
    // Spaces are kept; an empty field at the end of a line is a field.
    ok("print(csv_parse(\" a , b ,\"))", "Ok([[\" a \", \" b \", \"\"]])");
    // Another separator; the comma is then an ordinary character.
    ok("print(csv_parse(\"nom;prix\\npain;1,20\", \";\"))", "Ok([[\"nom\", \"prix\"], [\"pain\", \"1,20\"]])");
    ok("print(csv_parse(\"a\\tb\", \"\\t\"), csv_parse(\"a|b\", \"|\"))", "Ok([[\"a\", \"b\"]]) Ok([[\"a\", \"b\"]])");
    // The invisible mark that some programs put at the start of a file.
    ok("print(csv_parse(\"\\u{feff}a,b\"))", "Ok([[\"a\", \"b\"]])");
    // Rows may have different lengths: `csv_parse` keeps them as they are.
    ok("print(csv_parse(\"a,b\\nc\"))", "Ok([[\"a\", \"b\"], [\"c\"]])");
    // Any character, accents and emoji included.
    ok("print(csv_parse(\"é,日本,🎵\"))", "Ok([[\"é\", \"日本\", \"🎵\"]])");
}

#[test]
fn mistakes_say_where_they_are() {
    ok(
        "print(csv_parse(\"a,b\\nc,d\\\"e\"))",
        "Err(\"line 2, column 4: a quote in the middle of a field: put the whole field between quotes, and write each quote inside it twice (\\\"5\\\"\\\" screen\\\")\")",
    );
    ok(
        "print(csv_parse(\"a,\\\"b\\\"c\"))",
        "Err(\"line 1, column 6: after the quote that closes a field comes `,` or the end of the line, not `c`\")",
    );
    ok(
        "print(csv_parse(\"a\\n\\\"b\\nc\"))",
        "Err(\"line 2, column 1: the quote that starts this field is never closed (a quote inside a field is written twice: \\\"\\\")\")",
    );
    // Columns count characters, not bytes; a space after a quote is shown by its name.
    ok(
        "print(csv_parse(\"é,\\\"x\\\" \"))",
        "Err(\"line 1, column 6: after the quote that closes a field comes `,` or the end of the line, not a space\")",
    );
    // The line of a row that starts after a field on several lines.
    ok("print(csv_parse(\"\\\"a\\nb\\nc\\\"\\nd\\\"\"))", "Err(\"line 4, column 2: a quote in the middle of a field: put the whole field between quotes, and write each quote inside it twice (\\\"5\\\"\\\" screen\\\")\")");
    // In the language of the messages.
    let r = rythme_lang("fr", &["-e", "print(csv_parse(\"a\\\"\"))"]);
    assert!(r.stdout.starts_with("Err(\"ligne 1, colonne 2 : un guillemet au milieu d'un champ"), "{}", r.stdout);
    let r = rythme_lang("es", &["-e", "print(csv_records(\"a,a\"))"]);
    assert_eq!(r.stdout.trim(), "Err(\"línea 1: el encabezado nombra dos veces la colonne \\\"a\\\"\")".replace("colonne", "columna"));
    // A separator that cannot be one stops the program.
    let r = fails("csv_parse(\"a\", \"ab\")", "E023");
    assert!(r.stderr.contains("the separator of `csv_parse` is one character, not \"ab\""), "{}", r.stderr);
    assert!(r.stderr.contains("\";\" (French spreadsheets)"), "{}", r.stderr);
    for sep in ["\\\"", "\\n", "\\r", ""] {
        fails(&format!("csv_parse(\"a\", \"{sep}\")"), "E023");
    }
    fails("csv_parse(1)", "E005");
    fails("csv_records(\"a\", 1)", "E005");
}

#[test]
fn records_from_a_header() {
    ok(
        "rows = unwrap(csv_records(\"name,first name\\nLovelace,Ada\\nTuring,Alan\\n\"))\nprint(rows)\nprint(rows[0].name, get(rows[1], \"first name\"), keys(rows[0]))",
        "[{name: \"Lovelace\", \"first name\": \"Ada\"}, {name: \"Turing\", \"first name\": \"Alan\"}]\nLovelace Some(\"Alan\") [\"name\", \"first name\"]",
    );
    // Only a header, or nothing: no records.
    ok("print(csv_records(\"a,b\"), csv_records(\"\"))", "Ok([]) Ok([])");
    ok("print(csv_records(\"x;y\\n1;2\", \";\"))", "Ok([{x: \"1\", y: \"2\"}])");
    // The fields are texts: numbers come from parse_int and parse_float.
    ok(
        "total = unwrap(csv_records(\"item,price\\nbread,1.5\\nmilk,2\")).map { |r| unwrap(parse_float(r.price)) }.sum\nprint(total)",
        "3.5",
    );
    ok("print(csv_records(\"a,b\\n1,2\\n3\\n\"))", "Err(\"line 3: 1 field(s), but the header has 2 column(s)\")");
    ok("print(csv_records(\"a,b\\n1,2,3\"))", "Err(\"line 2: 3 field(s), but the header has 2 column(s)\")");
    ok("print(csv_records(\"a,b,a\\n1,2,3\"))", "Err(\"line 1: the header names the column \\\"a\\\" twice\")");
    ok(
        "print(csv_records(\"a,\\\"b\"))",
        "Err(\"line 1, column 3: the quote that starts this field is never closed (a quote inside a field is written twice: \\\"\\\")\")",
    );
}

// ----- writing -----------------------------------------------------------------

#[test]
fn writing_tables() {
    ok("print(csv_format([[\"a\", \"b\"], [\"c\", \"d\"]]) == \"a,b\\nc,d\\n\")", "true");
    // Quotes only where they are needed.
    ok(
        "print([csv_format([[\"x, y\", \"say \\\"hi\\\"\", \"two\\nlines\", \"plain\", \" spaced \"]])])",
        "[\"\\\"x, y\\\",\\\"say \\\"\\\"hi\\\"\\\"\\\",\\\"two\\nlines\\\",plain, spaced \\n\"]",
    );
    // Numbers and Bools are written as `print` writes them.
    ok("print([csv_format([[1, -2.5, true, 2 ** 70]])])", "[\"1,-2.5,true,1180591620717411303424\\n\"]");
    // A lonely empty field is written "" so that its line is not an empty line.
    ok("print([csv_format([[\"\"], [\"\", \"\"], []])])", "[\"\\\"\\\"\\n,\\n\\n\"]");
    ok("print([csv_format([])])", "[\"\"]");
    // Another separator: the comma no longer needs quotes, the separator does.
    ok("print([csv_format([[\"1,5\", \"a;b\"]], \";\")])", "[\"1,5;\\\"a;b\\\"\\n\"]");
    // Records: a header with the fields of the first one, in its order.
    ok("print([csv_format([{name: \"Ada\", age: 36}, {age: 41, name: \"Alan\"}])])", "[\"name,age\\nAda,36\\nAlan,41\\n\"]");
    let r = fails("csv_format([{a: 1}, {a: 2, b: 3}])", "E023");
    assert!(r.stderr.contains("record 2 has the fields {a, b}, but the first one has {a}"), "{}", r.stderr);
    let r = fails("csv_format([[\"a\", None]])", "E005");
    assert!(r.stderr.contains("the field 2 of row 1 must be a text, a number or a Bool, but it is Option None"), "{}", r.stderr);
    let r = fails("csv_format([[\"a\"], \"b\"])", "E005");
    assert!(r.stderr.contains("row 2 must be a list of fields"), "{}", r.stderr);
    fails("csv_format([{a: 1}, [1]])", "E005");
    fails("csv_format(\"a,b\")", "E005");
    fails("csv_format([[\"a\"]], \"\\\"\")", "E023");
}

#[test]
fn reading_what_was_written() {
    ok(
        "rows = [[\"a\", \"b, c\"], [\"say \\\"hi\\\"\", \"two\\r\\nlines\"], [\"\"], [\"é\", \"\"]]\nprint(csv_parse(csv_format(rows)) == Ok(rows), csv_parse(csv_format(rows, \";\"), \";\") == Ok(rows))",
        "true true",
    );
    ok(
        "rows = [{name: \"Ada\", note: \"a, \\\"b\\\"\"}, {name: \"Alan\", note: \"\"}]\nprint(csv_records(csv_format(rows)) == Ok(rows))",
        "true",
    );
}

#[test]
fn with_files() {
    let dir = std::env::temp_dir().join(format!("rythme-csv-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("grades.csv");
    // A file from a French spreadsheet: semicolons, CRLF, decimal commas.
    std::fs::write(&file, "élève;note\r\nAda;18,5\r\n\"Turing; Alan\";17\r\n").unwrap();
    let path = file.to_string_lossy().replace('\\', "/");
    let out = dir.join("out.csv").to_string_lossy().replace('\\', "/");
    let program = format!(
        "fn table(t) = csv_records(t, \";\")\n\nrows = unwrap(read_file(\"{path}\").and_then(table))\nprint(rows.map {{ |r| r.élève }})\nprint(write_file(\"{out}\", csv_format(rows)))\nprint(read_file(\"{out}\"))"
    );
    let file_ry = dir.join("grades.ry");
    std::fs::write(&file_ry, program).unwrap();
    let r = rythme(&[&file_ry.to_string_lossy()]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(r.stdout, "[\"Ada\", \"Turing; Alan\"]\nOk(())\nOk(\"élève,note\\nAda,\\\"18,5\\\"\\nTuring; Alan,17\\n\")\n");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_checker_knows_csv() {
    let dir = std::env::temp_dir().join(format!("rythme-csv-check-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for (code, error) in [
        ("csv_parse(42)", Some("E005")),
        ("csv_records(\"a\", 1)", Some("E005")),
        ("csv_format(\"a,b\")", Some("E005")),
        ("random_seed(\"x\")", Some("E005")),
        ("random_seed(1.5)", Some("E005")),
        // Correct uses say nothing: the fields of records are not known in advance.
        ("unwrap(csv_records(\"a\\n1\"))[0].anything", None),
        ("len(unwrap(csv_parse(\"a\")))", None),
        ("csv_format(unwrap(csv_records(\"a\\n1\")), \";\")", None),
        ("random_seed(-3)", None),
    ] {
        let file = dir.join("check.ry");
        std::fs::write(&file, format!("fn f() = {code}\n")).unwrap();
        let r = rythme(&["check", &file.to_string_lossy()]);
        match error {
            Some(e) => assert!(r.stderr.contains(&format!("error[{e}]")), "{code}: {}{}", r.stdout, r.stderr),
            None => assert_eq!(r.code, 0, "{code}: {}{}", r.stdout, r.stderr),
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_definition_of_the_program_keeps_its_name() {
    // The stability promise: a program that defined `csv_parse` or
    // `random_seed` before they existed keeps its own (W003 says so).
    ok("fn csv_parse(t) = \"mine\"\nrandom_seed = 3\nprint(csv_parse(\"a\"), random_seed)", "mine 3");
    let dir = std::env::temp_dir().join(format!("rythme-csv-own-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("own.ry");
    std::fs::write(&file, "fn csv_format(rows) = len(rows)\nprint(csv_format([1]))\n").unwrap();
    let r = rythme(&["check", &file.to_string_lossy()]);
    assert!(r.stderr.contains("warning[W003]") && r.stderr.contains("`csv_format` hides the built-in function"), "{}", r.stderr);
    let _ = std::fs::remove_dir_all(&dir);
}

// ----- random tables -----------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Characters that matter to CSV, and a few others.
const PIECES: &[char] = &['a', 'b', ' ', ',', ';', '"', '\n', '\r', '\t', 'é', '日'];

fn random_text(rng: &mut Rng, max: usize) -> String {
    (0..rng.below(max + 1)).map(|_| PIECES[rng.below(PIECES.len())]).collect()
}

#[test]
fn writing_then_reading_gives_the_table_back() {
    let mut rng = Rng(0x5eed_c5f0_1234_9876);
    for case in 0..20_000 {
        let sep = [',', ';', '\t', '|'][rng.below(4)];
        // Rows of at least one field: an empty row is an empty line, which is skipped.
        let rows: Vec<Vec<String>> = (0..rng.below(5)).map(|_| (0..1 + rng.below(4)).map(|_| random_text(&mut rng, 6)).collect()).collect();
        let text = format(&rows, sep);
        let back = parse(&text, sep).unwrap_or_else(|e| panic!("case {case}: {rows:?} written as {text:?}: {e:?}"));
        let back: Vec<Vec<String>> = back.into_iter().map(|r| r.fields).collect();
        assert_eq!(back, rows, "case {case}: written as {text:?}");
    }
}

#[test]
fn no_text_makes_the_reader_crash() {
    let mut rng = Rng(0x0dd_ba11_cafe_f00d);
    for _ in 0..50_000 {
        let text = random_text(&mut rng, 12);
        let sep = [',', ';', ' ', 'a'][rng.below(4)];
        // Ok or Err, never a panic; an error is always at a place of the text.
        match parse(&text, sep) {
            Ok(rows) => assert!(rows.iter().all(|r| r.line >= 1 && !r.fields.is_empty())),
            Err(e) => {
                assert!(e.line >= 1 && e.line <= text.matches('\n').count() + text.matches('\r').count() + 1, "{text:?}: {e:?}");
                assert!(e.column >= 1, "{text:?}: {e:?}");
                let _ = e.message(sep);
            }
        }
        if let Err(e) = parse_records(&text, sep) {
            assert!(e.line >= 1, "{text:?}: {e:?}");
            if matches!(e.kind, CsvErrorKind::Fields(..) | CsvErrorKind::SameColumn(_)) {
                assert_eq!(e.column, 0);
            }
        }
    }
}

// ----- the seed of random ----------------------------------------------------------

#[test]
fn the_same_seed_gives_the_same_draws() {
    let draws = "random_seed(42)\nprint(random(1, 6), random(1, 6), random(1, 1000000), random([\"a\", \"b\", \"c\"]), shuffle(1..8), text(random(), 6))";
    // These values are those of this version: same seed, same version, same draws.
    ok(draws, "5 2 744560 b [7, 8, 1, 2, 5, 4, 6, 3] 0.500297");
    ok(draws, "5 2 744560 b [7, 8, 1, 2, 5, 4, 6, 3] 0.500297");
    // Seeding again starts the same draws again; other seeds give others.
    ok("random_seed(7)\na = shuffle(1..20)\nrandom_seed(7)\nprint(a == shuffle(1..20))", "true");
    ok("random_seed(1)\na = shuffle(1..20)\nrandom_seed(2)\nprint(a == shuffle(1..20))", "false");
    ok("random_seed(-9223372036854775808)\nrandom_seed(0)\nrandom_seed(9223372036854775807)\nprint(random(5, 5))", "5");
    let r = fails("random_seed(2 ** 70)", "E023");
    assert!(r.stderr.contains("too large for `random_seed`"), "{}", r.stderr);
    fails("random_seed(\"42\")", "E005");
}

#[test]
fn rythme_run_takes_a_seed() {
    let dir = std::env::temp_dir().join(format!("rythme-seed-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("dice.ry");
    std::fs::write(&file, "print(random(1, 1000000), shuffle([1, 2, 3, 4, 5]), args())\n").unwrap();
    let path = file.to_string_lossy().to_string();
    let first = rythme(&["run", "--seed", "42", &path]);
    assert_eq!(first.code, 0, "{}", first.stderr);
    // The same as random_seed(42) at the start of the program.
    let inside = rythme(&["-e", "random_seed(42)\nprint(random(1, 1000000), shuffle([1, 2, 3, 4, 5]), args())"]);
    assert_eq!(first.stdout, inside.stdout);
    assert_eq!(rythme(&["run", "--seed=42", &path]).stdout, first.stdout);
    assert_ne!(rythme(&["run", "--seed", "43", &path]).stdout, first.stdout);
    // What follows the file is for the program, even `--seed`.
    let r = rythme(&["run", "--seed", "42", &path, "--seed", "1"]);
    assert!(r.stdout.ends_with("[\"--seed\", \"1\"]\n"), "{}", r.stdout);
    let r = rythme(&["run", "--seed", "lots", &path]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("the seed must be a whole number"), "{}", r.stderr);
    let r = rythme(&["run", "--seed"]);
    assert_eq!(r.code, 2);
    assert!(r.stderr.contains("`--seed` needs a whole number"), "{}", r.stderr);
    let r = rythme_lang("fr", &["run", "--seed", "x", &path]);
    assert!(r.stderr.contains("la graine doit être un nombre entier"), "{}", r.stderr);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn help_on_csv_and_random() {
    let r = rythme(&["help", "csv"]);
    assert!(r.stdout.starts_with("CSV: THE TABLES OF SPREADSHEETS"), "{}", r.stdout);
    let r = rythme_lang("fr", &["help", "tableur"]);
    assert!(r.stdout.starts_with("CSV : LES TABLEAUX DES TABLEURS"), "{}", r.stdout);
    // `random` is a topic and a function: both are shown.
    let r = rythme(&["help", "random"]);
    assert!(r.stdout.starts_with("RANDOM: CHANCE") && r.stdout.contains("random(a, b), random(list)"), "{}", r.stdout);
    let r = rythme(&["help", "seed"]);
    assert!(r.stdout.contains("rythme run --seed 42"), "{}", r.stdout);
    let r = rythme(&["help", "random_seed"]);
    assert!(r.stdout.contains("random_seed(n)"), "{}", r.stdout);
    let r = rythme(&["help"]);
    assert!(r.stdout.contains("  csv ") && r.stdout.contains("  random ") && r.stdout.contains("--seed"), "{}", r.stdout);
    for lang in ["es", "de", "pt", "zh"] {
        let r = rythme_lang(lang, &["help", "csv"]);
        assert!(r.stdout.starts_with("CSV"), "{lang}: {}", r.stdout);
        assert!(!r.stdout.contains("THE TABLES"), "{lang}: not translated");
    }
}

#[test]
fn names_from_other_languages() {
    for (name, rythme_name) in [
        ("parse_csv", "csv_parse"),
        ("read_csv", "csv_parse"),
        ("DictReader", "csv_records"),
        ("to_csv", "csv_format"),
        ("write_csv", "csv_format"),
        ("srand", "random_seed"),
        ("set_seed", "random_seed"),
    ] {
        let r = fails(&format!("{name}(1)"), "E003");
        assert!(r.stderr.contains(&format!("called `{rythme_name}`")), "{name}: {}", r.stderr);
    }
}
