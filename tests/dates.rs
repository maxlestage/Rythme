//! Dates and times: the calendar is right (leap years, month lengths), ISO
//! 8601 texts are read and written back, wrong dates are explained, and the
//! clock gives the local time with its offset.

mod common;

use common::{fails, ok, rythme, rythme_lang};

#[test]
fn the_calendar_knows_leap_years_and_month_lengths() {
    ok(
        "print(is_leap_year(2024), is_leap_year(2023), is_leap_year(1900), is_leap_year(2000), is_leap_year(2100), is_leap_year(-4))",
        "true false false true false true",
    );
    ok("print(map(1..12, fn(m) = days_in_month(2023, m)))", "[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]");
    ok("print(days_in_month(2024, 2), days_in_month(1900, 2), days_in_month(2000, 2))", "29 28 29");
    fails("days_in_month(2024, 13)", "E023");
    fails("days_in_month(2024, 0)", "E023");
    ok("print(sum(map(1..12, fn(m) = days_in_month(2024, m))), sum(map(1..12, fn(m) = days_in_month(2023, m))))", "366 365");
}

#[test]
fn adding_days_and_seconds_crosses_months_and_years() {
    ok(
        "print(add_days({year: 2024, month: 2, day: 28}, 1), add_days({year: 2023, month: 2, day: 28}, 1))",
        "{year: 2024, month: 2, day: 29} {year: 2023, month: 3, day: 1}",
    );
    ok(
        "print(add_days({year: 2024, month: 1, day: 1}, -1), add_days({year: 2024, month: 3, day: 1}, -1))",
        "{year: 2023, month: 12, day: 31} {year: 2024, month: 2, day: 29}",
    );
    ok(
        "print(add_days({year: 2000, month: 1, day: 1}, 146097), add_days({year: 2024, month: 5, day: 4}, 0))",
        "{year: 2400, month: 1, day: 1} {year: 2024, month: 5, day: 4}",
    );
    // The time of day and the offset stay.
    ok(
        "print(add_days({year: 2024, month: 10, day: 27, hour: 2, minute: 30, second: 0, offset: 120}, 1))",
        "{year: 2024, month: 10, day: 28, hour: 2, minute: 30, second: 0, offset: 120}",
    );
    ok(
        "print(add_seconds({year: 2024, month: 1, day: 1}, -1), add_seconds({year: 2024, month: 2, day: 28, hour: 23}, 3600))",
        "{year: 2023, month: 12, day: 31, hour: 23, minute: 59, second: 59} {year: 2024, month: 2, day: 29, hour: 0, minute: 0, second: 0}",
    );
    ok("print(add_days({year: 1, month: 1, day: 1}, 3652058))", "{year: 9999, month: 12, day: 31}");
    fails("add_days({year: 1, month: 1, day: 1}, -1)", "E023");
    fails("add_days({year: 9999, month: 12, day: 31}, 1)", "E023");
    fails("add_days({year: 2024, month: 1, day: 1}, 9223372036854775807)", "E023");
    fails("add_seconds({year: 2024, month: 1, day: 1}, -9223372036854775807 - 1)", "E023");
    fails("add_days({year: 2024, month: 1, day: 1}, 1.5)", "E005");
}

#[test]
fn differences_and_days_of_the_week() {
    ok("print(days_between({year: 2024, month: 1, day: 1}, {year: 2025, month: 1, day: 1}), days_between({year: 2023, month: 1, day: 1}, {year: 2024, month: 1, day: 1}))", "366 365");
    ok("print(days_between({year: 2025, month: 1, day: 1}, {year: 2024, month: 1, day: 1}), days_between({year: 1, month: 1, day: 1}, {year: 9999, month: 12, day: 31}))", "-366 3652058");
    // Whole days, rounded toward zero.
    ok(
        "print(days_between({year: 2024, month: 1, day: 1, hour: 12}, {year: 2024, month: 1, day: 3}), days_between({year: 2024, month: 1, day: 3}, {year: 2024, month: 1, day: 1, hour: 12}))",
        "1 -1",
    );
    // Offsets count: 12:00 at UTC+1 is 11:00 UTC.
    ok(
        "print(seconds_between({year: 2024, month: 6, day: 1, hour: 12, offset: 60}, {year: 2024, month: 6, day: 1, hour: 12, offset: 0}))",
        "3600",
    );
    ok(
        "print(seconds_between({year: 1970, month: 1, day: 1}, {year: 2001, month: 9, day: 9, hour: 1, minute: 46, second: 40}))",
        "1000000000",
    );
    ok(
        "print(map([{year: 2024, month: 2, day: 26}, {year: 2024, month: 3, day: 3}, {year: 1970, month: 1, day: 1}, {year: 1, month: 1, day: 1}], day_of_week))",
        "[1, 7, 4, 1]",
    );
    ok("print(day_of_week({year: 2000, month: 2, day: 29}), day_of_week({year: 2024, month: 12, day: 25, hour: 23, offset: -600}))", "2 3");
}

#[test]
fn iso_8601_texts_are_read_and_written_back() {
    ok(
        "print(parse_date(\"2024-02-29\"), parse_date(\" 2024-02-29\\n\"))",
        "Ok({year: 2024, month: 2, day: 29}) Ok({year: 2024, month: 2, day: 29})",
    );
    ok("print(parse_date(\"2024-02-29T13:05\"))", "Ok({year: 2024, month: 2, day: 29, hour: 13, minute: 5, second: 0})");
    ok(
        "print(parse_date(\"2024-02-29t13:05:09.999Z\"), parse_date(\"2024-02-29 13:05:09-05:30\"))",
        "Ok({year: 2024, month: 2, day: 29, hour: 13, minute: 5, second: 9, offset: 0}) Ok({year: 2024, month: 2, day: 29, hour: 13, minute: 5, second: 9, offset: -330})",
    );
    ok("print(parse_date(\"2024-02-29T13:05+0100\") == parse_date(\"2024-02-29T13:05:00+01\"))", "true");
    for text in ["2024-02-29", "2024-02-29T00:00:00", "0001-01-01T23:59:59Z", "9999-12-31T12:00:00+14:00", "2024-07-14T08:30:15-09:30"] {
        ok(&format!("print(format_date(unwrap(parse_date(\"{text}\"))))"), text);
    }
    ok(
        "print(format_date({year: 2024, month: 2, day: 29, hour: 9}), format_date({year: 2024, month: 2, day: 29, offset: 0}), format_date({year: 33, month: 1, day: 2}))",
        "2024-02-29T09:00:00 2024-02-29T00:00:00Z 0033-01-02",
    );
    for text in [
        "",
        "2024",
        "2024-2-29",
        "24-02-29",
        "2024/02/29",
        "2024-02-29T",
        "2024-02-29T13",
        "2024-02-29T13:5",
        "2024-02-29Z",
        "2024-02-29T13:05:00.",
        "2024-02-29T13:05+1",
        "2024-02-29T13:05+01:",
        "2024-02-29T13:05+01:60",
        "2024-02-29T13:05:00 Z",
        "２０２４-02-29",
        "2024-02-29T13:05:00Zx",
    ] {
        ok(&format!("print(is_err(parse_date(\"{text}\")))"), "true");
    }
    ok(
        "print(parse_date(\"2023-02-29\"))",
        "Err(\"\\\"2023-02-29\\\" is not a valid date: the day must go from 1 to 28 in February 2023, not 29 (2023 is not a leap year)\")",
    );
    for (text, why) in [
        ("0000-01-01", "the year must go from 1 to 9999, not 0"),
        ("2024-00-10", "the month must go from 1 to 12, not 0"),
        ("2024-04-31", "from 1 to 30 in April 2024, not 31"),
        ("2024-01-01T24:00", "the hour must go from 0 to 23, not 24"),
        ("2024-01-01T23:60", "the minute must go from 0 to 59, not 60"),
        ("2024-01-01T23:59:60", "the second must go from 0 to 59, not 60"),
        ("2024-01-01T23:59+19:00", "-1080 to 1080 minutes"),
        ("next tuesday", "is not a date written as ISO 8601"),
    ] {
        let r = rythme(&["-e", &format!("print(parse_date(\"{text}\"))")]);
        assert!(r.stdout.contains(why), "{text}: {}", r.stdout);
    }
    let r = rythme_lang("fr", &["-e", "print(parse_date(\"2023-02-29\"))"]);
    assert!(
        r.stdout.contains("le jour doit aller de 1 à 28 en février 2023, pas 29 (2023 n'est pas une année bissextile)"),
        "{}",
        r.stdout
    );
}

#[test]
fn wrong_dates_are_explained() {
    ok(
        "print(is_date({year: 2024, month: 2, day: 29}), is_date({year: 2023, month: 2, day: 29}), is_date({year: 2024, month: 2}), is_date(\"2024-02-29\"), is_date({year: 2024, month: 2, day: 1, colour: 1}))",
        "true false false false false",
    );
    let r = fails("add_days({year: 2023, month: 2, day: 29}, 1)", "E023");
    assert!(r.stderr.contains("2023 is not a leap year") && r.stderr.contains("{year: 2024, month: 2, day: 29}"), "{}", r.stderr);
    let r = fails("day_of_week({year: 2024, month: 2, dya: 29})", "E023");
    assert!(r.stderr.contains("a date has no field `dya` (did you mean `day`?)"), "{}", r.stderr);
    let r = fails("format_date({year: 2024, month: 2})", "E023");
    assert!(r.stderr.contains("the field `day` is missing"), "{}", r.stderr);
    let r = fails("format_date({year: 2024, month: 2, day: 1.5})", "E023");
    assert!(r.stderr.contains("the field `day` must be a whole number, not Float 1.5"), "{}", r.stderr);
    let r = fails("days_between({year: 2024, month: 1, day: 1}, {year: 2024, month: 1, day: 1, offset: 1200})", "E023");
    assert!(r.stderr.contains("the offset must go from -1080 to 1080"), "{}", r.stderr);
    // A text given for a date: before running, with a hint.
    let r = fails("print(add_days(\"2024-01-01\", 1))", "E005");
    assert!(
        r.stderr.contains("expects a date (a record {year, month, day}) as argument 1") && r.stderr.contains("parse_date(text)"),
        "{}",
        r.stderr
    );
    let r = rythme_lang("fr", &["-e", "day_of_week({year: 2024, month: 4, day: 31})"]);
    assert!(
        r.stderr.contains("`day_of_week` a besoin d'une date valide : le jour doit aller de 1 à 30 en avril 2024, pas 31"),
        "{}",
        r.stderr
    );
}

#[test]
fn the_checker_knows_the_dates() {
    // Found before running, in code that never runs.
    fails("fn later() = format_date(42)\nprint(1)", "E005");
    fails("fn later() = now().year + \"!\"\nprint(1)", "E005");
    fails("fn later() = unwrap(parse_date(\"2024-01-01\")).day + \"!\"\nprint(1)", "E005");
    // No false alarm: a date in a name, built or computed.
    ok(
        "fn next_week(d) = add_days(d, 7)\nd = {year: 2024, month: 2, day: 29}\nprint(next_week(d).month + 1, {..d, day: 1} |> day_of_week, today().year > 2000)",
        "4 4 true",
    );
}

#[test]
fn the_clock_gives_the_local_time_and_its_offset() {
    ok(
        "n = now()\nprint(keys(n), keys(today()), is_date(n), is_date(today()), n.offset >= -1080 and n.offset <= 1080)",
        "[\"year\", \"month\", \"day\", \"hour\", \"minute\", \"second\", \"offset\"] [\"year\", \"month\", \"day\"] true true true",
    );
    // `now()` is the same moment as `time()`, whatever the offset.
    ok("print(abs(seconds_between({year: 1970, month: 1, day: 1}, now()) - time()) < 5)", "true");
    ok("print(abs(days_between({..now(), offset: 0}, {..today(), hour: 12, minute: 0, second: 0, offset: 0})) <= 1)", "true");
}

#[cfg(all(unix, target_pointer_width = "64"))]
#[test]
fn the_offset_comes_from_the_time_zone_of_the_system() {
    for (tz, offset) in [("UTC0", "0"), ("XXX-2", "120"), ("XXX+3:30", "-210"), ("XXX-5:45", "345")] {
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_rythme"))
            .args(["-e", "n = now()\nprint(n.offset, abs(seconds_between({year: 1970, month: 1, day: 1}, n) - time()) < 5)"])
            .env("TZ", tz)
            .output()
            .expect("run rythme");
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), format!("{offset} true"), "TZ={tz}");
    }
}

#[test]
fn names_from_other_languages_get_a_hint() {
    for (name, rythme) in [
        ("strftime", "format_date"),
        ("toISOString", "format_date"),
        ("strptime", "parse_date"),
        ("weekday", "day_of_week"),
        ("isLeapYear", "is_leap_year"),
        ("utcnow", "now"),
    ] {
        let r = fails(&format!("{name}(1)"), "E003");
        assert!(r.stderr.contains(&format!("called `{rythme}`")), "{name}: {}", r.stderr);
    }
}

#[test]
fn help_knows_the_dates() {
    for lang in ["en", "fr"] {
        let r = rythme_lang(lang, &["help", "dates"]);
        assert_eq!(r.code, 0);
        assert!(r.stdout.contains("parse_date") && r.stdout.contains("ISO 8601"), "{}", r.stdout);
    }
    let r = rythme_lang("fr", &["help", "calendrier"]);
    assert!(r.stdout.contains("DATES ET HEURES"), "{}", r.stdout);
    let r = rythme(&["help", "builtins"]);
    assert!(r.stdout.contains("Dates and times") && r.stdout.contains("days_between(a, b)"), "{}", r.stdout);
    let r = rythme(&["-e", "help({year: 2024, month: 2, day: 29})"]);
    assert!(r.stdout.contains("add_days") && r.stdout.contains("map_values"), "{}", r.stdout);
    let r = rythme(&["-e", "help({year: 2024, month: 2, day: 30})"]);
    assert!(!r.stdout.contains("add_days"), "{}", r.stdout);
}
