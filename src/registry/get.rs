use crate::dependency::Dependency;
use crate::module::Module;
use crate::registry::Registry;
use std::path::Path;

impl Registry {
    pub fn get_item(&self, source_path: &Path) -> Option<&Module> {
        self.items.get(source_path)
    }

    pub fn get_item_mut(&mut self, source_path: &Path) -> Option<&mut Module> {
        self.items.get_mut(source_path)
    }

    pub(crate) fn get_module_mut(&mut self, source_path: &Path) -> Option<&mut Module> {
        self.items
            .get_mut(source_path)
            .and_then(|i| if i.is_executable() { None } else { Some(i) })
    }

    pub fn get_module(&self, source_path: &Path) -> Option<&Module> {
        let i = self.items.get(source_path);
        if i?.is_executable() {
            None
        } else {
            i
        }
    }

    pub fn get_dependency(
        &self,
        source_path: &Path,
        dependency_identifier: &str,
    ) -> Option<&Dependency> {
        let i = self.get_item(source_path);
        let d = i?.get_dependency(dependency_identifier);
        if self.dependency_exists(d?) {
            return d;
        }
        None
    }
}
