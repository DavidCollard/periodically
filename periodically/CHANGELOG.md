## v0.2.0

First release of `Periodically`.

### Added

* `Scheduler`
  * Allows registering/canceling both synchronous `Task`s and asynchronous `AsyncTask`s.
  * Added the tokio-based scheduler under the default `tokio` feature.
* `Schedule` trait
  * OneShotSchedule.
  * IntervalSchedule.
  * BackoffSchedule under the `backoff` feature. 
  * CronSchedule under the `cron` feature.
* `Task` trait
  * Defines tasks that run in synchronous threads.
* `AsyncTask` trait
  * Defines tasks that run in an asynchronous runtime.


## v0.1.0

Initial repo/crates.io setup.