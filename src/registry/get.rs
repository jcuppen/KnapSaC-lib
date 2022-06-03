use crate::entry::Entry;
use crate::executable::Executable;
use crate::module::Module;
use crate::registry::Registry;
use crate::Dependency::{PackageModule, StandaloneModule, StrayModule};
use crate::HasDependencies;
use std::collections::HashMap;
use std::path::Path;

impl Registry {
    pub fn get_module(&self, identifier: &str) -> Option<&Module> {
        self.modules.get(identifier)
    }

    pub fn get_executable(&self, source_path: &Path) -> Option<&Executable> {
        self.executables.get(source_path)
    }

    pub fn get_dependency(&self, entry: Entry, dependency_identifier: &str) -> Option<Module> {
        let d = match &entry {
            Entry::Executable(source_path) => self
                .get_executable(source_path)?
                .get_dependency(dependency_identifier),
            Entry::StandaloneModule(identifier) => self
                .get_module(identifier)?
                .get_dependency(dependency_identifier),
            Entry::PackageModule(package_identifier, module_identifier) => self
                .packages
                .get(package_identifier)?
                .get_module(module_identifier)?
                .get_dependency(dependency_identifier),
        };

        return match d? {
            StrayModule(output_path) => Some(Module {
                output_path: output_path.to_path_buf(),
                dependencies: HashMap::new(),
            }),
            StandaloneModule => self.modules.get(dependency_identifier).cloned(),
            PackageModule => {
                panic!()
            }
        };
    }
}
