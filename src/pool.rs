//! Low-level thread pool implementation.

use crate::error::PoolCreationError;

/// Implementation of a thread pool.
///
/// Instead of creation a thread for each action, this
pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if !(size > 0) {
            println!("Size is err, {}", size);
            return Err(PoolCreationError);
        }

        Ok(ThreadPool {})
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        func();
    }
}
