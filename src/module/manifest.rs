// use crate::error::ManifestError;
// use crate::error::ManifestError::InvalidManifest;
// use crate::module::standalone_module::StandaloneModule;
// use serde::{Deserialize, Serialize};
// use std::collections::HashSet;
// use std::fs::{read_to_string, write};
// use std::path::PathBuf;

// #[derive(Deserialize, Serialize, Clone)]
// pub(crate) struct Manifest {
//     #[serde(skip)]
    // pub(crate) location: PathBuf,
    // pub(crate) package_dependencies: HashSet<PackageDependency>,
    // pub(crate) module_dependencies: HashSet<StandaloneModule>,
// }

// impl Manifest {
    // Private Functions
/*
    fn initialize(manifest_path: PathBuf) -> Manifest {
        Manifest {
            location: manifest_path,
            module_dependencies: HashSet::new(),
        }
    }

    fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(self.location.clone(), contents).unwrap()
    }
 */
    // # Crate Public Functions
/*
    pub(crate) fn load(path: PathBuf) -> Result<Self, ManifestError> {
        if let Ok(data) = read_to_string(&path) {
            let res = serde_json::from_str(data.as_str());
            if res.is_err() {
                return Err(InvalidManifest);
            }
            let mut manifest: Manifest = res.unwrap();
            manifest.location = path;
            return Ok(manifest);
        }
        Ok(Self::initialize(path))
    }

    pub(crate) fn add_module_dependency(&mut self, dependency: StandaloneModule) {
        // self.module_dependencies.insert(dependency);
        self.save()
    }

    pub(crate) fn get_module_dependency(&self, identifier: &str) -> Option<&StandaloneModule> {
        self.module_dependencies.iter().find(|d| d.identifier == identifier)
    }
 */
    // pub(crate) fn has_module_dependency(&self, dependency: &StandaloneModule) -> bool {
        // self.module_dependencies.contains(dependency)
    // }
    /*
    pub(crate) fn remove_module_dependency(&mut self, dependency: &StandaloneModule) {
        self.module_dependencies.remove(dependency);
        self.save()
    }
     */
// }
