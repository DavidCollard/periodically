use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
    vec,
};

use periodically::{Schedule, Scheduler, Task};
use tokio::runtime::Runtime;

struct Sched {
    count: Arc<AtomicUsize>,
    captured: Arc<Mutex<Vec<usize>>>,
}

impl Schedule<Result<usize, ()>> for Sched {
    fn next(&self, task_output: Result<usize, ()>) -> Option<std::time::Duration> {
        self.count
            .fetch_add(1, std::sync::atomic::Ordering::Release);
        match task_output {
            Ok(v) => {
                self.captured.lock().unwrap().push(v);
                Some(Duration::from_secs(0))
            }
            Err(_) => None,
        }
    }
}

struct FalibleTask {
    remaining_success: Arc<AtomicUsize>,
}

impl Task<Result<usize, ()>> for FalibleTask {
    fn run(&self) -> Result<usize, ()> {
        if self.remaining_success.load(Ordering::Relaxed) == 0 {
            Err(())
        } else {
            Ok(self.remaining_success.fetch_sub(1, Ordering::Relaxed))
        }
    }
}

#[test]
fn task_output_is_piped_to_schedule() {
    let sched_counter = Arc::new(AtomicUsize::default());
    let captured = Arc::new(Mutex::new(Vec::default()));
    let schedule = Sched {
        count: sched_counter.clone(),
        captured: captured.clone(),
    };

    let task = FalibleTask {
        remaining_success: Arc::new(AtomicUsize::new(5)),
    };

    let mut scheduler = Scheduler::tokio_scheduler(Runtime::new().unwrap());
    scheduler.add_sync_task(task, schedule);

    std::thread::sleep(Duration::from_millis(100));

    assert_eq!(sched_counter.load(Ordering::Acquire), 6);
    assert_eq!(*captured.lock().unwrap(), vec![5, 4, 3, 2, 1]);
}
