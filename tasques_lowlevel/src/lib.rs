//! Provides a thread pool implementation that backs the TasqueManager.

mod pool;
mod worker;
mod errors;

pub use pool::ThreadPool;
