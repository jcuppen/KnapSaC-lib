use crate::registry::Registry;

impl Registry {
    pub fn remove_module(&mut self, identifier: &str) {
        self.modules.values_mut()
            .for_each(|m| m.remove_dependency(identifier));

        self.modules.remove(identifier);
        self.save()
    }

    fn remove_dependency(&mut self, identifier: &str, dependency_identifier: &str) {
        if let Some(m) = self.modules.get_mut(identifier) {
            m.remove_dependency(dependency_identifier);
        }
        self.save()
    }
}
