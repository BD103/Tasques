//! Low-level thread pool implementation.

use crate::error::PoolCreationError;
use crate::lowlevel::worker::{Job, Worker};

use std::sync::{mpsc, Arc, Mutex};

/// Messages sent from the pool to individual threads.
pub(crate) enum Message {
    NewJob(Job),
    Terminate,
}

/// Implementation of a thread pool.
///
/// This is the main backend behind the TasqueManager.
/// It prevents DOS attacks by having a set number of threads (instead of creating a new one per-task).
///
/// ```
/// use tasques::lowlevel::ThreadPool;
/// use std::thread;
///
/// let pool = ThreadPool::new(4);
///
/// for _ in 0..10 {
///     pool.execute(|| println!("Hello, world!"));
/// }
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Creates a new [ThreadPool] with the given size.
    ///
    /// # Errors
    ///
    /// Returns a [PoolCreationError] if the size is less than or equal to 0.
    /// (You need to have at least 1 thread!)
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        // Need at least 1 thread
        if !(size > 0) {
            println!("Size is err, {}", size);
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    /// Adds the given function to the queue to be executed.
    ///
    /// If there is an open thread, it will consume the function and execute it.
    /// If there is not an open thread, this function will have to wait for one to finish.
    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Waits for all threads to finish executing before destructing.
    fn drop(&mut self) {
        // Tell all threads to finish
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Join all workers into main thread
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
