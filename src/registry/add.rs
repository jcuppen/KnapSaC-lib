use crate::module::Module;
use crate::registry::Registry;
use std::path::{Path, PathBuf};
use crate::{Dependency, HasDependencies};
use crate::executable::Executable;

impl Registry {
    pub fn add_module(&mut self, identifier: String, output_directory: PathBuf) {
        let module = Module::create(output_directory);
        self.modules.insert(identifier, module);
        self.save();
    }
    pub fn add_dependency_to_module(&mut self, identifier: &str, dependency_identifier: String, dependency: Dependency) {
        if let Some(m) = self.modules.get_mut(identifier) {
            m.add_dependency(dependency_identifier, dependency);
        }
        self.save();
    }

    pub fn add_executable(&mut self, source_path: PathBuf) {
        let executable = Executable::create();
        self.executables.insert(source_path, executable);
        self.save();
    }
    pub fn add_dependency_to_executable(&mut self, source_path: &Path, dependency_identifier: String, dependency: Dependency) {
        if let Some(e) = self.executables.get_mut(source_path) {
            e.add_dependency(dependency_identifier, dependency);
        }
        self.save();
    }
}
