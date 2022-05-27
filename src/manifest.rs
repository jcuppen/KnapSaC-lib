// use crate::dependency::PackageDependency;
use crate::module::package_module::PackageModule;
use crate::module::standalone_module::StandaloneModule;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use std::path::{PathBuf};

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Manifest {
    location: PathBuf,
    // pub(crate) package_dependencies: HashSet<PackageDependency>,
    pub(crate) module_dependencies: HashSet<StandaloneModule>,
    pub(crate) modules: HashSet<PackageModule>,
}

/*
impl Manifest {
    pub(crate) fn initialize<P: AsRef<Path>>(path: P) -> Manifest {
        Manifest {
            location: path.as_ref().to_path_buf(),
            // package_dependencies: HashSet::new(),
            module_dependencies: HashSet::new(),
            modules: HashSet::new(),
        }
    }
    pub(crate) fn load(path: &Path) -> Result<Self, ManifestError> {
        if let Ok(data) = read_to_string(&path) {
            let res = serde_json::from_str(data.as_str());
            if res.is_err() {
                return Err(InvalidManifest);
            }
            let mut manifest: Manifest = res.unwrap();
            manifest.location = path.to_path_buf();
            return Ok(manifest);
        }
        Ok(Self::initialize(path))
    }
    pub(crate) fn save<P: AsRef<Path>>(&self, path: P) {
        let contents = serde_json::to_string(self).unwrap();
        write(path, contents).unwrap()
    }
/*
    pub(crate) fn add_package_dependency(&mut self, dependency: PackageDependency) {
        self.package_dependencies.insert(dependency);
    }
    pub(crate) fn has_package_dependency(&self, dependency: &PackageDependency) -> bool {
        self.package_dependencies.contains(dependency)
    }
    pub(crate) fn remove_package_dependency(&mut self, dependency: &PackageDependency) {
        self.package_dependencies.remove(dependency);
    }
*/
    pub(crate) fn add_module_dependency(&mut self, dependency: StandaloneModule) {
        self.module_dependencies.insert(dependency);
    }
    pub(crate) fn has_module_dependency(&self, dependency: &StandaloneModule) -> bool {
        self.module_dependencies.contains(dependency)
    }
    pub(crate) fn remove_module_dependency(&mut self, dependency: &StandaloneModule) {
        self.module_dependencies.remove(dependency);
    }

    pub(crate) fn add_module(&mut self, module: PackageModule) {
        self.modules.insert(module);
    }
    pub(crate) fn get_module_by_location(&self, path: &Path) -> Option<&PackageModule> {
        self.modules
            .iter()
            .find(|m| m.location == path)
    }
    pub(crate) fn remove_module(&mut self, module: &PackageModule) {
        self.modules.remove(module);
    }
}
*/
