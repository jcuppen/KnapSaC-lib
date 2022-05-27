use crate::module::Module;


impl Module {
    pub(crate) fn has_dependency(&self, identifier: &str) -> bool {
        self.dependencies.contains(identifier)
    }
}
