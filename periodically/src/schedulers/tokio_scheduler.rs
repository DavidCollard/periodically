use super::{Scheduler, SchedulerExt, SchedulerFlavour, TaskIdentifier};
use crate::{AsyncTask, Schedule, Task};
use std::{collections::HashMap, future::Future, sync::Arc, time::Duration};
use tokio::task::{spawn_blocking, JoinError, JoinHandle};

/// Constructors for [Tokio][tokio]-based schedulers.
///
/// Tokio-based schedulers spawns a future to schedule each registered task. [AsyncTasks][AsyncTask] run on
/// the runtime's executor, and [Tasks][Task] run on the blocking pool.
impl Scheduler {
    /// Creates a tokio-based scheduler with an owned runtime. The provided runtime will be used to
    ///   schedule and run all tasks. This is best used when working with periodic tasks that require
    ///   a specifically tuned runtime, or when there is a need to run multiple seperate runtimes in
    ///   the same process.
    ///
    /// This Scheduler will own the given runtime, and perform a best effort shutdown of it
    ///   when dropped.
    pub fn tokio_scheduler(runtime: tokio::runtime::Runtime) -> Self {
        Scheduler::from_flavour(TokioScheduler::new_from_runtime(runtime))
    }

    /// Creates a tokio-based scheduler using a runtime handle. The associated runtime will be
    ///   used to schedule and run all tasks. If the associated runtime is stopped (eg; [shutdown_background][tokio::runtime::Runtime::shutdown_background]),
    ///   this scheduler will cease to work.
    ///
    /// This Scheduler will perform a best effort stop of all running tasks on the given
    ///   runtime when dropped.
    pub fn tokio_scheduler_with_handle(handle: tokio::runtime::Handle) -> Self {
        Scheduler::from_flavour(TokioScheduler::new_from_handle(handle))
    }

    /// Creates a tokio-based scheduler using the current runtime context. The associated runtime will be
    ///   used to schedule and run all tasks. If the associated runtime is stopped (eg; [shutdown_background][tokio::runtime::Runtime::shutdown_background]),
    ///   this scheduler will cease to work.
    ///
    /// This Scheduler will perform a best effort stop of all running tasks on the given
    ///   runtime when dropped.
    pub fn tokio_scheduler_with_current() -> Self {
        let handle = tokio::runtime::Handle::current();
        Scheduler::from_flavour(TokioScheduler::new_from_handle(handle))
    }
}

pub struct TokioScheduler {
    runtime_handle: RuntimeHandle,
    task_handles: HashMap<TaskIdentifier, JoinHandle<()>>,
}

impl Into<SchedulerFlavour> for TokioScheduler {
    fn into(self) -> SchedulerFlavour {
        SchedulerFlavour::Tokio(self)
    }
}

impl TokioScheduler {
    fn new_from_runtime(runtime: tokio::runtime::Runtime) -> Self {
        Self {
            runtime_handle: RuntimeHandle::Runtime(runtime),
            task_handles: HashMap::new(),
        }
    }

    fn new_from_handle(handle: tokio::runtime::Handle) -> Self {
        Self {
            runtime_handle: RuntimeHandle::Handle(handle),
            task_handles: HashMap::new(),
        }
    }
}

impl SchedulerExt for TokioScheduler {
    fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl crate::Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        task_identifier: TaskIdentifier,
    ) {
        let handle = self
            .runtime_handle
            .spawn(sync_nanny(task, schedule, task_identifier));
        self.task_handles.insert(task_identifier, handle);
    }

    fn add_async_task<T>(
        &mut self,
        task: impl AsyncTask<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        task_identifier: TaskIdentifier,
    ) where
        T: 'static + Send + Sync,
    {
        let handle = self
            .runtime_handle
            .spawn(async_nanny(task, schedule, task_identifier));
        self.task_handles.insert(task_identifier, handle);
    }

    fn cancel_task(&mut self, task_identifier: TaskIdentifier) -> Option<()> {
        self.task_handles.remove(&task_identifier).map(|handle| {
            handle.abort();
        })
    }
}

async fn sync_nanny<T>(
    task: impl Task<T> + Sync + Send + 'static,
    schedule: impl Schedule<T>,
    task_identifier: TaskIdentifier,
) where
    T: Send + 'static,
{
    let mut next = schedule.initial();
    let task = Arc::new(task);
    loop {
        match next {
            Some(duration) => {
                tokio::time::sleep(duration).await;
                let task = task.clone();
                let join_handle = spawn_blocking(move || task.run());
                next = handle_task_result(join_handle.await, &schedule, task_identifier);
            }
            None => return,
        }
    }
}

async fn async_nanny<T>(
    task: impl AsyncTask<T> + Send + Sync + 'static,
    schedule: impl Schedule<T>,
    task_identifier: TaskIdentifier,
) where
    T: Send + Sync + 'static,
{
    let mut next = schedule.initial();
    let task = Arc::new(task);
    loop {
        match next {
            Some(duration) => {
                tokio::time::sleep(duration).await;
                let task = task.clone();
                let join_handle = tokio::spawn(async move { task.run().await });
                next = handle_task_result(join_handle.await, &schedule, task_identifier);
            }
            None => return,
        }
    }
}

fn handle_task_result<T>(
    join_handle: Result<T, JoinError>,
    schedule: &impl Schedule<T>,
    task_identifier: TaskIdentifier,
) -> Option<Duration> {
    let next = match join_handle {
        Ok(task_result) => schedule.next(task_result),
        Err(err) => {
            #[cfg(feature = "log")]
            log::error!("Cannot join: {err:?}");
            schedule.next_on_task_panic()
        }
    };
    #[cfg(feature = "log")]
    log::trace!("Next event for task [{task_identifier}] will be in [{next:?}]");
    next
}

enum RuntimeHandle {
    Runtime(tokio::runtime::Runtime),
    Handle(tokio::runtime::Handle),
}

impl RuntimeHandle {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match self {
            RuntimeHandle::Runtime(r) => r.spawn(future),
            RuntimeHandle::Handle(h) => h.spawn(future),
        }
    }
}
