use std::borrow::BorrowMut;
use std::collections::{HashMap};
use std::path::PathBuf;
use serde::Serialize;
use serde::Deserialize;
use crate::{Dependency, HasDependencies};

#[derive(Deserialize, Serialize, Clone)]
pub struct Module {
    pub output_path: PathBuf,
    pub(crate) dependencies: HashMap<String, Dependency>
}

impl Module {
    pub(crate) fn create(output_path: PathBuf) -> Self {
        Module {
            output_path,
            dependencies: HashMap::new(),
        }
    }
}

impl HasDependencies for Module {
    fn dependencies(&self) -> &HashMap<String, Dependency> {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut HashMap<String, Dependency> {
        self.dependencies.borrow_mut()
    }
}