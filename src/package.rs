use crate::module::Module;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    package_root: PathBuf,
    pub(crate) modules: HashMap<String, (PathBuf, Module)>,
}

impl Package {
    pub(crate) fn create(package_root: PathBuf) -> Self {
        Package {
            package_root,
            modules: HashMap::new(),
        }
    }

    pub(crate) fn add_module(&mut self, relative_path: PathBuf, module: &Module) {
        self.modules.insert(module.clone().identifier.unwrap(), (relative_path, module.clone()));
    }

    pub(crate) fn has_module_source(&self, source_file: &Path) -> bool {
        let stripped_path = source_file.strip_prefix(&self.package_root).unwrap();
        self.modules.values().any(|(a, _)| stripped_path == a)
    }
    pub(crate) fn has_module_id(&self, identifier: &str) -> bool {
        self.modules.contains_key(identifier)
    }

    pub(crate) fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier).map(|(_, b)| b)
    }
    pub(crate) fn get_module_mut(&mut self, identifier: &str) -> Option<&mut Module> {
        self.modules.get_mut(identifier).map(|(_, b)| b)
    }

    pub(crate) fn search_modules(&self, identifier: &str) -> Vec<(&PathBuf, &Module)> {
        self.modules
            .iter()
            .filter(|&(id, _)| id == identifier)
            .map(|(_, (a, b))| (a, b))
            .collect()
    }
}
