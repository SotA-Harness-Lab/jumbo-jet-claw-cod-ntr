Schedule a prompt to be delivered to you at times given by a cron expression, so that you can check on something later or do something regularly without the user asking again.

- `cron` is a standard 5-field expression in the user's local time: minute hour day-of-month month day-of-week. Examples: `*/10 * * * *` every 10 minutes, `0 9 * * 1-5` at 09:00 on weekdays, `30 14 20 11 *` once at 14:30 on November 20.
- `recurring` (default true): the prompt is delivered at every matching time. With false it is delivered once, at the next matching time, and the job is then deleted. For a one-shot job, pin minute, hour, day and month.
- Unless the user asked for an exact time, avoid minute 0 and 30, when everything else is scheduled too.
- A job is only delivered while no turn is running; one that comes due during a turn waits for it to end. It then starts a turn by itself, with the prompt, as if you had been sent it.
- Jobs live in this session only: they are gone when it ends. A recurring job is delivered one last time 7 days after it was created and is then deleted; tell the user about this limit when you create one.
- At most 50 jobs. Use CronList to see them and CronDelete to cancel one.