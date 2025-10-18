use std::collections::HashMap;

pub struct Registry<T> {
    entries: HashMap<String, T>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn register(&mut self, id: &str, value: T) {
        self.entries.insert(id.to_string(), value);
    }

    pub fn get(&self, id: &str) -> Option<&T> {
        self.entries.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.entries.iter()
    }
}