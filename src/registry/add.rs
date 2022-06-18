use crate::dependency::{Dependency, HasDependencies};
use crate::module::Module;
use crate::registry::Registry;
use std::path::{Path, PathBuf};

impl Registry {
    pub fn add_item(&mut self, source_file: PathBuf, output_directory: PathBuf) {
        let module = Module::create_module(output_directory);
        self.items.insert(source_file, module);
        self.save();
    }

    pub fn add_dependency_to_item(&mut self, source_file: &Path, dependency: Dependency) {
        if !self.dependency_exists(&dependency) {
            panic!()
        }

        let identifier = match &dependency {
            Dependency::Stray(identifier, _) => identifier.to_string(),
            Dependency::Standalone(_) | Dependency::Package(_, _) => self
                .dep_to_module(&dependency)
                .cloned()
                .unwrap()
                .identifier
                .unwrap(),
        };

        let m = self.get_item_mut(source_file).unwrap();
        m.add_dependency(identifier, dependency);

        self.save();
    }

    pub fn add_dependency_to_package_module(
        &mut self,
        package_identifier: &str,
        module_identifier: &str,
        dependency: Dependency,
    ) {
        if !dependency.is_package_module() {
            panic!()
        }

        if !self.dependency_exists(&dependency) {
            panic!()
        }

        let identifier = match &dependency {
            Dependency::Stray(_, _) | Dependency::Standalone(_) => panic!(),
            Dependency::Package(_, _) => self
                .dep_to_module(&dependency)
                .cloned()
                .unwrap()
                .identifier
                .unwrap(),
        };

        let package = self.get_package_mut(package_identifier).unwrap();
        let module = package.get_module_mut(module_identifier).unwrap();
        module.add_dependency(identifier, dependency);

        self.save();
    }
}
