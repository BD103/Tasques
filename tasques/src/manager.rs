use crate::Tasque;
use crate::lowlevel::ThreadPool;

use std::collections::HashMap;

pub struct TasqueManager {
    on_start: Vec<Tasque>,
    dep_tasques: HashMap<String, Vec<Tasque>>,
}

impl TasqueManager {
    pub fn new() -> Self {
        TasqueManager {
            on_start: Vec::new(),
            dep_tasques: HashMap::new(),
        }
    }

    pub fn register(&mut self, tasque: Tasque) {
        match tasque.requires {
            Some(ref id) => {
                let x = self.dep_tasques.entry(id.to_string()).or_insert(Vec::new());
                x.push(tasque);
            }
            None => {
                self.on_start.push(tasque);
            }
        }
    }

    pub fn run_with(&mut self, threads: usize) {
        let pool = ThreadPool::new(threads).unwrap();

        for t in &self.on_start {
            pool.execute(|| {
                println!("hi");
            });
        }
    }
}
