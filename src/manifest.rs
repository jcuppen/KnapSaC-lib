use crate::dependency::Dependency;
use crate::module::Module;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{read_to_string, write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub(crate) struct Manifest {
    pub(crate) dependencies: HashSet<Dependency>,
    pub(crate) modules: HashSet<Module>,
}

impl Manifest {
    pub(crate) fn initialize() -> Manifest {
        Manifest {
            dependencies: HashSet::new(),
            modules: HashSet::new(),
        }
    }
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(data) = read_to_string(&path) {
            return serde_json::from_str(data.as_str()).unwrap();
        }
        panic!("No manifest found @ {}", path.as_ref().display())
    }
    pub(crate) fn save<P: AsRef<Path>>(&self, path: P) {
        let contents = serde_json::to_string(self).unwrap();
        write(path, contents).unwrap()
    }

    pub(crate) fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.insert(dependency);
    }
    pub(crate) fn has_dependency(&self, dependency: &Dependency) -> bool {
        self.dependencies.contains(dependency)
    }
    pub(crate) fn remove_dependency(&mut self, dependency: &Dependency) {
        self.dependencies.remove(dependency);
    }

    pub(crate) fn add_module(&mut self, module: Module) {
        self.modules.insert(module);
    }
    pub(crate) fn get_module_by_location<P: AsRef<Path>>(&self, path: P) -> Option<&Module> {
        self.modules.iter().find(|m|m.location == path.as_ref().to_path_buf())
    }
    pub(crate) fn remove_module(&mut self, module: &Module) {
        self.modules.remove(module);
    }
}
