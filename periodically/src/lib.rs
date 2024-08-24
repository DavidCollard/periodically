//! [Periodically][crate] provides a robust and ergonomic library for writing periodic or scheduled jobs in Rust.
//!
//! ### The Scheduler
//!
//! The [Scheduler] is the core of periodically. It allows for tasks to be registered and scheduled.
//! It supports running synchronous [`Tasks`][Task] and asynchronous [`AsyncTasks`][AsyncTask] in the
//! same scheduling runtime.
//!
//! The core design of the scheduler is based on the idea of decoupling an executable task from it's schedule.
//!
//! For example, consider how this naive example of a periodic job requires so much boilerplate:
//!
//! ```
//! # use std::time::Duration;
//! # use std::sync::{Arc, atomic::AtomicBool};
//! # let should_terminate = true;
//! std::thread::spawn(move || {
//!   loop {
//!     // ..
//!     // do the actual work
//!     // ..
//!     if should_terminate {
//!         break;
//!     }
//!     std::thread::sleep(Duration::from_millis(100));
//!   }
//! });
//! ```
//!
//! This example also fails to account for edge cases like panic resilience.
//!
//! Whereas with `periodically`, the task can easily without worrying about the schedule,
//!   and then the schedule can be injected later. This also enables the re-use of schedules
//!   if there is common specialized logic for your application.
//!
//! ```
//! # use periodically::Task;
//! # use periodically::Scheduler;
//! # use periodically::IntervalSchedule;
//! # use std::time::Duration;
//! # struct MyTask;
//! # let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
//! impl Task for MyTask {
//!   fn run(&self) {
//!     // ..
//!     // do the actual work
//!     // ..
//!   }
//! }
//! let mut scheduler = Scheduler::tokio_scheduler(runtime);
//! scheduler.add_sync_task(MyTask {}, IntervalSchedule::every(Duration::from_secs(100)));
//! ```
//!
//! [`Scheduler`] executes tasks in a serial fashion. aka; for every time a task is registered,
//!   there is only ever at most one execution of that task.
//!
//! ### Tasks
//!
//! There are 2 types of task. The synchronous [`Task`], and it's async counterpart [`AsyncTask`].
//!   Their interfaces are very similar, with the major difference being that [`AsyncTask`] returns a future that is [`Send`].
//!
//! Both types of task can be scheduled on the same [`Scheduler`], via [`add_sync_task`][Scheduler::add_sync_task]
//!   and [`add_async_task`][Scheduler::add_async_task] respectively.
//!
//! ### Schedules
//!
//! The primary function of a [`Schedule`] is to consume context from a task execution, and decide when the next time that task will be executed.
//!
//! There are 3 control knobs of a [`Schedule`]:
//! 1. [`initial`][Schedule::initial] - this is called the very first time a task is being scheduled for execution.
//! 2. [`next`][Schedule::next] - this is called with the output of the last task execution.
//! 3. [`next_on_task_panic`][Schedule::next_on_task_panic] - this is called when the last task execution panicked.
//!
//! By using these knobs, and the internal state of the  `impl Schedule`, there is a lot of flexibility in how dynamic schedulers can be built.
//!   Additionally, since [`next`][Schedule::next] takes the output of the last task execution, an `impl Schedule` provides a way to egress
//!   execution data via mechanims like mpsc channels if desired.
//!
//! ### Features
//!
//! By default, the only features enabled are `tokio` and `log`
//!
//! * `full`: Enables all features
//! * `tokio`: Enables the tokio-based scheduler. As of 0.2.0, this is the only scheduler, and essentially required.
//! * `log`: Enables an intergration with the [`log`] crate in the [`Scheduler`]. Helps provide debug information when dealing with problematic tasks.
//! * `backoff`: Adds a built-in [`Schedule`] named [`BackoffSchedule`] which uses the external [`backoff`] crate.
//! * `cron`: Adds a built-in [`Schedule`] named [`CronSchedule`] which uses the external [`cron`] crate.

#![cfg_attr(docsrs, feature(doc_cfg))]

/// Conditionally compiles a list of items under `feature_name`
/// and annotates the docs of pub items with "Available on crate feature `feature_name` only.".
///
/// eg;
///
/// ```compile_fail
/// cfg_feature! {
///    "my-feature",
///    pub mod my_feature;
/// }
/// ```
///
/// Transforms into
///
/// ```compile_fail
/// # asdf
/// #[cfg(feature = "my-feature")]
/// #[cfg_attr(docsrs, doc(cfg(feature = "my-feature")))]
/// pub mod my_feature;
/// ```
macro_rules! cfg_feature {
    ($feature_name:literal, $($item:item)*) => {
        $(
            #[cfg(feature = $feature_name)]
            #[cfg_attr(docsrs, doc(cfg(feature = $feature_name)))]
            $item
        )*
    }
}

mod schedule;
mod schedulers;

use std::future::Future;

pub use schedule::*;
pub use schedulers::Scheduler;
pub use schedulers::TaskIdentifier;

/// Defines a task that can run in an synchronous runtime.
pub trait Task<T = ()> {
    /// Executes the task.
    fn run(&self) -> T;
}

/// Defines a task that can run in an asynchronous runtime.
pub trait AsyncTask<T = ()> {
    /// Executes the task.
    fn run(&self) -> impl Future<Output = T> + Send;
}
