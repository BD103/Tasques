//! Provides the [Worker] struct.

use crate::pool::Message;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A boxed closure that supports threads.
pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

/// A structure for consuming functions and executing them on a separate thread.
pub(crate) struct Worker {
    #[allow(dead_code)]
    pub(crate) id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new [Worker].
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
