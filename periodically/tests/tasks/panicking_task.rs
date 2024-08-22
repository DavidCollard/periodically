use std::{
    future::ready,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use periodically::{AsyncTask, Task};

/// Simple task that panics every modulo executions
pub struct PanickingTask {
    counter: Arc<AtomicUsize>,
    modulo: usize,
}

impl PanickingTask {
    pub fn with_modulo(modulo: usize) -> Self {
        PanickingTask {
            modulo,
            counter: Default::default(),
        }
    }

    pub fn counter(&self) -> Arc<AtomicUsize> {
        self.counter.clone()
    }
}

impl Task<()> for PanickingTask {
    fn run(&self) {
        if self.counter.fetch_add(1, Ordering::Relaxed) % self.modulo == 0 {
            panic!("Task panicked!");
        }
    }
}

impl AsyncTask<()> for PanickingTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send {
        if self.counter.fetch_add(1, Ordering::Relaxed) % self.modulo == 0 {
            panic!("Task panicked!");
        }
        ready(())
    }
}
