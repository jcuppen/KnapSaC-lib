use crate::error::RegistryError;
use crate::error::RegistryError::ModuleAlreadyInRegistry;
// use crate::error::RegistryError::ModuleAlreadyInRegistry;
use crate::module::standalone_module::StandaloneModule;
use crate::registry::Registry;

impl Registry {
    // Private Functions

    // Crate Public Functions

    // Public Functions

    /*
    /// Adds a [`Package`] to the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `package` - A [`Package`] that needs to be added
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// assert!(registry.is_empty());
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// registry.add_package(package.clone());
    /// assert!(registry.contains_package(&package));
    /// ```
    /// Adding the same [`Package`] twice does not create duplicate entries
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add_package(package.clone());
    ///
    /// assert!(registry.contains_package(&package));
    /// assert_eq!(registry.count_packages(), 1);
    ///
    /// registry.add_package(package.clone());
    ///
    /// assert!(registry.contains_package(&package));
    /// assert_eq!(registry.count_packages(), 1);
    /// ```
    pub fn add_package(&mut self, package: Package) -> Result<(), RegistryError> {
        self.packages.insert(package);
        self.save()
    }
    */

    /// Adds a [`StandaloneModule`] to the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `module` - A [`StandaloneModule`] that needs to be added
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::get::FindBy;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_out_path = env::temp_dir().join("a");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// let module = StandaloneModule::create("a", &module_out_path).unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add_module(module.clone()).unwrap();
    /// let found_module = registry.get_module(FindBy::OutputLocation(module_out_path)).unwrap().unwrap();
    ///
    /// assert_eq!(found_module, module);
    /// ```
    /// # Errors
    /// Returns a [`ModuleAlreadyInRegistry`] error when a module is being added where the path matches an already registered [`StandaloneModule`]
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::error::RegistryError::ModuleAlreadyInRegistry;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::get::FindBy;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_out_path = env::temp_dir().join("a");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// let module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// let other_module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// assert!(registry.is_empty());
    ///
    /// registry.add_module(module.clone()).unwrap();
    /// let found_module = registry.get_module(FindBy::OutputLocation(module_out_path)).unwrap().unwrap();
    ///
    /// assert_eq!(found_module, module);
    ///
    /// let err = registry.add_module(other_module).unwrap_err();
    ///
    /// assert_eq!(err, ModuleAlreadyInRegistry);
    /// ```
    pub fn add_module(&mut self, module: StandaloneModule) -> Result<(), RegistryError> {
        if self.modules.iter().any(|(i, _)| i == &module.identifier) {
            return Err(ModuleAlreadyInRegistry);
        }
        self.modules.insert(module.identifier.clone(), module);
        self.save()
    }
}
