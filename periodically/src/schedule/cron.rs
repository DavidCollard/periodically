use super::Schedule;
use std::{str::FromStr, time::Duration};

pub struct CronSchedule {
    cron: cron::Schedule,
}

/// Schedule events according to a cron schedule.
impl CronSchedule {
    /// Creates a [CronSchedule] that always returns `period`.
    ///
    /// ```
    /// use periodically::{Schedule, CronSchedule};
    /// use std::time::Duration;
    ///
    /// let expression = "0 * * * * *";
    /// let periodic = CronSchedule::from_cron_str(expression).unwrap();
    /// # let periodic = Box::new(periodic) as Box<dyn Schedule<()>>;
    /// assert!(periodic.initial().unwrap() < Duration::from_secs(60));
    /// assert!(periodic.next(()).unwrap() < Duration::from_secs(60));
    /// ```
    pub fn from_cron_str(cron_str: impl AsRef<str>) -> Result<Self, cron::error::Error> {
        let cron = cron::Schedule::from_str(cron_str.as_ref())?;
        Ok(Self { cron })
    }
    /// Creates a [CronSchedule] that always returns `period`.
    ///
    /// ```
    /// use periodically::{Schedule, CronSchedule};
    /// use std::str::FromStr;
    /// use std::time::Duration;
    ///
    /// let expression = "0 * * * * *";
    /// let cron_sched = cron::Schedule::from_str(expression).unwrap();
    /// let periodic = CronSchedule::from_cron_schedule(cron_sched);
    /// # let periodic = Box::new(periodic) as Box<dyn Schedule<()>>;
    /// assert!(periodic.initial() < Some(Duration::from_secs(60)));
    /// assert!(periodic.next(()) < Some(Duration::from_secs(60)));
    /// ```

    pub fn from_cron_schedule(cron: cron::Schedule) -> Self {
        Self { cron }
    }

    fn calculate_next(&self) -> Option<Duration> {
        let dt = self.cron.upcoming(chrono::Utc).next()?;
        let now = chrono::Utc::now();
        let delta = dt.signed_duration_since(&now);
        Some(delta.to_std().unwrap_or(Duration::from_secs(0)))
    }
}

impl<T> Schedule<T> for CronSchedule {
    fn initial(&self) -> Option<Duration> {
        self.calculate_next()
    }

    fn next(&self, _: T) -> Option<Duration> {
        self.calculate_next()
    }
}
