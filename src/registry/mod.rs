mod add;
mod get;
mod remove;

use crate::module::Module;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use crate::executable::Executable;

#[derive(Serialize, Deserialize)]
pub struct Registry {
    modules: HashMap<String, Module>,
    executables: HashMap<PathBuf, Executable>
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
            modules: HashMap::new(),
            executables: HashMap::new(),
        }
    }

    pub fn load() -> Self {
        if let Ok(data) = read_to_string(Self::registry_path()) {
            match serde_json::from_str(data.as_str()) {
                Err(e) => {
                    panic!("{}", e.to_string())
                }
                Ok(i) => return i,
            }
        }
        Registry::init()
    }
}
