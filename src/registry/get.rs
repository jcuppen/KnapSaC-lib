use crate::module::Module;
use crate::registry::Registry;

impl Registry {
    pub fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier)
    }

    pub fn get_dependency(&self, module_identifier: &str, dependency_identifier: &str) -> Option<&Module> {
        if let Some(m) = self.modules.get(module_identifier) {
           if m.has_dependency(dependency_identifier) {
               return self.modules.get(dependency_identifier)
           }
        }
        None
    }
}
