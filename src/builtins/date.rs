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

fn month_name(month: i64) -> &'static str {
    const EN: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const FR: [&str; 12] =
        ["janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre"];
    let i = (month - 1).clamp(0, 11) as usize;
    crate::lang::pick(EN[i], FR[i])
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
    Ok(match Date::parse(s) {
        Ok(d) => it.ok(d.to_value()),
        Err(why) => it.err(text_value(why)),
    })
}

pub(super) fn format_date(_: &mut Interp, a: &[Value], span: Span) -> Fallible<Value> {
    Ok(text_value(date(a, 0, "format_date", span)?.iso()))
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
