//! Dates and times: records `{year, month, day}`, with `hour`, `minute`,
//! `second` and `offset` (minutes ahead of UTC) when needed, in the
//! Gregorian calendar, from the year 1 to the year 9999. No time zone
//! database: the local offset comes from the system when it gives one, and
//! is 0 (UTC) otherwise.

use super::*;

/// The fields a date may have, in the order they are shown.
const FIELDS: [&str; 7] = ["year", "month", "day", "hour", "minute", "second", "offset"];

/// The largest offset accepted, in minutes: 18 hours, like ISO 8601 libraries.
const MAX_OFFSET: i64 = 18 * 60;

const DAY: i64 = 86_400;

/// A valid date, read from a record or a text.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Date {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
    /// Has a time of day (`hour`, `minute`, `second` or `offset`): the
    /// records made from it have the time fields too.
    timed: bool,
    offset: Option<i64>,
}

pub(crate) fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub(crate) fn month_len(year: i64, month: i64) -> i64 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// The number of days from 1 January 1970 to this day (negative before).
pub(crate) fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The day that is `days` days after 1 January 1970: (year, month, day).
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// The months in English, translated by [`crate::lang::pick`].
pub(crate) const MONTHS_EN: [&str; 12] =
    ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

fn month_name(month: i64) -> &'static str {
    const FR: [&str; 12] =
        ["janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre"];
    let i = (month - 1).clamp(0, 11) as usize;
    crate::lang::pick(MONTHS_EN[i], FR[i])
}

impl Date {
    /// Why these numbers are not a date, if they are not.
    fn problem(&self) -> Option<String> {
        let Date { year, month, day, hour, minute, second, .. } = *self;
        if !(1..=9999).contains(&year) {
            return Some(t!("the year must go from 1 to 9999, not {year}", "l'année doit aller de 1 à 9999, pas {year}"));
        }
        if !(1..=12).contains(&month) {
            return Some(t!("the month must go from 1 to 12, not {month}", "le mois doit aller de 1 à 12, pas {month}"));
        }
        let len = month_len(year, month);
        if !(1..=len).contains(&day) {
            let name = month_name(month);
            let mut why = t!(
                "the day must go from 1 to {len} in {name} {year}, not {day}",
                "le jour doit aller de 1 à {len} en {name} {year}, pas {day}"
            );
            if month == 2 && day == 29 {
                why.push_str(&t!(" ({year} is not a leap year)", " ({year} n'est pas une année bissextile)"));
            }
            return Some(why);
        }
        if !(0..=23).contains(&hour) {
            return Some(t!("the hour must go from 0 to 23, not {hour}", "l'heure doit aller de 0 à 23, pas {hour}"));
        }
        if !(0..=59).contains(&minute) {
            return Some(t!("the minute must go from 0 to 59, not {minute}", "la minute doit aller de 0 à 59, pas {minute}"));
        }
        if !(0..=59).contains(&second) {
            return Some(t!("the second must go from 0 to 59, not {second}", "la seconde doit aller de 0 à 59, pas {second}"));
        }
        if let Some(offset) = self.offset {
            if !(-MAX_OFFSET..=MAX_OFFSET).contains(&offset) {
                return Some(t!(
                    "the offset must go from -1080 to 1080 minutes (-18:00 to +18:00), not {offset}",
                    "le décalage doit aller de -1080 à 1080 minutes (-18:00 à +18:00), pas {offset}"
                ));
            }
        }
        None
    }

    /// Reads a record; the text says what is wrong with it.
    fn read(r: &Record) -> Result<Date, String> {
        for k in r.keys() {
            if !FIELDS.contains(&&**k) {
                let mut why = t!("a date has no field `{k}`", "une date n'a pas de champ `{k}`");
                if let Some(best) = crate::suggest::closest(k, FIELDS) {
                    why.push_str(&t!(" (did you mean `{best}`?)", " (vouliez-vous dire `{best}` ?)"));
                }
                return Err(why);
            }
        }
        let mut n = [0i64; 7];
        let mut present = [false; 7];
        for (i, k) in FIELDS.iter().enumerate() {
            match r.get(k) {
                Some(Value::Int(x)) => {
                    n[i] = *x;
                    present[i] = true;
                }
                Some(Value::Big(_)) => {
                    return Err(t!("the field `{k}` is too large for a date", "le champ `{k}` est trop grand pour une date"))
                }
                Some(v) => {
                    return Err(t!(
                        "the field `{k}` must be a whole number, not {}",
                        "le champ `{k}` doit être un nombre entier, pas {}",
                        v.describe()
                    ))
                }
                None if i < 3 => return Err(t!("the field `{k}` is missing", "le champ `{k}` manque")),
                None => {}
            }
        }
        let d = Date {
            year: n[0],
            month: n[1],
            day: n[2],
            hour: n[3],
            minute: n[4],
            second: n[5],
            timed: present[3..].contains(&true),
            offset: present[6].then_some(n[6]),
        };
        match d.problem() {
            Some(why) => Err(why),
            None => Ok(d),
        }
    }

    /// Days since 1 January 1970, of the day as written (in its own offset).
    fn days(&self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }

    /// Seconds since 1 January 1970 as written, without the offset.
    fn wall_seconds(&self) -> i64 {
        self.days() * DAY + self.hour * 3600 + self.minute * 60 + self.second
    }

    /// Seconds since 1 January 1970 at 00:00 UTC.
    fn utc_seconds(&self) -> i64 {
        self.wall_seconds() - self.offset.unwrap_or(0) * 60
    }

    /// The date `wall` seconds after 1970-01-01 00:00 as written, if it is
    /// between the years 1 and 9999.
    fn from_wall(wall: i128, timed: bool, offset: Option<i64>) -> Option<Date> {
        let days = i64::try_from(wall.div_euclid(DAY as i128)).ok()?;
        if !(days_from_civil(1, 1, 1)..=days_from_civil(9999, 12, 31)).contains(&days) {
            return None;
        }
        let rest = wall.rem_euclid(DAY as i128) as i64;
        let (year, month, day) = civil_from_days(days);
        Some(Date { year, month, day, hour: rest / 3600, minute: rest / 60 % 60, second: rest % 60, timed, offset })
    }

    fn to_value(self) -> Value {
        let mut r = Record::new();
        let all = [self.year, self.month, self.day, self.hour, self.minute, self.second];
        let shown = if self.timed { 6 } else { 3 };
        for (k, v) in FIELDS.iter().zip(all).take(shown) {
            r.set(Rc::from(*k), Value::Int(v));
        }
        if let Some(o) = self.offset {
            r.set(Rc::from("offset"), Value::Int(o));
        }
        Value::Record(Rc::new(r))
    }

    /// ISO 8601: `2024-02-29`, `2024-02-29T13:05:00`, `2024-02-29T13:05:00+01:00`.
    fn iso(&self) -> String {
        let mut s = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);
        if self.timed {
            s.push_str(&format!("T{:02}:{:02}:{:02}", self.hour, self.minute, self.second));
            match self.offset {
                Some(0) => s.push('Z'),
                Some(o) => s.push_str(&format!("{}{:02}:{:02}", if o < 0 { '-' } else { '+' }, o.abs() / 60, o.abs() % 60)),
                None => {}
            }
        }
        s
    }

    /// Reads ISO 8601: `YYYY-MM-DD`, then optionally `THH:MM`, `:SS`, a
    /// fraction of second (ignored), and `Z` or `+HH:MM`, `-HH:MM`, `+HHMM`, `+HH`.
    fn parse(text: &str) -> Result<Date, String> {
        let s = text.trim().as_bytes();
        let mut at = 0;
        let digits = |at: &mut usize, n: usize| -> Option<i64> {
            let part = s.get(*at..*at + n)?;
            if !part.iter().all(u8::is_ascii_digit) {
                return None;
            }
            *at += n;
            Some(part.iter().fold(0, |acc, c| acc * 10 + i64::from(c - b'0')))
        };
        let expect = |at: &mut usize, c: &[u8]| -> Option<()> {
            if s.get(*at).is_some_and(|x| c.contains(x)) {
                *at += 1;
                Some(())
            } else {
                None
            }
        };
        let mut read = || -> Option<Date> {
            let year = digits(&mut at, 4)?;
            expect(&mut at, b"-")?;
            let month = digits(&mut at, 2)?;
            expect(&mut at, b"-")?;
            let day = digits(&mut at, 2)?;
            let mut d = Date { year, month, day, hour: 0, minute: 0, second: 0, timed: false, offset: None };
            if at == s.len() {
                return Some(d);
            }
            expect(&mut at, b"Tt ")?;
            d.timed = true;
            d.hour = digits(&mut at, 2)?;
            expect(&mut at, b":")?;
            d.minute = digits(&mut at, 2)?;
            if expect(&mut at, b":").is_some() {
                d.second = digits(&mut at, 2)?;
                if expect(&mut at, b".,").is_some() {
                    digits(&mut at, 1)?;
                    while digits(&mut at, 1).is_some() {}
                }
            }
            if expect(&mut at, b"Zz").is_some() {
                d.offset = Some(0);
            } else if at < s.len() {
                let sign = if expect(&mut at, b"+").is_some() {
                    1
                } else {
                    expect(&mut at, b"-")?;
                    -1
                };
                let hours = digits(&mut at, 2)?;
                let colon = expect(&mut at, b":").is_some();
                let minutes = match digits(&mut at, 2) {
                    Some(m) if m < 60 => m,
                    Some(_) => return None,
                    None if colon => return None,
                    None => 0,
                };
                d.offset = Some(sign * (hours * 60 + minutes));
            }
            (at == s.len()).then_some(d)
        };
        let Some(d) = read() else {
            return Err(t!(
                "\"{text}\" is not a date written as ISO 8601: write it like \"2024-02-29\" or \"2024-02-29T13:05:00\"",
                "\"{text}\" n'est pas une date écrite en ISO 8601 : écrivez-la comme \"2024-02-29\" ou \"2024-02-29T13:05:00\""
            ));
        };
        match d.problem() {
            Some(why) => Err(t!("\"{text}\" is not a valid date: {why}", "\"{text}\" n'est pas une date valide : {why}")),
            None => Ok(d),
        }
    }
}

// ----- argument helpers --------------------------------------------------

/// Argument `i` of `f`, which must be a record (checked before the other
/// arguments, as the checker does); [`valid`] reads it then.
fn date_record<'a>(a: &'a [Value], i: usize, f: &str, span: Span) -> Fallible<&'a Rc<Record>> {
    match &a[i] {
        Value::Record(r) => Ok(r),
        v => Err(wrong(f, i, Expected::Date, v, span)),
    }
}

fn valid(r: &Record, f: &str, span: Span) -> Fallible<Date> {
    Date::read(r).map_err(|why| {
        Diagnostic::error("E023", t!("`{f}` needs a valid date: {why}", "`{f}` a besoin d'une date valide : {why}"))
            .at(span)
            .label(t!("not a valid date", "pas une date valide"))
            .help(t!(
                "a date is a record like `{{year: 2024, month: 2, day: 29}}`, with `hour`, `minute`, `second` and `offset` (minutes ahead of UTC) if needed",
                "une date est un enregistrement comme `{{year: 2024, month: 2, day: 29}}`, avec `hour`, `minute`, `second` et `offset` (minutes d'avance sur UTC) si besoin"
            ))
            .boxed()
    })
}

fn date(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<Date> {
    valid(date_record(a, i, f, span)?, f, span)
}

fn out_of_years(f: &str, span: Span) -> Box<Diagnostic> {
    invalid(t!("`{f}` gives a date outside of the years 1 to 9999", "`{f}` donne une date en dehors des années 1 à 9999"), span)
}

fn month_arg(a: &[Value], i: usize, f: &str, span: Span) -> Fallible<i64> {
    let m = int(a, i, f, span)?;
    if !(1..=12).contains(&m) {
        return Err(invalid(t!("the month must go from 1 to 12, not {m}", "le mois doit aller de 1 à 12, pas {m}"), span));
    }
    Ok(m)
}

/// The date and time now, where the program runs.
fn current(timed: bool, span: Span) -> Fallible<Value> {
    let now = crate::io::now().floor();
    let secs = if now.is_finite() { now.clamp(-1e15, 1e15) as i64 } else { 0 };
    let offset = crate::io::local_offset(secs).filter(|o| o.abs() <= MAX_OFFSET).unwrap_or(0);
    match Date::from_wall(i128::from(secs) + i128::from(offset) * 60, timed, timed.then_some(offset)) {
        Some(d) => Ok(d.to_value()),
        None => Err(invalid(
            t!(
                "the clock of the system gives a date outside of the years 1 to 9999",
                "l'horloge du système donne une date en dehors des années 1 à 9999"
            ),
            span,
        )),
    }
}

// ----- the functions -----------------------------------------------------

pub(super) fn now(_: &mut Interp, _: &[Value], span: Span) -> Fallible<Value> {
    current(true, span)
}

pub(super) fn today(_: &mut Interp, _: &[Value], span: Span) -> Fallible<Value> {
    current(false, span)
}

pub(super) fn parse_date(it: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let s = text(a, 0, "parse_date", span)?;
    let read = match a.get(1) {
        Some(_) => parse_pattern(s, text(a, 1, "parse_date", span)?, span)?,
        None => Date::parse(s),
    };
    Ok(match read {
        Ok(d) => it.ok(d.to_value()),
        Err(why) => it.err(text_value(why)),
    })
}

pub(super) fn format_date(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = date_record(a, 0, "format_date", span)?;
    let pattern = if a.len() == 2 { Some(text(a, 1, "format_date", span)?) } else { None };
    let d = valid(r, "format_date", span)?;
    Ok(text_value(match pattern {
        Some(p) => format_pattern(&d, p, span)?,
        None => d.iso(),
    }))
}

pub(super) fn is_date(_: &mut Interp, a: &[Value], _: Span) -> Fallible<Value> {
    Ok(Value::Bool(matches!(&a[0], Value::Record(r) if Date::read(r).is_ok())))
}

pub(super) fn add_days(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = date_record(a, 0, "add_days", span)?;
    let n = int(a, 1, "add_days", span)?;
    let d = valid(r, "add_days", span)?;
    let wall = i128::from(d.wall_seconds()) + i128::from(n) * i128::from(DAY);
    Date::from_wall(wall, d.timed, d.offset).map(Date::to_value).ok_or_else(|| out_of_years("add_days", span))
}

pub(super) fn add_seconds(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = date_record(a, 0, "add_seconds", span)?;
    let n = int(a, 1, "add_seconds", span)?;
    let d = valid(r, "add_seconds", span)?;
    let wall = i128::from(d.wall_seconds()) + i128::from(n);
    Date::from_wall(wall, true, d.offset).map(Date::to_value).ok_or_else(|| out_of_years("add_seconds", span))
}

fn two_dates(a: &[Value], f: &str, span: Span) -> Fallible<(Date, Date)> {
    let (x, y) = (date_record(a, 0, f, span)?, date_record(a, 1, f, span)?);
    Ok((valid(x, f, span)?, valid(y, f, span)?))
}

pub(super) fn seconds_between(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y) = two_dates(a, "seconds_between", span)?;
    Ok(Value::Int(y.utc_seconds() - x.utc_seconds()))
}

pub(super) fn days_between(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let (x, y) = two_dates(a, "days_between", span)?;
    Ok(Value::Int((y.utc_seconds() - x.utc_seconds()) / DAY))
}

pub(super) fn day_of_week(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let d = date(a, 0, "day_of_week", span)?;
    // 1 January 1970 was a Thursday, the 4th day of an ISO week.
    Ok(Value::Int((d.days() + 3).rem_euclid(7) + 1))
}

pub(super) fn days_in_month(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let year = int(a, 0, "days_in_month", span)?;
    let month = month_arg(a, 1, "days_in_month", span)?;
    Ok(Value::Int(month_len(year, month)))
}

pub(super) fn is_leap_year(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(Value::Bool(is_leap(int(a, 0, "is_leap_year", span)?)))
}

/// Does this record hold a valid date? (`help(value)` lists the date functions then.)
pub(crate) fn is_date_record(r: &Record) -> bool {
    Date::read(r).is_ok()
}

// ----- months, days of the year, patterns ---------------------------------

pub(super) fn add_months(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let r = date_record(a, 0, "add_months", span)?;
    let n = int(a, 1, "add_months", span)?;
    let d = valid(r, "add_months", span)?;
    let months = i128::from(d.year) * 12 + i128::from(d.month - 1) + i128::from(n);
    let year = months.div_euclid(12);
    if !(1..=9999).contains(&year) {
        return Err(out_of_years("add_months", span));
    }
    let (year, month) = (year as i64, months.rem_euclid(12) as i64 + 1);
    // 31 January + 1 month: the last day of February.
    let day = d.day.min(month_len(year, month));
    Ok(Date { year, month, day, ..d }.to_value())
}

pub(super) fn day_of_year(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    let d = date(a, 0, "day_of_year", span)?;
    Ok(Value::Int(d.days() - days_from_civil(d.year, 1, 1) + 1))
}

/// The days of the week in English, Monday first (patterns are not translated:
/// a program prints the same thing in every language).
const WEEKDAYS_EN: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

fn pattern_error(pattern: &str, c: char, span: Span) -> Box<Diagnostic> {
    Diagnostic::error(
        "E023",
        t!("the pattern \"{pattern}\" has an unknown directive `%{c}`", "le motif \"{pattern}\" a une directive inconnue `%{c}`"),
    )
    .at(span)
    .help(t!(
        "the directives are %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second), %y (year in 2 digits), %B and %b (month name), %A and %a (day name), %I and %p (12-hour clock), %j (day of the year), %z (offset) and %% (a %)",
        "les directives sont %Y (année), %m (mois), %d (jour), %H (heure), %M (minute), %S (seconde), %y (année en 2 chiffres), %B et %b (nom du mois), %A et %a (nom du jour), %I et %p (horloge de 12 heures), %j (jour de l'année), %z (décalage) et %% (un %)"
    ))
    .boxed()
}

/// A date written with a pattern like `"%d/%m/%Y %H:%M"`, as `strftime` does.
fn format_pattern(d: &Date, pattern: &str, span: Span) -> Fallible<String> {
    let mut out = String::new();
    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let Some(k) = chars.next() else { return Err(pattern_error(pattern, ' ', span)) };
        let month = MONTHS_EN[(d.month - 1) as usize];
        let weekday = WEEKDAYS_EN[(d.days() + 3).rem_euclid(7) as usize];
        let hour12 = if d.hour % 12 == 0 { 12 } else { d.hour % 12 };
        match k {
            'Y' => out.push_str(&format!("{:04}", d.year)),
            'y' => out.push_str(&format!("{:02}", d.year % 100)),
            'm' => out.push_str(&format!("{:02}", d.month)),
            'd' => out.push_str(&format!("{:02}", d.day)),
            'H' => out.push_str(&format!("{:02}", d.hour)),
            'I' => out.push_str(&format!("{hour12:02}")),
            'M' => out.push_str(&format!("{:02}", d.minute)),
            'S' => out.push_str(&format!("{:02}", d.second)),
            'p' => out.push_str(if d.hour < 12 { "AM" } else { "PM" }),
            'j' => out.push_str(&format!("{:03}", d.days() - days_from_civil(d.year, 1, 1) + 1)),
            'B' => out.push_str(month),
            'b' => out.push_str(&month[..3]),
            'A' => out.push_str(weekday),
            'a' => out.push_str(&weekday[..3]),
            'z' => {
                let o = d.offset.unwrap_or(0);
                out.push_str(&format!("{}{:02}{:02}", if o < 0 { '-' } else { '+' }, o.abs() / 60, o.abs() % 60));
            }
            '%' => out.push('%'),
            other => return Err(pattern_error(pattern, other, span)),
        }
    }
    Ok(out)
}

/// Reads a date written with a pattern: `Ok(date)`, `Err(why)` when the text
/// does not follow it, or a diagnostic for a pattern that is not one.
fn parse_pattern(text: &str, pattern: &str, span: Span) -> Fallible<Result<Date, String>> {
    let s: Vec<char> = text.trim().chars().collect();
    let mut at = 0;
    let mut d = Date { year: 1900, month: 1, day: 1, hour: 0, minute: 0, second: 0, timed: false, offset: None };
    let mut pm: Option<bool> = None;
    let mismatch = || Ok(Err(t!("\"{text}\" does not follow the pattern \"{pattern}\"", "\"{text}\" ne suit pas le motif \"{pattern}\"")));
    // A number of 1 to `max` digits.
    let number = |at: &mut usize, max: usize| -> Option<i64> {
        let start = *at;
        while *at < s.len() && *at - start < max && s[*at].is_ascii_digit() {
            *at += 1;
        }
        (*at > start).then(|| s[start..*at].iter().fold(0, |acc, c| acc * 10 + i64::from(c.to_digit(10).unwrap_or(0))))
    };
    // One of these names (or its first three letters), whatever the case.
    let name = |at: &mut usize, names: &[&str]| -> Option<i64> {
        let rest: String = s[*at..].iter().collect::<String>().to_lowercase();
        for (i, n) in names.iter().enumerate() {
            for candidate in [n.to_lowercase(), n[..3].to_lowercase()] {
                if rest.starts_with(&candidate) {
                    *at += candidate.chars().count();
                    return Some(i as i64);
                }
            }
        }
        None
    };
    let mut p = pattern.chars();
    while let Some(c) = p.next() {
        if c != '%' {
            if c.is_whitespace() {
                while at < s.len() && s[at].is_whitespace() {
                    at += 1;
                }
                continue;
            }
            if s.get(at) != Some(&c) {
                return mismatch();
            }
            at += 1;
            continue;
        }
        let Some(k) = p.next() else { return Err(pattern_error(pattern, ' ', span)) };
        let read = match k {
            'Y' => number(&mut at, 4).map(|n| d.year = n),
            'y' => number(&mut at, 2).map(|n| d.year = if n < 69 { 2000 + n } else { 1900 + n }),
            'm' => number(&mut at, 2).map(|n| d.month = n),
            'd' => number(&mut at, 2).map(|n| d.day = n),
            'H' | 'I' => number(&mut at, 2).map(|n| {
                d.hour = n;
                d.timed = true;
            }),
            'M' => number(&mut at, 2).map(|n| {
                d.minute = n;
                d.timed = true;
            }),
            'S' => number(&mut at, 2).map(|n| {
                d.second = n;
                d.timed = true;
            }),
            'j' => number(&mut at, 3).map(|n| d.day = -n),
            'B' | 'b' => name(&mut at, &MONTHS_EN).map(|i| d.month = i + 1),
            'A' | 'a' => name(&mut at, &WEEKDAYS_EN).map(|_| ()),
            'p' => {
                let rest: String = s[at..].iter().take(2).collect::<String>().to_uppercase();
                let found = match rest.as_str() {
                    "AM" => Some(false),
                    "PM" => Some(true),
                    _ => None,
                };
                found.map(|found| {
                    at += 2;
                    pm = Some(found);
                })
            }
            'z' => {
                if s.get(at).is_some_and(|c| *c == 'Z' || *c == 'z') {
                    at += 1;
                    d.offset = Some(0);
                    Some(())
                } else {
                    let sign = match s.get(at) {
                        Some('+') => 1,
                        Some('-') => -1,
                        _ => return mismatch(),
                    };
                    at += 1;
                    let hours = number(&mut at, 2);
                    if s.get(at) == Some(&':') {
                        at += 1;
                    }
                    let minutes = number(&mut at, 2);
                    match (hours, minutes) {
                        (Some(h), Some(m)) if m < 60 => {
                            d.offset = Some(sign * (h * 60 + m));
                            d.timed = true;
                            Some(())
                        }
                        _ => None,
                    }
                }
            }
            '%' => (s.get(at) == Some(&'%')).then(|| at += 1),
            other => return Err(pattern_error(pattern, other, span)),
        };
        if read.is_none() {
            return mismatch();
        }
    }
    if at != s.len() {
        return mismatch();
    }
    if let Some(pm) = pm {
        if !(1..=12).contains(&d.hour) {
            return Ok(Err(t!(
                "\"{text}\" is not a valid date: {why}",
                "\"{text}\" n'est pas une date valide : {why}",
                why =
                    t!("the hour must go from 1 to 12 with AM or PM, not {}", "l'heure doit aller de 1 à 12 avec AM ou PM, pas {}", d.hour)
            )));
        }
        d.hour = d.hour % 12 + if pm { 12 } else { 0 };
    }
    // `%j`: the day of the year, counted from 1 January.
    if d.day < 0 {
        let n = -d.day;
        let len = if is_leap(d.year) { 366 } else { 365 };
        if !(1..=len).contains(&n) {
            return Ok(Err(t!(
                "\"{text}\" is not a valid date: {why}",
                "\"{text}\" n'est pas une date valide : {why}",
                why = t!("the day of the year must go from 1 to {len}, not {n}", "le jour de l'année doit aller de 1 à {len}, pas {n}")
            )));
        }
        let (_, month, day) = civil_from_days(days_from_civil(d.year, 1, 1) + n - 1);
        d.month = month;
        d.day = day;
    }
    Ok(match d.problem() {
        Some(why) => Err(t!("\"{text}\" is not a valid date: {why}", "\"{text}\" n'est pas une date valide : {why}")),
        None => Ok(d),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_and_civil_dates_agree_over_the_whole_range() {
        let mut days = days_from_civil(1, 1, 1);
        for year in 1..=9999 {
            for month in 1..=12 {
                for day in 1..=month_len(year, month) {
                    assert_eq!(days_from_civil(year, month, day), days, "{year}-{month}-{day}");
                    assert_eq!(civil_from_days(days), (year, month, day));
                    days += 1;
                }
            }
        }
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days, days_from_civil(9999, 12, 31) + 1);
    }

    #[test]
    fn leap_years() {
        assert!(is_leap(2024) && is_leap(2000) && is_leap(1600));
        assert!(!is_leap(2023) && !is_leap(1900) && !is_leap(2100));
        assert_eq!((1..=400).filter(|&y| is_leap(y)).count(), 97);
    }
}
