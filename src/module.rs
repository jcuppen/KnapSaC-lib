use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::path::{Path, PathBuf};
use crate::dependency::Dependency;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Module {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) identifier: Option<String>,
    pub output_path: PathBuf,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    dependencies: HashMap<String, Dependency>,
}

impl Module {
    pub(crate) fn create(output_path: &Path) -> Self {
        Module {
            identifier: None,
            output_path: output_path.to_path_buf(),
            dependencies: HashMap::new(),
        }
    }

    pub(crate) fn is_executable(&self) -> bool {
        self.identifier.is_none()
    }

    pub(crate) fn add_dependency(&mut self, identifier: String, dependency: Dependency) {
        self.dependencies.insert(identifier, dependency);
    }

    pub(crate) fn get_dependency(&self, identifier: &str) -> Option<&Dependency> {
        self.dependencies.get(identifier)
    }

    pub(crate) fn has_dependency(&self, identifier: &str) -> bool {
        self.dependencies.contains_key(identifier)
    }

    pub(crate) fn remove_dependency(&mut self, identifier: &str, dependency: &Dependency) {
        if match self.dependencies.get(identifier) {
            None => false,
            Some(d) => d == dependency,
        } {
            self.dependencies.remove(identifier);
        }
    }
}
