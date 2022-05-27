use std::path::Path;
use crate::executable::Executable;
use crate::module::Module;
use crate::registry::Registry;

impl Registry {
    pub fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier)
    }

    pub fn get_dependency_for_module(&self, module_identifier: &str, dependency_identifier: &str) -> Option<&Module> {
        if let Some(m) = self.modules.get(module_identifier) {
            if m.has_dependency(dependency_identifier) {
                return self.modules.get(dependency_identifier)
            }
        }
        None
    }

    pub fn get_executable(&self, source_path: &Path) -> Option<&Executable> {
        self.executables.get(source_path)
    }

    pub fn get_dependency_for_executable(&self, source_path: &Path, dependency_identifier: &str) -> Option<&Module> {
        if let Some(e) = self.executables.get(source_path) {
            if e.has_dependency(dependency_identifier) {
                return self.modules.get(dependency_identifier)
            }
        }
        None
    }
}
