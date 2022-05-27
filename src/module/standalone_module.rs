use crate::error::ModuleError::{NoSuchDependency, OutputLocationDoesNotExist, OutputLocationNotAbsolute, OutputLocationNotADirectory, RegistryError};
use crate::error::ModuleError::{CyclicDependency};
use crate::module::Module;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::vec;
use crate::error::{ModuleError};
use crate::registry::get::FindBy;
use crate::registry::Registry;
use crate::REGISTRY_PATH;

#[derive(Deserialize, Serialize, Hash, Eq, PartialEq, Clone, Debug)]
pub struct StandaloneModule {
    pub identifier: String,
    pub output_location: PathBuf,
    pub module_dependencies: Vec<String>,
}

impl StandaloneModule {
    // # Private Functions

    // fn load_manifest(&self) -> Result<Manifest, ModuleError> {
    //     match Manifest::load(self.manifest_location()?) {
    //         Ok(m) => Ok(m),
    //         Err(e) => match e {
    //             ManifestError::InvalidManifest => Err(InvalidManifest),
    //         },
    //     }
    // }

    // fn manifest_location(&self) -> Result<PathBuf, ModuleError> {
    //     let mut path = self.source_location.to_path_buf();
    //     let file_stem = path
    //         .file_stem()
    //         .unwrap_or_else(|| panic!("Module location does not point to a file"))
    //         .to_os_string();
    //     path = PathBuf::from(
    //         path.parent()
    //             .unwrap_or_else(|| panic!("Module location does not have a parent directory")),
    //     );
    //     path.push(file_stem);
    //     path.set_extension("knapsac");
    //     Ok(path)
    // }

    fn dependency_tree_contains_module(
        &self,
        module: &StandaloneModule,
    ) -> Result<bool, ModuleError> {
        if self.identifier == module.identifier {
            return Ok(true)
        }

        let r = Registry::load(Path::new(REGISTRY_PATH)).unwrap();
        for id in &self.module_dependencies {
            let module = r.get_module(FindBy::Identifier(id.to_string()));
            match module {
                Ok(Some(m)) => {
                    if self.dependency_tree_contains_module(&m)? {
                        return Ok(true)
                    }
                }
                Ok(None) => {}
                Err(e) => return Err(RegistryError(e)),
            }
        }
        Ok(false)
    }

    // # Crate Public Functions

    // # Public Functions

    /// Creates a new [`StandaloneModule`] based on the given [`Path`]
    ///
    /// TODO: fix args
    /// # Arguments
    /// * `path` - An absolute [`Path`] that points to a module file
    /// * `identifier` - the identifier of the [`StandaloneModule`], defaults to the file stem
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    ///
    /// # let module_out_path = env::temp_dir().join("a");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// # assert!(module_out_path.exists());
    /// let module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// assert_eq!(module.identifier, "a");
    /// ```
    /// # Errors
    /// Returns a [`OutputLocationDoesNotExist`] error when [`Path`] does not point to and existing file
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::error::ModuleError::{OutputLocationDoesNotExist};
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    ///
    /// let module_out_path = env::temp_dir().join("nonexistent");
    /// let err = StandaloneModule::create("nonexistent", &module_out_path).unwrap_err();
    /// assert_eq!(err, OutputLocationDoesNotExist);
    /// ```
    /// Returns a [`OutputLocationNotAbsolute`] error when [`Path`] is not absolute
    /// ```
    /// # use std::{env, fs};
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::error::ModuleError::{OutputLocationNotAbsolute};
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    ///
    /// let module_out_path: PathBuf = ["src", "a"].iter().collect();
    /// let err = StandaloneModule::create("a", &module_out_path).unwrap_err();
    /// assert_eq!(err, OutputLocationNotAbsolute);
    /// ```
    /// TODO: add not dir error
    pub fn create(
        identifier: &str,
        output_location: &Path,
    ) -> Result<Self, ModuleError> {
        if output_location.is_relative() {
            return Err(OutputLocationNotAbsolute);
        }
        if !output_location.exists() {
            return Err(OutputLocationDoesNotExist);
        }
        if !output_location.exists() {
            return Err(OutputLocationNotADirectory);
        }

        Ok(StandaloneModule {
            identifier: identifier.to_string(),
            output_location: output_location.to_path_buf(),
            module_dependencies: vec![],
        })
    }

    /// Adds a [`StandaloneModule`] as a dependency to another [`StandaloneModule`]
    ///
    /// # Arguments
    /// `dependency` - A [`StandaloneModule`] that needs to be added as dependency
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_out_path = env::temp_dir().join("a");
    /// # let dependency_out_path = env::temp_dir().join("b");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// # if !dependency_out_path.exists() { fs::create_dir_all(&dependency_out_path).unwrap()}
    /// let mut module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// let dependency = StandaloneModule::create("b", &dependency_out_path).unwrap();
    ///
    /// registry.add_module(module.clone()).unwrap();
    /// registry.add_module(dependency.clone()).unwrap();
    ///
    /// module.add_module_dependency(&dependency.identifier).unwrap();
    /// assert!(module.has_module_dependency(&dependency.identifier).unwrap());
    /// ```
    ///
    /// # Errors
    /// Returns a [`CyclicDependency`] error when a cyclic dependency would be created
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::error::ModuleError::CyclicDependency;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_a_out_path = env::temp_dir().join("a");
    /// # let module_b_out_path = env::temp_dir().join("b");
    /// # if !module_a_out_path.exists() { fs::create_dir_all(&module_a_out_path).unwrap()}
    /// # if !module_b_out_path.exists() { fs::create_dir_all(&module_b_out_path).unwrap()}
    /// let mut module_a = StandaloneModule::create("a", &module_a_out_path).unwrap();
    /// let mut module_b = StandaloneModule::create("b", &module_b_out_path).unwrap();
    ///
    /// registry.add_module(module_a.clone()).unwrap();
    /// registry.add_module(module_b.clone()).unwrap();
    ///
    /// module_a.add_module_dependency("b").unwrap();
    ///
    /// let err = module_b.add_module_dependency("a").unwrap_err();
    /// assert_eq!(CyclicDependency, err);
    /// ```
    pub fn add_module_dependency(&mut self, dependency: &str) -> Result<(), ModuleError> {
        let r = Registry::load(Path::new(REGISTRY_PATH)).unwrap();
        let res = match r.get_module(FindBy::Identifier(dependency.to_string())) {
            Ok(None) => Err(NoSuchDependency),
            Ok(Some(d)) => Ok(d),
            Err(e) => Err(RegistryError(e)),
        };

        if res?.dependency_tree_contains_module(self)? {
            return Err(CyclicDependency);
        }

        match self.has_module_dependency(dependency) {
            Ok(true) => return Ok(()),
            Ok(false) => {},
            Err(e) => return Err(e),
        }

        self.module_dependencies.push(dependency.to_string());
        Ok(())
    }

    /// Checks the [`StandaloneModule`] if it depends on the given [`StandaloneModule`]
    ///
    /// # Arguments
    /// * `dependency` - A reference to a [`StandaloneModule`] that needs to be checked
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_out_path = env::temp_dir().join("a");
    /// # let dependency_out_path = env::temp_dir().join("b");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// # if !dependency_out_path.exists() { fs::create_dir_all(&dependency_out_path).unwrap()}
    /// let mut module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// let dependency = StandaloneModule::create("b", &dependency_out_path).unwrap();
    ///
    /// registry.add_module(module.clone()).unwrap();
    /// registry.add_module(dependency).unwrap();
    ///
    /// module.add_module_dependency(&dependency.identifier).unwrap();
    /// assert!(module.has_module_dependency(&dependency.identifier).unwrap());
    /// ```
    pub fn has_module_dependency(
        &self,
        identifier: &str,
    ) -> Result<bool, ModuleError> {
        Ok(self.module_dependencies.contains(&identifier.to_string()))
    }

    /// Checks the [`StandaloneModule`] if it depends on the given [`StandaloneModule`]
    ///
    /// # Arguments
    /// * `dependency` - A reference to a [`StandaloneModule`] that needs to be checked
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::package::Package;
    /// use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(&env::temp_dir().join("registry.json")).unwrap();
    /// # let module_out_path = env::temp_dir().join("a");
    /// # let dependency_out_path = env::temp_dir().join("b");
    /// # if !module_out_path.exists() { fs::create_dir_all(&module_out_path).unwrap()}
    /// # if !dependency_out_path.exists() { fs::create_dir_all(&dependency_out_path).unwrap()}
    /// let mut module = StandaloneModule::create("a", &module_out_path).unwrap();
    /// let dependency = StandaloneModule::create("b", &dependency_out_path).unwrap();
    ///
    /// registry.add_module(module.clone()).unwrap();
    /// registry.add_module(dependency.clone()).unwrap();
    ///
    /// module.add_module_dependency(&dependency.identifier).unwrap();
    ///
    /// let found_module = module.get_module_dependency(&dependency.identifier).unwrap().unwrap();
    /// assert_eq!(found_module, dependency);
    /// ```
    pub fn get_module_dependency(
        &self,
        identifier: &str,
    ) -> Result<Option<StandaloneModule>, ModuleError> {
        let r = Registry::load(Path::new(REGISTRY_PATH)).unwrap();
        let i = match r.get_module(FindBy::Identifier(identifier.to_string())) {
            Ok(dep) => Ok(dep),
            Err(e) => Err(ModuleError::RegistryError(e))
        };
        if self.module_dependencies.contains(&identifier.to_string()) {
            return i;
        }
        Err(NoSuchDependency)
    }
}

impl Module for StandaloneModule {}
