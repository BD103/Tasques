pub struct Tasque {
    id: String,
    func: Box<dyn FnOnce()>,
    pub requires: Option<String>,
}

impl Tasque {
    pub fn new(id: String, func: impl FnOnce() + 'static) -> Self {
        Tasque {
            id,
            func: Box::new(func),
            requires: None,
        }
    }

    pub fn requires(mut self, id: String) -> Self {
        self.requires = Some(id);
        self
    }

    pub fn on_start(mut self) -> Self {
        self.requires = None;
        self
    }

    pub(crate) fn run(&mut self) {
        // RUN THING
    }
}
