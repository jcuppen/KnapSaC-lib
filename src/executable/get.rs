use crate::executable::Executable;

impl Executable {
    pub(crate) fn has_dependency(&self, identifier: &str) -> bool {
        self.dependencies.contains(identifier)
    }
}
