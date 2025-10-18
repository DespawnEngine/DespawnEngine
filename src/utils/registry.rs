use std::collections::HashMap;
use std::sync::Arc;

pub struct Registry<T> {
    entries: HashMap<String, Arc<T>>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn register(&mut self, id: &str, value: T) {
        self.entries.insert(id.to_string(), Arc::new(value));
    }

    pub fn get(&self, id: &str) -> Option<Arc<T>> {
        self.entries.get(id).cloned() // clone the Arc
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<T>)> {
        self.entries.iter()
    }
}