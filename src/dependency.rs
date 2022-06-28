use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Hash, Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub enum Dependency {
    Stray(String, PathBuf),
    Standalone(PathBuf),
    Package(String, String),
}

impl Dependency {
    pub(crate) fn is_package_module(&self) -> bool {
        match self {
            Dependency::Stray(_, _) |
            Dependency::Standalone(_) => false,
            Dependency::Package(_, _) => true,
        }
    }
}

pub(crate) trait HasDependencies {
    fn dependencies(&self) -> &HashMap<String, Dependency>;
    fn dependencies_mut(&mut self) -> &mut HashMap<String, Dependency>;

    fn add_dependency(&mut self, identifier: String, dependency: Dependency) {
        self.dependencies_mut().insert(identifier, dependency);
    }

    fn get_dependency(&self, identifier: &str) -> Option<&Dependency> {
        self.dependencies().get(identifier)
    }

    fn has_dependency(&self, identifier: &str) -> bool {
        self.dependencies().contains_key(identifier)
    }

    fn has_only_package_module_dependencies(&self) -> bool {
        self.dependencies().iter().all(|(_,d)|d.is_package_module())
    }

    fn remove_dependency(&mut self, identifier: &str, dependency: &Dependency) {
        if match self.dependencies().get(identifier) {
            None => false,
            Some(d) => d == dependency,
        } {
            self.dependencies_mut().remove(identifier);
        }
    }
}
