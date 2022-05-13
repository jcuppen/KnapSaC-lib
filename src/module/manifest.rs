use std::collections::HashSet;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::error::ManifestError;
use crate::error::ManifestError::InvalidManifest;
use crate::module::standalone_module::StandaloneModule;

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Manifest {
    #[serde(skip)]
    pub(crate) location: PathBuf,
    // pub(crate) package_dependencies: HashSet<PackageDependency>,
    pub(crate) module_dependencies: HashSet<StandaloneModule>,
}

impl Manifest {
    pub(crate) fn initialize<P: AsRef<Path>>(manifest_path: P) -> Manifest {
        Manifest {
            location: manifest_path.as_ref().to_path_buf(),
            module_dependencies: HashSet::new(),
        }
    }

    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<Self, ManifestError> {
        if let Ok(data) = read_to_string(&path) {
            let res = serde_json::from_str(data.as_str());
            if res.is_err() {
                return Err(InvalidManifest);
            }
            let mut manifest: Manifest = res.unwrap();
            manifest.location = path.as_ref().to_path_buf();
            return Ok(manifest);
        }
        Ok(Self::initialize(path))
    }

    pub(crate) fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(self.location.clone(), contents).unwrap()
    }

    pub(crate) fn add_module_dependency(&mut self, dependency: StandaloneModule) {
        self.module_dependencies.insert(dependency);
        self.save()
    }
    pub(crate) fn has_module_dependency(&self, dependency: &StandaloneModule) -> bool {
        self.module_dependencies.contains(dependency)
    }
    pub(crate) fn remove_module_dependency(&mut self, dependency: &StandaloneModule) {
        self.module_dependencies.remove(dependency);
    }
}
