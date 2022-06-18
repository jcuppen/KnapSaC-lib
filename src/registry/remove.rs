use std::path::{Path};
use crate::dependency::{Dependency, HasDependencies};
use crate::module::Module;
use crate::registry::Registry;

impl Registry {
    pub(crate) fn remove_executable(&mut self, source_file: &Path) {
        self.items.remove(source_file);
    }
    pub(crate) fn remove_module(&mut self, source_file: &Path, remove_dependencies: bool) -> Option<Module> {
        let r_mod = self.items.remove(source_file);
        if let Some(removed_module) = &r_mod {
            if remove_dependencies {
                let dep = Dependency::Standalone(source_file.to_path_buf());

                for item in self.items.values_mut() {
                    if let Some(id) = &removed_module.identifier {
                        item.remove_dependency(id, &dep)
                    }
                }
            }
        }
        r_mod
    }

    pub fn remove_item(&mut self, source_file: &Path) -> Option<Module> {
        let mut removed_item = None;
        if let Some(item) = self.get_module_mut(source_file) {
            if !item.is_executable() {
                removed_item = self.remove_module(source_file, true);
            } else {
                self.remove_executable(source_file);
            }
        }
        self.save();
        removed_item
    }

    pub fn remove_package(&mut self, package_identifier: &str) {
        let removed_package_opt = self.packages.remove(package_identifier);
        if removed_package_opt.is_none() {
            return;
        }

        let removed_package = removed_package_opt.unwrap();

        for removed_module_id in removed_package.modules.keys() {
            let dep = Dependency::Package(package_identifier.to_string(), removed_module_id.clone());

            for item in self.items.values_mut() {
                item.remove_dependency(removed_module_id, &dep);
            }

            for package in self.packages.values_mut() {
                for (_, package_module) in package.modules.values_mut() {
                    package_module.remove_dependency(removed_module_id, &dep)
                }
            }
        }

        self.save();
    }
}
