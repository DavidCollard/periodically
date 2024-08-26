use env_logger::Env;
use log::info;
use periodically::{AsyncTask, IntervalSchedule, Scheduler};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

/// This demonstrates a simple example app that runs a async periodic job using tokio::main, and panics
///   once every 5 seconds. Also shows how droping the scheduler stops tasks.
///
/// Run with `cargo r --example basic-async`
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut scheduler = Scheduler::tokio_scheduler_with_current();

    let id = scheduler.add_async_task(
        MyTask::default(),
        IntervalSchedule::every(Duration::from_secs(1)),
    );

    info!("Starting tasks, press enter to stop them.");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Should be able to read from stdin");
    info!("Stopping tasks. Press enter to exit.");
    scheduler
        .cancel_task(id)
        .expect("Should not err for a known identifier");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Should be able to read from stdin");
    info!("Exiting");
}

#[derive(Default)]
struct MyTask {
    counter: AtomicUsize,
}

impl AsyncTask for MyTask {
    async fn run(&self) {
        info!("MyTask is running");
        if self.counter.fetch_add(1, Ordering::Relaxed) % 5 == 0 {
            panic!("My task panicked!");
        }
    }
}
