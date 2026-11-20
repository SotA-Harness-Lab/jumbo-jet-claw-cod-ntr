// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The schedules of the `CronCreate` tool: five-field cron expressions (minute, hour, day
//! of month, month, day of week) read in local time, as Claude Code's tool takes them.
//!
//! Each field is `*`, a number, a range `a-b`, a step `*/n` or `a-b/n`, or a list of those
//! separated by commas. Sunday is 0 or 7. When both the day of month and the day of week
//! are restricted, a day matches when either does, as in cron.

use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike;

/// How far ahead a schedule is searched for its next time: a schedule without one within a
/// year (`0 0 31 2 *`) is refused.
const SEARCH_DAYS: i64 = 366;

/// A parsed cron expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CronSchedule {
    minutes: Vec<u32>,
    hours: Vec<u32>,
    days_of_month: Vec<u32>,
    months: Vec<u32>,
    /// 0 is Sunday.
    days_of_week: Vec<u32>,
    /// Whether the day of month, and the day of week, were written as something other
    /// than `*`.
    day_of_month_restricted: bool,
    day_of_week_restricted: bool,
}

impl CronSchedule {
    /// Parses a five-field expression.
    pub fn parse(expression: &str) -> Result<Self, String> {
        let fields = expression.split_whitespace().collect::<Vec<_>>();
        let [minute, hour, day_of_month, month, day_of_week] = fields.as_slice() else {
            return Err(format!(
                "a cron expression has 5 fields (minute hour day-of-month month day-of-week), got {}",
                fields.len()
            ));
        };
        let mut days_of_week = parse_field(day_of_week, "day of week", 0, 7)?;
        // 7 is Sunday too.
        for day in &mut days_of_week {
            *day %= 7;
        }
        days_of_week.sort_unstable();
        days_of_week.dedup();
        Ok(Self {
            minutes: parse_field(minute, "minute", 0, 59)?,
            hours: parse_field(hour, "hour", 0, 23)?,
            days_of_month: parse_field(day_of_month, "day of month", 1, 31)?,
            months: parse_field(month, "month", 1, 12)?,
            days_of_week,
            day_of_month_restricted: !is_any(day_of_month),
            day_of_week_restricted: !is_any(day_of_week),
        })
    }

    /// The first minute after `after` the schedule matches, within a year.
    pub fn next_after(&self, after: NaiveDateTime) -> Option<NaiveDateTime> {
        let mut candidate = after
            .with_second(0)?
            .with_nanosecond(0)?
            .checked_add_signed(Duration::minutes(1))?;
        let limit = after.checked_add_signed(Duration::days(SEARCH_DAYS))?;
        while candidate <= limit {
            if !self.months.contains(&candidate.month()) || !self.day_matches(candidate) {
                // Nothing in this day matches: go to its end.
                candidate = candidate
                    .date()
                    .succ_opt()?
                    .and_hms_opt(0, 0, 0)?;
                continue;
            }
            if !self.hours.contains(&candidate.hour()) {
                candidate = candidate
                    .with_minute(0)?
                    .checked_add_signed(Duration::hours(1))?;
                continue;
            }
            if self.minutes.contains(&candidate.minute()) {
                return Some(candidate);
            }
            candidate = candidate.checked_add_signed(Duration::minutes(1))?;
        }
        None
    }

    fn day_matches(&self, time: NaiveDateTime) -> bool {
        let day_of_month = self.days_of_month.contains(&time.day());
        let day_of_week = self
            .days_of_week
            .contains(&time.weekday().num_days_from_sunday());
        match (self.day_of_month_restricted, self.day_of_week_restricted) {
            (true, true) => day_of_month || day_of_week,
            (true, false) => day_of_month,
            (false, true) => day_of_week,
            (false, false) => true,
        }
    }
}

fn is_any(field: &str) -> bool {
    field == "*"
}

/// The values a field names, in order, between `min` and `max`.
fn parse_field(field: &str, name: &str, min: u32, max: u32) -> Result<Vec<u32>, String> {
    let invalid = || format!("invalid {name} in the cron expression: {field:?}");
    let number = |text: &str| {
        text.parse::<u32>()
            .ok()
            .filter(|value| (min..=max).contains(value))
            .ok_or_else(invalid)
    };
    let mut values = Vec::new();
    for part in field.split(',') {
        let (range, step) = match part.split_once('/') {
            Some((range, step)) => (
                range,
                step.parse::<u32>()
                    .ok()
                    .filter(|step| *step > 0)
                    .ok_or_else(invalid)?,
            ),
            None => (part, 1),
        };
        let (start, end) = if range == "*" {
            (min, max)
        } else if let Some((start, end)) = range.split_once('-') {
            (number(start)?, number(end)?)
        } else if part.contains('/') {
            // `5/15`: from 5 on, every 15.
            (number(range)?, max)
        } else {
            let value = number(range)?;
            (value, value)
        };
        if start > end {
            return Err(invalid());
        }
        values.extend((start..=end).step_by(step as usize));
    }
    values.sort_unstable();
    values.dedup();
    Ok(values)
}

#[cfg(test)]
#[path = "cron_tests.rs"]
mod tests;
