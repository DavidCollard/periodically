use std::collections::HashMap;

use tokio::task::JoinHandle;

use crate::{Schedule, Task};

use super::{SchedulerExt, TaskIdentifer};

pub struct TokioScheduler {
    runtime: tokio::runtime::Runtime,
    handles: HashMap<TaskIdentifer, JoinHandle<()>>,
}

impl TokioScheduler {
    pub fn new(runtime: tokio::runtime::Runtime) -> Self {
        Self {
            runtime,
            handles: HashMap::new(),
        }
    }
}

impl SchedulerExt for TokioScheduler {
    fn add_task<T>(
        &mut self,
        task: impl Task<T> + 'static,
        schedule: impl Schedule<T> + 'static,
        task_identifier: TaskIdentifer,
    ) where
        T: 'static + Send,
    {
        let handle = self.runtime.spawn(nanny(task, schedule));
        self.handles.insert(task_identifier, handle);
    }

    fn cancel_task(&mut self, task_identifier: super::TaskIdentifer) -> Option<()> {
        self.handles.remove(&task_identifier).map(|handle| {
            handle.abort();
        })
    }
}

async fn nanny<T: Send + 'static>(
    task: impl Task<T> + 'static,
    schedule: impl Schedule<T> + 'static,
) {
    let mut next = schedule.inital();
    loop {
        match next {
            Some(duration) => {
                tokio::time::sleep(duration).await;
                let fut = task.run();
                let join_handle = tokio::spawn(fut).await;
                let task_output = match join_handle {
                    Ok(task_result) => task_result,
                    // FIXME
                    Err(err) => panic!("Cannot join: {err:?}"),
                };
                next = schedule.next(task_output)
            }
            None => return,
        }
    }
}
