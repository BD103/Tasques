//! Various errors used by the library.

use std::fmt;

/// Raised when there is an error creating a [crate::ThreadPool].
///
/// This will only be raised when calling [crate::ThreadPool::new].
///
/// ```should_panic
/// use tasques_lowlevel::ThreadPool;
///
/// // Panics with PoolCreationError
/// let pool = ThreadPool::new(0).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error creating thread pool")
    }
}
