use crate::module::Module;
use crate::{Dependency, HasDependencies};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Package {
    modules: HashMap<String, Module>,
}

impl Package {
    pub(crate) fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier)
    }

    pub(crate) fn add_dependency(
        &mut self,
        module_identifier: &str,
        dependency_identifier: String,
        dependency: Dependency,
    ) {
        let m = self.modules.get_mut(module_identifier).unwrap();
        m.add_dependency(dependency_identifier, dependency);
    }
}
