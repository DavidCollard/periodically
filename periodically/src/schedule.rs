use std::time::Duration;

mod interval;
pub use interval::IntervalSchedule;

mod oneshot;
pub use oneshot::OneShot;

#[cfg(feature = "cron")]
mod cron;
#[cfg(feature = "cron")]
pub use cron::CronSchedule;

/// Defines an execution schedule for a given task.
/// All duration values returned define how long until
/// the task should start execution.
pub trait Schedule<T>: Send {
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
}
