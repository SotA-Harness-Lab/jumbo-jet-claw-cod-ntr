// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use pretty_assertions::assert_eq;

use super::CronJobs;
use super::MAX_JOBS;

fn at(day: u32, hour: u32, minute: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 11, day)
        .and_then(|date| date.and_hms_opt(hour, minute, 0))
        .expect("a valid time")
}

#[tokio::test]
async fn a_recurring_job_is_put_off_and_a_one_shot_job_removed() {
    let jobs = CronJobs::default();
    let every_hour = jobs
        .create("0 * * * *", "check the build".to_string(), true, at(20, 10, 7))
        .await
        .expect("create");
    let once = jobs
        .create("30 10 * * *", "remind me".to_string(), false, at(20, 10, 7))
        .await
        .expect("create");

    assert_eq!(jobs.take_due(at(20, 10, 29)).await, Vec::new());

    let due = jobs.take_due(at(20, 10, 30)).await;
    assert_eq!(
        due.iter()
            .map(|prompt| (prompt.id.as_str(), prompt.last))
            .collect::<Vec<_>>(),
        vec![(once.id.as_str(), true)]
    );

    let due = jobs.take_due(at(20, 11, 0)).await;
    assert_eq!(
        due.iter()
            .map(|prompt| (prompt.id.as_str(), prompt.last, prompt.prompt.as_str()))
            .collect::<Vec<_>>(),
        vec![(every_hour.id.as_str(), false, "check the build")]
    );
    // Taken once: it is due again only at its next time.
    assert_eq!(jobs.take_due(at(20, 11, 30)).await, Vec::new());
    assert_eq!(jobs.take_due(at(20, 12, 0)).await.len(), 1);
    assert_eq!(
        jobs.list()
            .await
            .iter()
            .map(|job| job.id.clone())
            .collect::<Vec<_>>(),
        vec![every_hour.id]
    );
}

#[tokio::test]
async fn a_recurring_job_runs_a_last_time_once_it_is_a_week_old() {
    let jobs = CronJobs::default();
    jobs.create("0 9 * * *", "daily".to_string(), true, at(1, 10, 0))
        .await
        .expect("create");

    assert_eq!(jobs.take_due(at(7, 9, 0)).await[0].last, false);
    let last = jobs.take_due(at(8, 10, 0)).await;
    assert_eq!((last.len(), last[0].last), (1, true));
    assert_eq!(jobs.list().await, Vec::new());
}

#[tokio::test]
async fn jobs_are_refused_for_a_bad_expression_and_past_the_limit() {
    let jobs = CronJobs::default();
    assert!(
        jobs.create("61 * * * *", "x".to_string(), true, at(20, 10, 7))
            .await
            .is_err()
    );
    assert_eq!(
        jobs.create("0 0 31 2 *", "x".to_string(), true, at(20, 10, 7))
            .await,
        Err("the cron expression \"0 0 31 2 *\" matches no time within the next year".to_string())
    );
    for _ in 0..MAX_JOBS {
        jobs.create("* * * * *", "x".to_string(), true, at(20, 10, 7))
            .await
            .expect("create");
    }
    assert_eq!(
        jobs.create("* * * * *", "x".to_string(), true, at(20, 10, 7))
            .await,
        Err(format!("Too many scheduled jobs (max {MAX_JOBS}). Cancel one first."))
    );
    let id = jobs.list().await[0].id.clone();
    assert!(jobs.delete(&id).await);
    assert!(!jobs.delete(&id).await);
}
