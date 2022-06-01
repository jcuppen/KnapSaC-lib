use std::borrow::BorrowMut;
use std::collections::{HashMap};
use serde::Deserialize;
use serde::Serialize;
use crate::{Dependency, HasDependencies};

#[derive(Deserialize, Serialize)]
pub struct Executable {
    pub(crate) dependencies: HashMap<String, Dependency>
}

impl Executable {
    pub(crate) fn create() -> Self {
        Executable {
            dependencies: HashMap::new(),
        }
    }
}

impl HasDependencies for Executable {
    fn dependencies(&self) -> &HashMap<String, Dependency> {
        &self.dependencies
    }

    fn dependencies_mut(&mut self) -> &mut HashMap<String, Dependency> {
        self.dependencies.borrow_mut()
    }
}