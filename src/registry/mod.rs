mod add;
mod get;
mod has;
mod mark;
mod remove;
mod search;
mod package;

use crate::dependency::{Dependency};
use crate::module::Module;
use crate::package::Package;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Registry {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    packages: HashMap<PathBuf, Package>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    items: HashMap<PathBuf, Module>,
}

impl Registry {
    fn registry_path() -> PathBuf {
        dirs::home_dir().unwrap().join("knapsac_registry.json")
    }

    fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(Self::registry_path(), contents).unwrap();
    }

    fn init() -> Self {
        Registry {
            packages: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn load() -> Self {
        if let Ok(data) = read_to_string(Self::registry_path()) {
            match serde_json::from_str(data.as_str()) {
                Err(e) => panic!("{}", e.to_string()),
                Ok(i) => return i,
            }
        }
        Registry::init()
    }

    pub fn dep_to_module(&self, dependency: &Dependency) -> Option<&Module> {
        if self.dependency_exists(dependency) {
            return match dependency {
                Dependency::Stray(_, _) => panic!(),
                Dependency::Standalone(s) => self.get_module(s),
                Dependency::Package(package_identifier, module_identifier) => self
                    .get_package(package_identifier)?.1
                    .get_module(module_identifier),
            };
        }
        None
    }
}
