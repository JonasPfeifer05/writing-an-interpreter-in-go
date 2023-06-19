use std::collections::HashMap;
use crate::evaluate::build_in::LenFunction;
use crate::evaluate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    build_in: HashMap<String, Object>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut build_in = HashMap::default();

        build_in.insert("len".to_string(), Object::BuildIn(Box::new(LenFunction)));

        Self {
            store: HashMap::default(),
            build_in
        }
    }
}

impl Environment {
    pub fn get_build_in(&self, identifier: &String) -> Option<&Object> {
        self.build_in.get(identifier)
    }

    pub fn get(&self, identifier: &String) -> Option<&Object> {
        self.store.get(identifier)
    }

    pub fn set(&mut self, identifier: String, object: Object) {
        self.store.insert(identifier, object);
    }

    pub fn store(&self) -> &HashMap<String, Object> {
        &self.store
    }
}