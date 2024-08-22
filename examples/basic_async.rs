use periodically::{AsyncTask, IntervalSchedule, Scheduler};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::tokio_scheduler(runtime);

    let id = scheduler.add_async_task(
        MyTask::default(),
        IntervalSchedule::every(Duration::from_secs(1)),
    );

    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Should be able to read from stdin");
    println!("stopping tasks");
    scheduler
        .stop_task(id)
        .expect("Should not err for a known identifier");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Should be able to read from stdin");
    println!("Exiting");
}

#[derive(Default)]
struct MyTask {
    counter: AtomicUsize,
}

impl AsyncTask<()> for MyTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send {
        println!("MyTask is running");
        if self.counter.fetch_add(1, Ordering::Relaxed) % 5 == 0 {
            panic!("My task panicked!");
        }
        std::future::ready(())
    }
}
