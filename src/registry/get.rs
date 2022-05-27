use crate::error::RegistryError;
use crate::module::standalone_module::StandaloneModule;
use crate::registry::Registry;
use std::path::PathBuf;

pub enum FindBy {
    OutputLocation(PathBuf),
    Identifier(String),
}

impl Registry {
    /*
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
            },
        }
    }

    //TODO: document
    pub fn get_package(&self, find_by: FindBy) -> Result<Option<&Package>, NotAPackageError> {
        match find_by {
            FindBy::Location(path) => self.get_package_by_local_location(path),
        }
    }
    */

    /// Retrieves the [`StandaloneModule`] that is registered at the given [`Path`]
    ///
    /// # Arguments
    /// * `location` - [`Path`] pointing to a location for which a [`StandaloneModule`] should be registered
    fn get_module_by_output_location(&self, output_location: PathBuf) -> Option<StandaloneModule> {
        self.modules
            .iter()
            .find(|(_,m)| m.output_location == output_location).map(|(_,m)|m).cloned()
    }

    /// Retrieves the [`StandaloneModule`] that is registered at the given [`Path`]
    ///
    /// # Arguments
    /// * `location` - [`Path`] pointing to a location for which a [`StandaloneModule`] should be registered
    fn get_module_by_identifier(&self, identifier: &String) -> Option<StandaloneModule> {
        self.modules.get(identifier).cloned()
    }

    /// Retrieves the [`StandaloneModule`] that is matches the given [`FindBy`]
    ///
    /// # Arguments
    /// * `find_by` - [`FindBy`] object representing how to look for a [`StandaloneModule`]
    ///
    /// TODO: write docs
    pub fn get_module(&self, find_by: FindBy) -> Result<Option<StandaloneModule>, RegistryError> {
        match find_by {
            FindBy::OutputLocation(output_location) => {
                Ok(self.get_module_by_output_location(output_location))
            }
            FindBy::Identifier(id) => {
                Ok(self.get_module_by_identifier(&id))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /*
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
    */

    /*
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

     */
    /*

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

    */
}
