use std::time::Duration;

mod interval;
pub use interval::IntervalSchedule;

mod oneshot;
pub use oneshot::OneShot;

cfg_feature! {
    "cron",
    mod cron;
    pub use cron::CronSchedule;
}

cfg_feature! {
    "backoff",
    mod backoff;
    pub use backoff::BackoffSchedule;
}

/// Defines an execution schedule for a given task.
/// All duration values returned define how long until
/// the task should start execution.
pub trait Schedule<T> {
    /// How long to wait for the initial execution. Exists since
    /// the task's output is not available initially. Can be used
    /// to control initial task execution delays.
    fn initial(&self) -> Option<Duration> {
        Some(Duration::from_secs(0))
    }
    /// Returns the time until this task should be scheduled again.
    /// None indicates that this task should never run again.
    /// `task_output` is the last return value of the task, which
    /// can be ignored if desired.
    fn next(&self, task_output: T) -> Option<Duration>;

    /// Returns the time until this task should be scheduled again.
    /// Only called when the previous task execution fails to return
    /// a value. By default, implemented as [Schedule::initial].
    fn next_on_task_panic(&self) -> Option<Duration> {
        self.initial()
    }
}
