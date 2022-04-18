//! Provides a thread pool implementation that backs the TasqueManager.

mod pool;
mod worker;

pub use pool::ThreadPool;
pub(crate) use worker::Job;
