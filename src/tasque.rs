use crate::lowlevel::Job;

pub struct Tasque {
    pub func: Job,
    pub requires: Option<&'static str>,
}

impl Tasque {
    pub fn requires<F>(requires: &'static str, func: F) -> Tasque
    where
        F: FnOnce() + Send + 'static,
    {
        Tasque {
            func: Box::new(func),
            requires: Some(requires),
        }
    }

    pub fn on_start<F>(func: F) -> Tasque
    where
        F: FnOnce() + Send + 'static,
    {
        Tasque {
            func: Box::new(func),
            requires: None,
        }
    }
}
