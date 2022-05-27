use crate::executable::Executable;
use crate::registry::Registry;

impl Executable {
    pub(crate) fn add_dependency(&mut self, identifier: &str) {
        if Registry::load().get_module(identifier).is_some() {
            self.dependencies.insert(identifier.to_string());
        }
    }
}