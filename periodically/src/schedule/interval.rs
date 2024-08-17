use std::time::Duration;

use super::Schedule;

/// Schedules a simple interval execution.
///
/// **Danger: Does not account for clock drift when executing a task**.
/// eg; If the interval is 10s, and the task takes 3s to execute, the next
/// task will not be scheduled until 13s after the first execution started.
pub struct IntervalSchedule {
    delay: Option<Duration>,
    interval: Duration,
}

impl IntervalSchedule {
    /// Creates a [Schedule] that always returns `interval`.
    ///
    /// ```
    /// use periodically::{Schedule, IntervalSchedule};
    /// use std::time::Duration;
    ///
    /// let interval = IntervalSchedule::every(Duration::from_secs(1));
    /// # let interval = Box::new(interval) as Box<dyn Schedule<()>>;
    /// assert_eq!(interval.initial(), Some(Duration::from_secs(1)));
    /// assert_eq!(interval.next(()), Some(Duration::from_secs(1)));
    /// ```
    pub fn every(interval: Duration) -> Self {
        Self {
            delay: None,
            interval,
        }
    }

    /// Creates a [Schedule] that will first return after a delay, and then always returns `interval`.
    ///
    /// ```
    /// use periodically::{Schedule, IntervalSchedule};
    /// use std::time::Duration;
    ///
    /// let interval = IntervalSchedule::with_initial_delay(Duration::from_secs(2), Duration::from_secs(1));
    /// # let interval = Box::new(interval) as Box<dyn Schedule<()>>;
    /// assert_eq!(interval.initial(), Some(Duration::from_secs(1)));
    /// assert_eq!(interval.next(()), Some(Duration::from_secs(2)));
    /// ```
    pub fn with_initial_delay(interval: Duration, delay: Duration) -> Self {
        Self {
            delay: Some(delay),
            interval,
        }
    }
}

impl<T> Schedule<T> for IntervalSchedule {
    fn next(&self, _: T) -> Option<Duration> {
        Some(self.interval)
    }

    fn initial(&self) -> Option<Duration> {
        self.delay.or(Some(self.interval))
    }
}
