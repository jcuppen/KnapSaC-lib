use std::path::Path;
use crate::registry::Registry;

impl Registry {
    pub fn remove_module(&mut self, identifier: &str) {
        self.modules.values_mut()
            .for_each(|m| m.remove_dependency(identifier));
        self.executables.values_mut()
            .for_each(|e| e.remove_dependency(identifier));


        self.modules.remove(identifier);
        self.save()
    }

    // fn remove_dependency_from_module(&mut self, identifier: &str, dependency_identifier: &str) {
    //     if let Some(m) = self.modules.get_mut(identifier) {
    //         m.remove_dependency(dependency_identifier);
    //     }
    //     self.save()
    // }
    //

    pub fn remove_executable(&mut self, source_path: &Path) {
        self.executables.remove(source_path);
        self.save()
    }

    //
    // fn remove_dependency_from_executable(&mut self, source_path: &Path, dependency_identifier: &str) {
    //     if let Some(e) = self.executables.get_mut(source_path) {
    //         e.remove_dependency(dependency_identifier);
    //     }
    //     self.save()
    // }
}
