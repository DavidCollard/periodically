use crate::{schedule::Schedule, Task};

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

    /// Adds a periodic task to the schedule. Can be cancelled using the returned identifier.
    pub fn add_task<T>(
        &mut self,
        task: impl Task<T> + 'static,
        schedule: impl Schedule<T> + 'static,
    ) -> TaskIdentifer
    where
        T: Send + 'static,
    {
        let next_id = self.next_identifier;
        self.next_identifier += 1;
        match &mut self.flavour {
            #[cfg(feature = "tokio")]
            SchedulerFlavour::Tokio(tok) => tok.add_task(task, schedule, next_id),
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
    fn add_task<T: Send + 'static>(
        &mut self,
        task: impl Task<T> + 'static,
        schedule: impl Schedule<T> + 'static,
        identifier: TaskIdentifer,
    );

    fn cancel_task(&mut self, identifier: TaskIdentifer) -> Option<()>;
}
