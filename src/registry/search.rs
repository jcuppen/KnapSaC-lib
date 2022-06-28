use crate::module::Module;
use crate::registry::Registry;
use std::path::{Path, PathBuf};

impl Registry {
    pub fn search_modules_by_id(&self, identifier: &str) -> Vec<(&PathBuf, &Module)> {
        self.items
            .iter()
            .filter(|(_, v)| {
                if let Some(id) = &v.identifier {
                    id == identifier
                } else {
                    false
                }
            })
            .collect()
    }

    pub(crate) fn search_modules_by_source_prefix(&self, prefix: &Path) -> Vec<(&PathBuf, &Module)> {
        self.items
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .collect()
    }

    pub fn search_package_modules(&self, identifier: &str) -> Vec<(&PathBuf, &Module)> {
        self.packages
            .iter()
            .flat_map(|(_, package)| package.search_modules(identifier))
            .collect()
    }
}
