use crate::executable::Executable;

impl Executable {
    pub(crate) fn remove_dependency(&mut self, identifier: &str) {
        self.dependencies.remove(identifier);
    }
}