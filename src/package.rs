use crate::module::Module;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;


use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;
use crate::language::Language;
use crate::package_manifest::PackageManifest;

use crate::version::{SemVerIncrement, Version};


#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub(crate) package_root: PathBuf,
    pub(crate) language: Language,
    pub(crate) modules: HashMap<String, (PathBuf, Module)>,
}

impl Package {
    fn manifest_path(&self) -> PathBuf {
        self.package_root.join("manifest.json")
    }

    pub(crate) fn create(package_root: PathBuf, compiler_command_name: String, output_option: String) -> Self {
        Package {
            package_root,
            language: Language { compiler_command_name, output_option },
            modules: HashMap::new(),
        }
    }

    pub fn build(&self) {
        for (path,module) in self.modules.values() {
            Command::new(&self.language.compiler_command_name)
                .arg(self.package_root.join(path))
                .arg(&self.language.output_option)
                .arg(self.package_root.join(&module.output_path)).output().expect("failed to build");
        }
    }

    pub(crate) fn add_module(&mut self, relative_path: PathBuf, module: Module) {
        self.modules.insert(module.identifier.clone().unwrap(), (relative_path, module));
    }

    pub(crate) fn has_module_source(&self, source_file: &Path) -> bool {
        let stripped_path = source_file.strip_prefix(&self.package_root).unwrap();
        self.modules.values().any(|(a, _)| stripped_path == a)
    }

    pub(crate) fn has_module_id(&self, identifier: &str) -> bool {
        self.modules.contains_key(identifier)
    }

    pub(crate) fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier).map(|(_, b)| b)
    }
    pub(crate) fn get_module_mut(&mut self, identifier: &str) -> Option<&mut Module> {
        self.modules.get_mut(identifier).map(|(_, b)| b)
    }

    pub(crate) fn search_modules(&self, identifier: &str) -> Vec<(&PathBuf, &Module)> {
        self.modules
            .iter()
            .filter(|&(id, _)| id == identifier)
            .map(|(_, (a, b))| (a, b))
            .collect()
    }

    pub(crate) fn set_remote_location(&self, git_url: Url) {
        let mut manifest = PackageManifest::load(self.manifest_path());
        manifest.remote_location = Some(git_url);
        manifest.save(self.manifest_path());
    }

    pub(crate) fn get_remote_location(&self) -> Option<Url> {
        let manifest = PackageManifest::load(self.manifest_path());
        manifest.remote_location
    }

    pub(crate) fn get_version(&self) -> Version {
        let manifest = PackageManifest::load(self.manifest_path());
        manifest.version
    }

    pub(crate) fn increment_version(&mut self, version_increment: SemVerIncrement) {
        let mut manifest = PackageManifest::load(self.manifest_path());
        manifest.increment_version(version_increment);
        manifest.save(self.manifest_path());
    }
}
