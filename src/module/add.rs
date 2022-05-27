use crate::module::Module;
use crate::registry::Registry;

impl Module {
    pub(crate) fn add_dependency(&mut self, identifier: &str) {
        if Registry::load().get_module(identifier).is_some() {
            self.dependencies.insert(identifier.to_string());
        }
    }
}
