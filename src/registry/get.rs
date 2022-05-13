use crate::error::{NotAPackageError, RepositoryError};
use crate::package::Package;
use crate::registry::Registry;
use crate::utils::infer_working_directory;
use std::path::{Path, PathBuf};
use crate::module::standalone_module::StandaloneModule;

pub enum FindBy {
    LocalLocation(PathBuf),
}

impl Registry {
    /// Retrieves the [`Package`] that is registered at the given [`Path`]
    ///
    /// # Arguments
    /// * `local_location` - [`Path`] pointing to a location for which a [`Package`] should be registered.
    ///
    /// # Errors
    /// * [`RepoDiscoverError`] - when given [`Path`] does not point to a git repository
    fn get_package_by_local_location<P: AsRef<Path>>(
        &self,
        local_location: P,
    ) -> Result<Option<&Package>, NotAPackageError> {
        match infer_working_directory(local_location) {
            Ok(working_directory) => Ok(self
                .packages
                .iter()
                .find(|p| p.local_location == working_directory)),
            Err(e) => match e {
                RepositoryError::BareRepository => Err(NotAPackageError),
                RepositoryError::RepositoryDiscoveryFailed => Err(NotAPackageError),
            }
        }
    }

    //TODO: document
    pub fn get_package(&self, find_by: FindBy) -> Result<Option<&Package>, NotAPackageError> {
        match find_by {
            FindBy::LocalLocation(path) => self.get_package_by_local_location(path),
        }
    }

    /// Retrieves the [`StandaloneModule`] that is registered at the given [`Path`]
    ///
    /// # Arguments
    /// * `location` - [`Path`] pointing to a location for which a [`StandaloneModule`] should be registered.
    fn get_module_by_location<P: AsRef<Path>>(
        &self,
        location: P,
    ) -> Option<StandaloneModule> {
        self.modules.iter().find(|m|m.location == location.as_ref().to_path_buf()).cloned()
    }

    //TODO: document
    pub fn get_module(&self, find_by: FindBy) -> Option<StandaloneModule> {
        match find_by {
            FindBy::LocalLocation(path) => self.get_module_by_location(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::package::Package;
    use crate::registry::Registry;
    use git2::Repository;
    use std::collections::HashSet;
    use std::env;
    use url::Url;
    use crate::error::NotAPackageError;

    #[test]
    fn test_get_package_by_local_location() {
        let registry_path = env::temp_dir().join("registry.json");
        let mut registry = Registry {
            location: registry_path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };

        let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
        let package = Package::download(url, env::temp_dir()).unwrap();
        registry.add_package(package.clone()).unwrap();
        let found_package = registry
            .get_package_by_local_location(&package.local_location)
            .unwrap()
            .unwrap();
        assert_eq!(found_package, &package);
    }

    #[test]
    fn test_get_package_by_local_location_not_found() {
        let registry_path = env::temp_dir().join("registry.json");
        let package_path = env::temp_dir().join("unknown_package");
        assert!(Repository::init(&package_path).is_ok());

        let registry = Registry {
            location: registry_path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };

        let opt_package = registry
            .get_package_by_local_location(&package_path)
            .unwrap();
        assert!(opt_package.is_none());
    }

    #[test]
    fn test_get_package_by_local_location_panic_not_a_git_repo() {
        let registry_path = env::temp_dir().join("registry.json");
        let package_path = env::temp_dir().join("not_a_repository");
        let repo = Repository::discover(&package_path);

        assert!(repo.is_err());

        let registry = Registry {
            location: registry_path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };

        let err = registry
            .get_package_by_local_location(package_path)
            .unwrap_err();
        assert_eq!(err, NotAPackageError);
    }
}
