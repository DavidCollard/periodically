# Periodically


`Periodically` provides a robust and ergonomic library for writing periodic or scheduled jobs in Rust.

[Documentation](https://docs.rs/periodically/0.1.0/periodically/)

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/periodically.svg
[crates-url]: https://crates.io/crates/periodically
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/DavidCollard/periodically/blob/master/LICENSE

## Quick example

It is easy to write a task definition:

```rust
struct MyTask;

impl Task<()> for MyTask {
    fn run(&self) -> impl std::future::Future<Output = ()> + Send + 'static {
        println!("MyTask is running");
        std::future::ready(())
    }
}
```

and then plug it into a scheduler using `Periodically`'s provided schedules:

```rust
fn main() {
    /// Create a scheduler using a tokio runtime, or provide your own
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut scheduler = Scheduler::tokio_scheduler(runtime);

    /// the task begins running once a second
    let id = scheduler.add_task(MyTask, IntervalSchedule::every(Duration::from_secs(1)));

    /// the task stops running
    scheduler
        .stop_task(id)
        .expect("Should not err for a known identifier");
}

```

or, define your own schedule if needed:

```rust
pub struct OneShot {
    delay: Duration,
}

impl<T> Schedule<T> for OneShot {
    fn initial(&self) -> Option<std::time::Duration> {
        Some(self.delay)
    }

    fn next(&self, _: T) -> Option<std::time::Duration> {
        None
    }
}
```

More examples can be found [here](https://github.com/DavidCollard/periodically/tree/main/examples).
