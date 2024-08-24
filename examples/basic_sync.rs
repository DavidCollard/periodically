use env_logger::Env;
use log::info;
use periodically::{IntervalSchedule, Scheduler, Task};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

/// This demonstrates a simple example app that runs a sync periodic job that panics once every
///   5 seconds, and shows how the scheduler can stop tasks.
///
/// Run with `cargo r --example basic-sync`
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::tokio_scheduler(runtime);

    let id = scheduler.add_sync_task(
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

impl Task for MyTask {
    fn run(&self) {
        info!("MyTask is running");
        if self.counter.fetch_add(1, Ordering::Relaxed) % 5 == 0 {
            panic!("My task panicked!");
        }
    }
}
