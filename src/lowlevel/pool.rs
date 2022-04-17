//! Low-level thread pool implementation.

use crate::error::PoolCreationError;
use crate::lowlevel::worker::{Job, Worker};

use std::sync::{mpsc, Arc, Mutex};

pub(crate) enum Message {
    NewJob(Job),
    Terminate,
}

/// Implementation of a thread pool.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
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

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
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
