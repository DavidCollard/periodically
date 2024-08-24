use crate::{schedule::Schedule, AsyncTask, Task};

cfg_feature! {
    "tokio",
    mod tokio_scheduler;
}

/// The core scheduler of `periodically`. Provides the interfaces for
///   scheduling individual tasks.
pub struct Scheduler {
    flavour: SchedulerFlavour,
    next_identifier: TaskIdentifier,
    in_use_ids: Vec<TaskIdentifier>,
}

enum SchedulerFlavour {
    #[cfg(feature = "tokio")]
    Tokio(tokio_scheduler::TokioScheduler),
}

impl Scheduler {
    fn from_flavour(flavour: impl Into<SchedulerFlavour>) -> Self {
        Self {
            flavour: flavour.into(),
            next_identifier: Default::default(),
            in_use_ids: Default::default(),
        }
    }

    /// Registers a synchronous [`Task`] with this scheduler.
    ///
    /// Returns a [`TaskIdentifier`], which is associated with this
    ///   specific scheduler. This identifier can be used to cancel
    ///   the task at a later time using [`cancel_task`][Scheduler::cancel_task].
    pub fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
    ) -> TaskIdentifier {
        let next_id = self.next_identifier;
        #[cfg(feature = "log")]
        log::info!(
            "Registering task [{}] with a TaskIdentifier of [{}].",
            std::any::type_name_of_val(&task),
            next_id
        );
        self.next_identifier = self.next_identifier.next();
        match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.add_sync_task(task, schedule, next_id),
        };
        next_id
    }

    /// Registers an asynchronous [`AsyncTask`] with this scheduler.
    ///
    /// Returns a [`TaskIdentifier`], which is associated with this
    ///   specific scheduler. This identifier can be used to cancel
    ///   the task at a later time using [`cancel_task`][Scheduler::cancel_task].
    pub fn add_async_task<T>(
        &mut self,
        task: impl AsyncTask<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
    ) -> TaskIdentifier
    where
        T: Send + 'static + Sync,
    {
        let next_id = self.next_identifier;
        #[cfg(feature = "log")]
        log::info!(
            "Registering task [{}] with a TaskIdentifier of [{}].",
            std::any::type_name_of_val(&task),
            next_id
        );
        self.next_identifier = self.next_identifier.next();
        match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.add_async_task(task, schedule, next_id),
        };
        next_id
    }

    #[allow(clippy::result_unit_err)]
    /// Stops a task from continuing to be scheduled. Running tasks may continue to run
    ///   until completion after being cancelled. See the documentation for the specific
    ///   flavour of Scheduler being used.
    ///
    /// Returns an `Ok(())` if the task was successfully marked for cancellation.
    /// Returns an `Err(())` if the task was not registered with the scheduler.
    pub fn cancel_task(&mut self, identifier: TaskIdentifier) -> Result<(), ()> {
        let res = match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.cancel_task(identifier),
        };
        res.ok_or(())
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        // relevant for when we only own a handle to an external runtime,
        // so we need to cancel the ongoing work in that runtime.
        while let Some(id) = self.in_use_ids.pop() {
            let _ = self.cancel_task(id);
        }
    }
}

/// Implemented by variants of [SchedulerFlavour].
trait SchedulerExt {
    /// See [Scheduler::add_sync_task].
    fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        identifier: TaskIdentifier,
    );

    /// See [Scheduler::add_async_task].
    fn add_async_task<T: Send + Sync + 'static>(
        &mut self,
        task: impl AsyncTask<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        identifier: TaskIdentifier,
    );

    /// See [Scheduler::cancel_task].
    fn cancel_task(&mut self, identifier: TaskIdentifier) -> Option<()>;
}

/// Task Identifiers are created by a [Scheduler] when registering a task, and
///   are used for performing modifications on a running task with a scheduler.
#[derive(Debug, Hash, Eq, PartialEq, Default, Clone, Copy)]
pub struct TaskIdentifier(usize);

impl std::fmt::Display for TaskIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("TaskIdentifier({})", self.0))
    }
}

impl TaskIdentifier {
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
