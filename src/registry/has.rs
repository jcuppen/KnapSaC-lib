use crate::dependency::Dependency;
use crate::registry::Registry;
use std::path::Path;

impl Registry {
    pub fn has_item_source(&self, source_path: &Path) -> bool {
        self.items.contains_key(source_path)
    }

    pub fn has_module_source(&self, source_path: &Path) -> bool {
        self.items
            .iter()
            .any(|(k, v)| k == source_path && !v.is_executable())
    }
    pub fn has_module_id(&self, identifier: &str) -> bool {
        self.items.values().any(|v| {
            if let Some(id) = &v.identifier {
                id == identifier && !v.is_executable()
            } else {
                false
            }
        })
    }

    pub fn has_dependency(&self, source_path: &Path, dependency_identifier: &str) -> bool {
        if let Some(i) = self.get_item(source_path) {
            return i.has_dependency(dependency_identifier);
        }
        false
    }

    pub(crate) fn dependency_exists(&self, dependency: &Dependency) -> bool {
        match dependency {
            Dependency::Stray(_, _) => true,
            Dependency::Standalone(source_path) => self.has_module_source(source_path),
            Dependency::Package => panic!(),
        }
    }

    pub fn has_executable_source(&self, source_path: &Path) -> bool {
        self.items.iter().any(|(k,v)|k == source_path && v.is_executable())
    }
}
