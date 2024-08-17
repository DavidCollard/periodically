use std::{
    future::ready,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use periodically::{IntervalSchedule, Scheduler, Task};
use tokio::runtime::Runtime;

#[test]
fn interval_scheduling_is_not_too_fast() {
    const TEST_DURATION: Duration = Duration::from_secs(1);
    const INTERVAL: Duration = Duration::from_millis(10);
    const EXPECTED_COUNT: usize = (TEST_DURATION.as_micros() / INTERVAL.as_micros()) as usize;

    let count = Arc::new(AtomicUsize::new(0));
    let task = SpyingTask {
        count: count.clone(),
    };

    let mut scheduler = Scheduler::tokio_scheduler(Runtime::new().unwrap());
    let id = scheduler.add_task(task, IntervalSchedule::every(INTERVAL));
    sleep(TEST_DURATION);
    scheduler.stop_task(id).unwrap();

    let real_count = count.load(Ordering::Relaxed);
    assert!(real_count <= EXPECTED_COUNT);
}

struct SpyingTask {
    count: Arc<AtomicUsize>,
}

impl Task<()> for SpyingTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send + 'static {
        self.count.fetch_add(1, Ordering::Relaxed);
        ready(())
    }
}
