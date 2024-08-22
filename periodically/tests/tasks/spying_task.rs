use periodically::{AsyncTask, Task};
use std::{
    future::ready,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[derive(Default)]
pub struct SpyingTask {
    count: Arc<AtomicUsize>,
}

impl SpyingTask {
    pub fn counter(&self) -> Arc<AtomicUsize> {
        self.count.clone()
    }
}

impl Task<()> for SpyingTask {
    fn run(&self) -> () {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
}

impl AsyncTask<()> for SpyingTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send {
        self.count.fetch_add(1, Ordering::Relaxed);
        ready(())
    }
}
