use std::cell::RefCell;

use backoff::backoff::Backoff;

use super::Schedule;

///
pub struct BackoffSchedule<B> {
    backoff: RefCell<B>,
}

impl<B: Backoff> BackoffSchedule<B> {
    /// Creates a [BackoffSchedule] using the given backoff policy.
    ///
    /// ```
    /// use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
    /// use periodically::{Schedule, BackoffSchedule};
    /// use std::time::Duration;
    ///
    /// let backoff: ExponentialBackoff = ExponentialBackoffBuilder::new().build();
    /// let backoff_schedule = BackoffSchedule::from_backoff(backoff);
    /// # let backoff_schedule = Box::new(backoff_schedule) as Box<dyn Schedule<Result<(), ()>>>;
    /// assert_eq!(backoff_schedule.initial(), Some(Duration::from_secs(0)));
    /// assert!(backoff_schedule.next(Err(())).unwrap() < Duration::from_secs(1));
    /// ```
    pub fn from_backoff(backoff: B) -> Self {
        Self {
            backoff: RefCell::new(backoff),
        }
    }
}

impl<T, E, B: Backoff + Send> Schedule<Result<T, E>> for BackoffSchedule<B> {
    fn next(&self, task_output: Result<T, E>) -> Option<std::time::Duration> {
        let mut backoff = self.backoff.borrow_mut();
        if let Ok(_) = task_output {
            backoff.reset()
        }
        backoff.next_backoff()
    }
}
