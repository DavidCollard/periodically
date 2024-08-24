pub mod tasks;

use periodically::{IntervalSchedule, Scheduler};
use std::{sync::atomic::Ordering, thread::sleep, time::Duration};
use tasks::SpyingTask;
use tokio::runtime::Runtime;

#[test]
fn interval_scheduling_is_not_too_fast() {
    const TEST_DURATION: Duration = Duration::from_secs(1);
    const INTERVAL: Duration = Duration::from_millis(10);
    const EXPECTED_COUNT: usize = (TEST_DURATION.as_micros() / INTERVAL.as_micros()) as usize;

    let task = SpyingTask::default();
    let counter = task.counter();

    let mut scheduler = Scheduler::tokio_scheduler(Runtime::new().unwrap());
    let id = scheduler.add_async_task(task, IntervalSchedule::every(INTERVAL));
    sleep(TEST_DURATION);
    scheduler.cancel_task(id).unwrap();

    let real_count = counter.load(Ordering::Relaxed);
    assert!(real_count <= EXPECTED_COUNT);
}
