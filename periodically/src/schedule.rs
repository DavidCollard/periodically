pub mod periodic;

use std::time::Duration;

pub trait Schedule<T>: Send {
    /// How long to wait for the initial schedule. Exists since
    /// the task_output is not available initially. Can also be used
    /// to control initial task execution delays.
    fn inital(&self) -> Option<Duration>;
    /// Returns the time until this task should be scheduled again.
    /// None indicates that this task should never run again.
    /// `task_output` is the return value of the task.
    fn next(&self, task_output: T) -> Option<Duration>;
}
