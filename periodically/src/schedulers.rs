use crate::{schedule::Schedule, AsyncTask, Task};

#[cfg(feature = "tokio")]
mod tokio_scheduler;

pub struct Scheduler {
    flavour: SchedulerFlavour,
    next_identifier: TaskIdentifer,
}

enum SchedulerFlavour {
    #[cfg(feature = "tokio")]
    Tokio(tokio_scheduler::TokioScheduler),
}

type TaskIdentifer = usize;

impl Scheduler {
    #[cfg(feature = "tokio")]
    pub fn tokio_scheduler(runtime: tokio::runtime::Runtime) -> Self {
        Self {
            flavour: SchedulerFlavour::Tokio(tokio_scheduler::TokioScheduler::new(runtime)),
            next_identifier: Default::default(),
        }
    }

    pub fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
    ) -> TaskIdentifer {
        let next_id = self.next_identifier;
        self.next_identifier += 1;
        match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.add_sync_task(task, schedule, next_id),
        };
        next_id
    }

    /// Adds a periodic task to the schedule. Can be cancelled using the returned identifier.
    pub fn add_async_task<T>(
        &mut self,
        task: impl AsyncTask<T> + Sync + Send + 'static,
        schedule: impl Schedule<T> + Send + 'static,
    ) -> TaskIdentifer
    where
        T: Send + 'static + Sync,
    {
        let next_id = self.next_identifier;
        self.next_identifier += 1;
        match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.add_async_task(task, schedule, next_id),
        };
        next_id
    }

    #[allow(clippy::result_unit_err)]
    /// Stops a peridic task from being scheduled.
    pub fn stop_task(&mut self, identifier: TaskIdentifer) -> Result<(), ()> {
        let res = match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.cancel_task(identifier),
        };
        res.ok_or(())
    }
}

trait SchedulerExt {
    fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        identifier: TaskIdentifer,
    );

    fn add_async_task<T: Send + Sync + 'static>(
        &mut self,
        task: impl AsyncTask<T> + Sync + Send + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        identifier: TaskIdentifer,
    );

    fn cancel_task(&mut self, identifier: TaskIdentifer) -> Option<()>;
}
