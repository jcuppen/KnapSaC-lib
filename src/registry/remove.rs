use std::path::{Path};
use crate::dependency::Dependency;
use crate::registry::Registry;

impl Registry {
    pub(crate) fn remove_executable(&mut self, source_file: &Path) {
        self.items.remove(source_file);
    }
    pub(crate) fn remove_module(&mut self, source_file: &Path) {
        let i = self.items.remove_entry(source_file);
        if let Some((_,removed_module)) = i {
            for module in self.items.values_mut() {
                let dep = Dependency::Standalone(source_file.to_path_buf());
                if let Some(id) = &removed_module.identifier {
                    module.remove_dependency(id, &dep)
                }
            }
        }
    }
    pub fn remove_item(&mut self, source_file: &Path) {
        if let Some(i) = self.get_module_mut(source_file) {
            if !i.is_executable() {
                self.remove_module(source_file);
            } else {
                self.remove_executable(source_file);
            }
        }
        self.save();
    }
}
