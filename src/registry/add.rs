use crate::module::Module;
use crate::registry::Registry;
use std::path::PathBuf;
use crate::{Dependency, HasDependencies};
use crate::entry::Entry;
use crate::executable::Executable;

impl Registry {
    pub fn add_module(&mut self, identifier: String, output_directory: PathBuf) {
        let module = Module::create(output_directory);
        self.modules.insert(identifier, module);
        self.save();
    }

    pub fn add_executable(&mut self, source_path: PathBuf) {
        let executable = Executable::create();
        self.executables.insert(source_path, executable);
        self.save();
    }

    pub fn add_dependency(&mut self, entry: Entry, dependency_identifier: String, dependency: Dependency) {
        if self.get_module(&dependency_identifier).is_some() {
            match &entry {
                Entry::Executable(source_path) => {
                    let e = self.executables.get_mut(source_path).unwrap();
                    e.add_dependency(dependency_identifier, dependency);
                }
                Entry::StandaloneModule(identifier) => {
                    let m = self.modules.get_mut(identifier).unwrap();
                    m.add_dependency(dependency_identifier, dependency);
                }
                Entry::PackageModule(package_identifier, module_identifier) => {
                    let p = self.packages.get_mut(package_identifier).unwrap();
                    p.add_dependency(module_identifier, dependency_identifier, dependency);
                }
            }

        }
        println!("{:?}", self);
        self.save();
    }
}
