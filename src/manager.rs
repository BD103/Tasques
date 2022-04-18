use crate::lowlevel::ThreadPool;
use crate::Tasque;

pub struct TasqueManager {
    tasques: Vec<Tasque>,
    pool_size: usize,
}

impl TasqueManager {
    pub fn new() -> TasqueManager {
        TasqueManager {
            tasques: Vec::new(),
            // 4 is default for now
            pool_size: 4,
        }
    }

    pub fn with_size(size: usize) -> TasqueManager {
        TasqueManager {
            tasques: Vec::new(),
            pool_size: size,
        }
    }

    pub fn register(&mut self, tasque: Tasque) {
        &self.tasques.push(tasque);
    }

    pub fn run(&mut self) {
        let pool = ThreadPool::new(self.pool_size).unwrap();
    }
}
