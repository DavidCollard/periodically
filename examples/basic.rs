use std::time::Duration;

use periodically::{periodic::PeriodicSchedule, Scheduler, Task};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::tokio_scheduler(runtime);

    let id = scheduler.add_task(MyTask, PeriodicSchedule::every(Duration::from_secs(1)));

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

struct MyTask;

impl Task<()> for MyTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send + 'static {
        println!("MyTask is running");
        std::future::ready(())
    }
}
