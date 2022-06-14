use crate::module::Module;
use crate::registry::Registry;
use std::path::PathBuf;
use crate::dependency::{Dependency};

impl Registry {
    pub fn add_item(
        &mut self,
        source_file: PathBuf,
        output_directory: PathBuf,
    ) {
        let module = Module::create(&output_directory);
        println!("{:?}", module);
        self.items.insert(source_file, module);
        self.save();
    }

    pub fn add_dependency(
        &mut self,
        source_file: PathBuf,
        dependency: Dependency,
    ) {
        if self.dependency_exists(&dependency) {
            let identifier = if let Dependency::Stray(id,_) = &dependency {
                id.to_string()
            } else {
                let d = self.dep_to_module(&dependency).unwrap();
                d.identifier.unwrap()
            };

            let m = self.get_item_mut(&source_file).unwrap();
            m.add_dependency(identifier, dependency);
        } else {
            panic!();
        }

        self.save();
    }
}
