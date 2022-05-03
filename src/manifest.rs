use crate::dependency::Dependency;
use crate::module::Module;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{read_to_string, write};
use std::path::Path;
use url::Url;

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub(crate) dependencies: HashSet<Dependency>,
    pub(crate) modules: Vec<Module>,
}

impl Manifest {
    pub(crate) fn initialize() -> Manifest {
        Manifest {
            dependencies: HashSet::new(),
            modules: vec![],
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

    pub(crate) fn add_dependency(&mut self, url: Url) {
        let dependency = Dependency { git_url: url };
        self.dependencies.insert(dependency);
    }

    pub(crate) fn remove_dependency(&mut self, url: Url) {
        self.dependencies.retain(|d| d.git_url != url);
    }
}
