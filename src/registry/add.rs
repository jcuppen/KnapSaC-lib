use crate::module::Module;
use crate::registry::Registry;
use std::path::PathBuf;

impl Registry {
    pub fn add_module(&mut self, identifier: String, output_directory: PathBuf) {
        let module = Module::create(output_directory);
        self.modules.insert(identifier, module);
        self.save();
    }

    pub fn add_dependency(&mut self, identifier: &str, dependency_identifier: &str) {
        if let Some(m) = self.modules.get_mut(identifier) {
            m.add_dependency(dependency_identifier);
        }
        self.save();
    }
}
