use crate::error::RegistryError;
use crate::module::standalone_module::StandaloneModule;
use crate::package::Package;
use crate::registry::Registry;

impl Registry {
    /// Removes a [`Package`] from the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `package` - A reference to a [`Package`] that needs to removed
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// assert!(registry.is_empty());
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url,env::temp_dir()).unwrap();
    /// registry.add_package(package.clone());
    /// assert!(registry.contains_package(&package));
    /// registry.remove_package(&package);
    /// assert!(registry.is_empty());
    /// ```
    /// If the [`Registry`] does not contain the [`Package`] referenced, it does nothing.
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
    /// let package = Package::download(url,env::temp_dir()).unwrap();
    /// registry.remove_package(&package);
    /// assert!(registry.is_empty());
    /// ```
    pub fn remove_package(&mut self, package: &Package) -> Result<(), RegistryError> {
        self.packages.remove(package);
        self.save()
    }

    /// Removes a [`StandaloneModule`] from the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `module` - A reference to a [`StandaloneModule`] that needs to removed
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// assert!(registry.is_empty());
    /// let module_path = env::temp_dir().join("a.sac");
    /// let module = StandaloneModule::create(module_path).unwrap();
    /// registry.add_module(module.clone());
    /// assert!(registry.contains_module(&module));
    /// registry.remove_module(&module);
    /// assert!(registry.is_empty());
    /// ```
    /// If the [`Registry`] does not contain the [`StandaloneModule`] referenced, it does nothing.
    /// ```
    /// # use std::env;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// assert!(registry.is_empty());
    /// let module_path = env::temp_dir().join("a.sac");
    /// let module = StandaloneModule::create(module_path).unwrap();
    /// registry.remove_module(&module);
    /// assert!(registry.is_empty());
    /// ```
    pub fn remove_module(&mut self, module: &StandaloneModule) -> Result<(), RegistryError> {
        self.modules.remove(module);
        self.save()
    }
}
