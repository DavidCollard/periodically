use super::{SchedulerExt, TaskIdentifer};
use crate::{AsyncTask, Schedule, Task};
use std::{collections::HashMap, sync::Arc};
use tokio::task::{spawn_blocking, JoinHandle};

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
    fn add_sync_task<T: Send + 'static>(
        &mut self,
        task: impl crate::Task<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        task_identifier: TaskIdentifer,
    ) {
        let handle = self.runtime.spawn(sync_nanny(task, schedule));
        self.handles.insert(task_identifier, handle);
    }

    fn add_async_task<T>(
        &mut self,
        task: impl AsyncTask<T> + Send + Sync + 'static,
        schedule: impl Schedule<T> + Send + 'static,
        task_identifier: TaskIdentifer,
    ) where
        T: 'static + Send + Sync,
    {
        let handle = self.runtime.spawn(async_nanny(task, schedule));
        self.handles.insert(task_identifier, handle);
    }

    fn cancel_task(&mut self, task_identifier: super::TaskIdentifer) -> Option<()> {
        self.handles.remove(&task_identifier).map(|handle| {
            handle.abort();
        })
    }
}

async fn sync_nanny<T>(task: impl Task<T> + Sync + Send + 'static, schedule: impl Schedule<T>)
where
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
                next = match join_handle.await {
                    Ok(task_result) => schedule.next(task_result),
                    Err(err) => {
                        println!("Cannot join: {err:?}");
                        schedule.initial()
                    }
                };
            }
            None => return,
        }
    }
}

async fn async_nanny<T>(task: impl AsyncTask<T> + Send + Sync + 'static, schedule: impl Schedule<T>)
where
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
                next = match join_handle.await {
                    Ok(task_result) => schedule.next(task_result),
                    Err(err) => {
                        println!("Cannot join: {err:?}");
                        schedule.initial()
                    }
                };
            }
            None => return,
        }
    }
}
