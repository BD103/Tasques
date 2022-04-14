use tasques::pool::ThreadPool;

use std::thread;
use std::time::Duration;

fn main() {
    let pool = ThreadPool::new(4).unwrap();

    pool.execute(|| {
        thread::sleep(Duration::from_secs(4));
        println!("Done sleeping 4s");
    });

    pool.execute(|| {
        thread::sleep(Duration::from_secs(7));
        println!("Done sleeping 7s");
    });
}
