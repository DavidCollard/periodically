use std::time::Duration;

use super::Schedule;

/// Schedules a single ('one shot') execution after
/// a specified delay of time.
///
/// ```
/// use periodically::{Schedule, OneShot};
/// use std::time::Duration;
///
/// let one_shot = OneShot::after(Duration::from_secs(1));
/// # let one_shot = Box::new(one_shot) as Box<dyn Schedule<()>>;
/// assert_eq!(one_shot.initial(), Some(Duration::from_secs(1)));
/// assert_eq!(one_shot.next(()), None);
/// ```
pub struct OneShot {
    delay: Duration,
}

impl OneShot {
    pub fn after(delay: Duration) -> Self {
        Self { delay }
    }
}

impl<T> Schedule<T> for OneShot {
    fn initial(&self) -> Option<std::time::Duration> {
        Some(self.delay)
    }

    fn next(&self, _: T) -> Option<std::time::Duration> {
        None
    }

    fn next_on_task_panic(&self) -> Option<Duration> {
        None
    }
}
