//! Example of using the [ThreadPool](crate::lowlevel::ThreadPool).
//!
//! This program creates 10 functions to be executed by 4 threads.
//! Each function waits 1 second, then prints the function number and elapsed time.
//!
//! Example output looks like the following:
//!
//! ```shell
//! Task 1 finished in 1.0027556 seconds
//! Task 3 finished in 1.0026968 seconds
//! Task 2 finished in 1.0027252 seconds
//! Task 0 finished in 1.0028104 seconds
//! Task 7 finished in 2.0190709 seconds
//! Task 6 finished in 2.0191739 seconds
//! Task 4 finished in 2.0192463 seconds
//! Task 5 finished in 2.019192 seconds
//! Task 9 finished in 3.0309925 seconds
//! Task 8 finished in 3.0310125 seconds
//! ```
//!
//! Notice how only 4 functions complete in the same second.

use tasques::lowlevel::ThreadPool;

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Create thread pool with 4 threads
    let pool = ThreadPool::new(4).unwrap();
    // Keep track of time
    let now = Instant::now();

    // Create 10 tasks to be executed asynchronously
    for i in 0..10 {
        // Add functions to the queue
        pool.execute(move || {
            thread::sleep(Duration::from_secs(1));
            let elapsed = now.elapsed();
            println!("Task {} finished in {} seconds", i, elapsed.as_secs_f32());
        });
    }
}
