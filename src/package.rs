
use crate::module::Module;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;


use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;
use crate::language::Language;

use crate::version::{SemVerIncrement, Version};
use crate::version::Version::SemVer;


#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub(crate) identifier: String,
    pub(crate) version: Version,
    pub(crate) language: Language,
    pub(crate) remote_location: Option<Url>,
    pub(crate) modules: HashMap<String, (PathBuf, Module)>,
}

impl Package {
    pub(crate) fn create(identifier: String, language: Language) -> Self {
        Package {
            identifier,
            version: Default::default(),
            language,
            remote_location: None,
            modules: Default::default()
        }
    }

    pub fn build(&self, package_root: &Path) {
        for (path,module) in self.modules.values() {
            Command::new(&self.language.compiler_command_name)
                .arg(package_root.join(path))
                .arg(&self.language.output_option)
                .arg(package_root.join(&module.output_path)).output().expect("failed to build");
        }
    }

    pub(crate) fn add_module(&mut self, relative_path: PathBuf, module: Module) {
        self.modules.insert(module.identifier.clone().unwrap(), (relative_path, module));
    }

    pub(crate) fn has_module_source(&self, package_root: &Path, source_file: &Path) -> bool {
        let stripped_path = source_file.strip_prefix(package_root).unwrap();
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

    pub(crate) fn increment_version(&mut self, version_increment: SemVerIncrement) {
        let new_version = match self.version {
            Version::NotVersioned => {
                match version_increment {
                    SemVerIncrement::Major => SemVer(1,0,0),
                    SemVerIncrement::Minor => SemVer(0,1,0),
                    SemVerIncrement::Patch => SemVer(0,0,1),
                }
            }
            SemVer(major, minor, patch) => {
                match version_increment {
                    SemVerIncrement::Major => SemVer(major + 1,0,0),
                    SemVerIncrement::Minor => SemVer(major,minor + 1,0),
                    SemVerIncrement::Patch => SemVer(major,minor,patch + 1),
                }
            }
        };
        self.version = new_version;
    }
}
