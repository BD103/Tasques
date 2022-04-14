use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

pub(crate) struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {loop {

        }});

        Worker { id, thread }
    }
}
