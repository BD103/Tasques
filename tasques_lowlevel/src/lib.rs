//! Provides a thread pool implementation that backs the TasqueManager.

mod pool;
mod worker;

pub mod errors;

pub use pool::ThreadPool;
