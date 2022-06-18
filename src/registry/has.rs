use crate::dependency::{Dependency, HasDependencies};
use crate::registry::Registry;
use std::path::Path;

impl Registry {
    pub fn has_item_source(&self, source_path: &Path) -> bool {
        self.items.contains_key(source_path)
    }

    pub fn has_module_source(&self, source_path: &Path) -> bool {
        if self.get_module(source_path).is_some() {
            return true;
        }

        self.packages
            .values()
            .any(|v| v.has_module_source(source_path))
    }

    pub fn has_module_id(&self, identifier: &str) -> bool {
        let in_items = self.items.values().any(|v| {
            if let Some(id) = &v.identifier {
                return id == identifier && !v.is_executable();
            }
            false
        });

        if in_items {
            return true;
        }

        self.packages.values().any(|v| v.has_module_id(identifier))
    }

    pub fn has_package(&self, identifier: &str) -> bool {
        self.packages.contains_key(identifier)
    }

    pub fn has_dependency(&self, source_path: &Path, dependency_identifier: &str) -> bool {
        if let Some(i) = self.get_item(source_path) {
            return i.has_dependency(dependency_identifier);
        }
        false
    }

    pub(crate) fn dependency_exists(&self, dependency: &Dependency) -> bool {
        match dependency {
            Dependency::Stray(_identifier, _output_dir) => true,
            Dependency::Standalone(source_file) => self.has_module_source(source_file),
            Dependency::Package(package_id, module_id) => {
                if let Some(p) = self.get_package(package_id) {
                    return p.has_module_id(module_id);
                }
                false
            }
        }
    }

    pub fn has_executable_source(&self, source_path: &Path) -> bool {
        self.items
            .iter()
            .any(|(k, v)| k == source_path && v.is_executable())
    }
}
