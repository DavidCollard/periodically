use std::time::Duration;

use super::Schedule;

pub struct PeriodicSchedule {
    delay: Option<Duration>,
    period: Duration,
}

impl PeriodicSchedule {
    pub fn every(period: Duration) -> Self {
        Self {
            delay: None,
            period,
        }
    }
    pub fn with_initial_delay(period: Duration, delay: Duration) -> Self {
        Self {
            delay: Some(delay),
            period,
        }
    }
}

impl<T> Schedule<T> for PeriodicSchedule {
    fn next(&self, _: T) -> Option<Duration> {
        Some(self.period)
    }

    fn inital(&self) -> Option<Duration> {
        self.delay.or(Some(self.period))
    }
}
