mod schedule;
mod schedulers;

use std::future::Future;

pub use schedule::*;
pub use schedulers::Scheduler;

pub trait Task<T> {
    fn run(&self) -> T;
}

pub trait AsyncTask<T> {
    fn run(&self) -> impl Future<Output = T> + Send;
}
