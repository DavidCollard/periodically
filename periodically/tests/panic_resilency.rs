pub mod tasks;

use periodically::{IntervalSchedule, Scheduler};
use std::{sync::atomic::Ordering, thread::sleep, time::Duration};
use tasks::PanickingTask;
use tokio::runtime::Runtime;

#[test]
fn panicking_sync_task_keeps_running() {
    let task = PanickingTask::with_modulo(2);
    let schedule = IntervalSchedule::every(Duration::from_millis(10));
    let mut scheduler = Scheduler::tokio_scheduler(Runtime::new().unwrap());
    let counter = task.counter();
    scheduler.add_sync_task(task, schedule);

    sleep(Duration::from_millis(100));
    assert!(counter.load(Ordering::Acquire) >= 5);
}

#[test]
fn panicking_async_task_keeps_running() {
    let task = PanickingTask::with_modulo(2);
    let schedule = IntervalSchedule::every(Duration::from_millis(10));
    let mut scheduler = Scheduler::tokio_scheduler(Runtime::new().unwrap());
    let counter = task.counter();
    scheduler.add_async_task(task, schedule);

    sleep(Duration::from_millis(100));
    assert!(counter.load(Ordering::Acquire) >= 5);
}
