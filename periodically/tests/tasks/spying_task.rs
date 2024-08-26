use periodically::{AsyncTask, Task};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
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

impl Task for SpyingTask {
    fn run(&self) -> () {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
}

impl AsyncTask for SpyingTask {
    async fn run(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
}
