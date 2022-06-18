use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::path::{PathBuf};
use crate::dependency::{Dependency, HasDependencies};

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
    pub(crate) fn create_module(output_path: PathBuf) -> Self {
        Module {
            identifier: None,
            output_path,
            dependencies: HashMap::new(),
        }
    }

    pub(crate) fn is_executable(&self) -> bool {
        self.identifier.is_none()
    }
}

impl HasDependencies for Module {
    fn dependencies(&self) -> &HashMap<String, Dependency> {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut HashMap<String, Dependency> {
        &mut self.dependencies
    }
}
