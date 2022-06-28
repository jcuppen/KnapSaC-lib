use crate::dependency::{Dependency, HasDependencies};
use crate::module::Module;
use crate::package::Package;
use crate::registry::Registry;
use std::path::{Path, PathBuf};

impl Registry {
    pub fn get_item(&self, source_path: &Path) -> Option<&Module> {
        self.items.get(source_path)
    }

    pub fn get_item_mut(&mut self, source_path: &Path) -> Option<&mut Module> {
        self.items.get_mut(source_path)
    }

    pub fn get_package(&self, identifier: &str) -> Option<(&PathBuf, &Package)> {
        self.packages.iter().find(|(_,package)|package.identifier == identifier)
    }
    pub fn get_package_mut(&mut self, identifier: &str) -> Option<(&PathBuf, &mut Package)> {
        self.packages.iter_mut().find(|(_,package)|package.identifier == identifier)
    }

    pub(crate) fn get_module_mut(&mut self, source_path: &Path) -> Option<&mut Module> {
        self.items
            .get_mut(source_path)
            .and_then(|i| if i.is_executable() { None } else { Some(i) })
    }

    pub fn get_module(&self, source_path: &Path) -> Option<&Module> {
        self.items
            .get(source_path)
            .and_then(|v| if v.is_executable() { None } else { Some(v) })
    }

    pub fn get_dependency(
        &self,
        source_path: &Path,
        dependency_identifier: &str,
    ) -> Option<&Dependency> {

        let i = self.get_item(source_path)?
            .get_dependency(dependency_identifier)
            .and_then(|d| {
                if self.dependency_exists(d) {
                    Some(d)
                } else {
                    None
                }
            });
        i
    }
}
