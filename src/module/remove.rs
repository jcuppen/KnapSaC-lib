use crate::module::Module;

impl Module {
    pub(crate) fn remove_dependency(&mut self, identifier: &str) {
        self.dependencies.remove(identifier);
    }
}
