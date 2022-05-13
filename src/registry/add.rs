use crate::error::RegistryError;
use crate::module::standalone_module::StandaloneModule;
use crate::package::Package;
use crate::registry::Registry;

impl Registry {
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
    /// use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// let module_path = env::temp_dir().join("a.sac");
    /// # fs::write(&module_path, "hello");
    /// let module = StandaloneModule::create(module_path).unwrap();
    /// assert!(registry.is_empty());
    /// registry.add_module(module.clone());
    /// assert!(registry.contains_module(&module));
    /// ```
    /// Adding the same [`Package`] twice does not create duplicate entries
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use url::Url;
    /// use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// let module_path = env::temp_dir().join("a.sac");
    /// let module = StandaloneModule::create(module_path).unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add_module(module.clone());
    ///
    /// assert!(registry.contains_module(&module));
    /// assert_eq!(registry.count_modules(), 1);
    ///
    /// registry.add_module(module.clone());
    ///
    /// assert!(registry.contains_module(&module));
    /// assert_eq!(registry.count_modules(), 1);
    /// ```
    pub fn add_module(&mut self, module: StandaloneModule) -> Result<(), RegistryError> {
        self.modules.insert(module);
        self.save()
    }
}
