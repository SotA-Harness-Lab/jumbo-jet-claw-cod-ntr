// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use pretty_assertions::assert_eq;

use super::CronSchedule;

fn at(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(hour, minute, 30))
        .expect("a valid time")
}

fn next(expression: &str, after: NaiveDateTime) -> Option<String> {
    CronSchedule::parse(expression)
        .expect("a valid expression")
        .next_after(after)
        .map(|time| time.format("%Y-%m-%d %H:%M").to_string())
}

#[test]
fn the_next_time_is_the_first_matching_minute_after() {
    // 2026-11-20 is a Friday.
    let now = at(2026, 11, 20, 10, 7);
    for (expression, expected) in [
        ("* * * * *", "2026-11-20 10:08"),
        ("*/15 * * * *", "2026-11-20 10:15"),
        ("7 * * * *", "2026-11-20 11:07"),
        ("30 9 * * *", "2026-11-21 09:30"),
        ("0 9 * * 1-5", "2026-11-23 09:00"),
        ("0 9 * * 0", "2026-11-22 09:00"),
        ("0 9 * * 7", "2026-11-22 09:00"),
        ("0 0 1 * *", "2026-12-01 00:00"),
        ("15,45 10-11 * * *", "2026-11-20 10:15"),
        ("5/20 * * * *", "2026-11-20 10:25"),
        ("0 12 25 12 *", "2026-12-25 12:00"),
    ] {
        assert_eq!(next(expression, now).as_deref(), Some(expected), "{expression}");
    }
}

#[test]
fn day_of_month_and_day_of_week_together_match_either() {
    // The 1st of December 2026 is a Tuesday; the next Monday is the 23rd of November.
    assert_eq!(
        next("0 8 1 * 1", at(2026, 11, 20, 10, 7)).as_deref(),
        Some("2026-11-23 08:00")
    );
    assert_eq!(
        next("0 8 1 * 1", at(2026, 11, 30, 9, 0)).as_deref(),
        Some("2026-12-01 08:00")
    );
}

#[test]
fn a_schedule_with_no_time_within_a_year_has_no_next_time() {
    assert_eq!(next("0 0 31 2 *", at(2026, 11, 20, 10, 7)), None);
    // The 29th of February comes within the year searched only before a leap year.
    assert_eq!(
        next("0 0 29 2 *", at(2027, 11, 20, 10, 7)).as_deref(),
        Some("2028-02-29 00:00")
    );
    assert_eq!(next("0 0 29 2 *", at(2026, 11, 20, 10, 7)), None);
}

#[test]
fn expressions_that_are_not_five_valid_fields_are_refused() {
    for expression in [
        "",
        "* * * *",
        "* * * * * *",
        "60 * * * *",
        "* 24 * * *",
        "* * 0 * *",
        "* * * 13 *",
        "* * * * 8",
        "*/0 * * * *",
        "10-5 * * * *",
        "a * * * *",
    ] {
        assert!(CronSchedule::parse(expression).is_err(), "{expression:?}");
    }
}
