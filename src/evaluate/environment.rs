use std::collections::HashMap;
use crate::evaluate::object::Object;

#[derive(Default, Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, identifier: &String) -> Option<&Object> {
        self.store.get(identifier)
    }

    pub fn set(&mut self, identifier: String, object: Object) {
        self.store.insert(identifier, object);
    }

    pub fn reset(&mut self) {
        self.store.clear();
    }
}