use crate::entry::Entry;
use crate::HasDependencies;
use crate::registry::Registry;

impl Registry {
    pub fn remove(&mut self, entry: Entry) {
        match entry {
            Entry::Executable(source_path) => {
                self.executables.remove(&source_path);
            }
            Entry::StandaloneModule(identifier) => {
                self.modules.values_mut()
                    .for_each(|m| m.remove_dependency(&identifier));
                self.executables.values_mut()
                    .for_each(|e| e.remove_dependency(&identifier));

                self.modules.remove(&identifier);
            }
            Entry::PackageModule(_package_identifier, _module_identifier) => {
                panic!()
            }
        };
        self.save();
    }
}
