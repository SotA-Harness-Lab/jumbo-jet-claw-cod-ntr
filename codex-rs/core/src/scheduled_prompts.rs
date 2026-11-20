// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The jobs of the `CronCreate` tool: prompts delivered to the session at the times a cron
//! expression gives.
//!
//! Jobs live with the session and are gone when it ends. One task a session looks once a
//! second for jobs that are due and delivers them while no turn is running; a job that
//! comes due during a turn waits for the turn to end.

use std::sync::Arc;
use std::sync::Weak;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use chrono::NaiveDateTime;
use codex_claude_compat::tools::cron::CronSchedule;
use tokio::sync::Mutex;

use crate::context::ContextualUserFragment;
use crate::context::ScheduledPrompt;
use crate::session::session::Session;

/// How many jobs a session may hold.
pub(crate) const MAX_JOBS: usize = 50;

/// A recurring job runs one last time once it is this old, and is then removed, so that a
/// forgotten job does not go on for the life of a long session.
const RECURRING_MAX_AGE: chrono::Duration = chrono::Duration::days(7);

const TICK: Duration = Duration::from_secs(1);

/// One scheduled prompt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CronJob {
    pub(crate) id: String,
    pub(crate) cron: String,
    pub(crate) schedule: CronSchedule,
    pub(crate) prompt: String,
    pub(crate) recurring: bool,
    created_at: NaiveDateTime,
    next_run: NaiveDateTime,
}

/// A session's jobs, kept in its extension data.
#[derive(Default)]
pub(crate) struct CronJobs {
    jobs: Mutex<Vec<CronJob>>,
    scheduler_started: AtomicBool,
}

impl CronJobs {
    /// The jobs of `session`.
    pub(crate) fn of(session: &Session) -> Arc<Self> {
        session
            .services
            .thread_extension_data
            .get_or_init(Self::default)
    }

    /// Adds a job, due first at the schedule's next time after `now`. `Err` says why not.
    pub(crate) async fn create(
        &self,
        cron: &str,
        prompt: String,
        recurring: bool,
        now: NaiveDateTime,
    ) -> Result<CronJob, String> {
        let schedule = CronSchedule::parse(cron)?;
        let next_run = schedule.next_after(now).ok_or_else(|| {
            format!("the cron expression {cron:?} matches no time within the next year")
        })?;
        let mut jobs = self.jobs.lock().await;
        if jobs.len() >= MAX_JOBS {
            return Err(format!(
                "Too many scheduled jobs (max {MAX_JOBS}). Cancel one first."
            ));
        }
        let job = CronJob {
            id: uuid::Uuid::new_v4().simple().to_string()[..8].to_string(),
            cron: cron.split_whitespace().collect::<Vec<_>>().join(" "),
            schedule,
            prompt,
            recurring,
            created_at: now,
            next_run,
        };
        jobs.push(job.clone());
        Ok(job)
    }

    pub(crate) async fn list(&self) -> Vec<CronJob> {
        self.jobs.lock().await.clone()
    }

    /// Removes the job `id`; whether there was one.
    pub(crate) async fn delete(&self, id: &str) -> bool {
        let mut jobs = self.jobs.lock().await;
        let before = jobs.len();
        jobs.retain(|job| job.id != id);
        jobs.len() != before
    }

    /// Takes the jobs due at `now`, each as the prompt to deliver. A recurring job is put
    /// off to its next time; a one-shot job, and a recurring one past its age limit or with
    /// no further time, is removed and its prompt marked as the last.
    async fn take_due(&self, now: NaiveDateTime) -> Vec<ScheduledPrompt> {
        let mut jobs = self.jobs.lock().await;
        let mut due = Vec::new();
        jobs.retain_mut(|job| {
            if job.next_run > now {
                return true;
            }
            let next_run = (job.recurring && now - job.created_at < RECURRING_MAX_AGE)
                .then(|| job.schedule.next_after(now))
                .flatten();
            due.push(ScheduledPrompt {
                id: job.id.clone(),
                cron: job.cron.clone(),
                recurring: job.recurring,
                last: next_run.is_none(),
                prompt: job.prompt.clone(),
            });
            match next_run {
                Some(next_run) => {
                    job.next_run = next_run;
                    true
                }
                None => false,
            }
        });
        due
    }

    /// Starts the session's scheduler, once. It ends with the session.
    pub(crate) fn ensure_scheduler(self: &Arc<Self>, session: &Arc<Session>) {
        if self.scheduler_started.swap(true, Ordering::AcqRel) {
            return;
        }
        let jobs = Arc::clone(self);
        let session: Weak<Session> = Arc::downgrade(session);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(TICK).await;
                let Some(session) = session.upgrade() else {
                    break;
                };
                if session.turn_starts_closed() {
                    break;
                }
                // A prompt delivered into a running turn would cut into other work.
                if session.active_turn.lock().await.is_some() {
                    continue;
                }
                for prompt in jobs.take_due(chrono::Local::now().naive_local()).await {
                    session
                        .deliver_background_notice(ContextualUserFragment::into(prompt))
                        .await;
                }
            }
        });
    }
}

/// The time of a job's next run, for the tool's answers.
pub(crate) fn describe_next_run(job: &CronJob) -> String {
    job.next_run.format("%Y-%m-%d %H:%M").to_string()
}

#[cfg(test)]
#[path = "scheduled_prompts_tests.rs"]
mod tests;
