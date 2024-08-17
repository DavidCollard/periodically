mod schedule;
mod schedulers;

use std::future::Future;

pub use schedule::*;
pub use schedulers::Scheduler;

pub trait Task<T>: Send
where
    T: Send + 'static,
{
    fn run(&self) -> impl Future<Output = T> + Send + 'static;
}
