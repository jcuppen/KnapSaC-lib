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

    pub fn get_dependency_for_module(
        &self,
        module_identifier: &str,
        dependency_identifier: &str,
    ) -> Option<Module> {
        let m = self.modules.get(module_identifier)?;
        return match m.get_dependency(dependency_identifier)? {
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

    pub fn get_executable(&self, source_path: &Path) -> Option<&Executable> {
        self.executables.get(source_path)
    }

    pub fn get_dependency_for_executable(
        &self,
        source_path: &Path,
        dependency_identifier: &str,
    ) -> Option<Module> {
        let e = self.executables.get(source_path)?;
        return match e.get_dependency(dependency_identifier)? {
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
