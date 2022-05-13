pub mod get;
mod add;
mod remove;

use crate::package::Package;
use crate::error::RegistryError;
use crate::error::RegistryError::{
    InvalidRegistry, NoRegistryFound, RegistryPathNotAbsolute, RegistryPathNotFile,
    RegistryPathNotJSON,
};
use crate::module::standalone_module::StandaloneModule;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// A [`Registry`] represents all [`Package`]s and [`Module`]s managed by KnapSaC
pub struct Registry {
    #[serde(skip)]
    pub(crate) location: PathBuf,
    pub(crate) packages: HashSet<Package>,
    pub(crate) modules: HashSet<StandaloneModule>,
}

impl Registry {
    /// Creates a new empty [`Registry`] and writes it to the given [`Path`]
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("registry.json");
    /// let registry = Registry::initialize(path).unwrap();
    /// # assert!(registry.is_empty());
    /// ```
    pub fn initialize<P: AsRef<Path>>(path: P) -> Result<Self, RegistryError> {
        let registry = Registry {
            location: path.as_ref().to_path_buf(),
            packages: HashSet::new(),
            modules: HashSet::new(),
        };
        registry.save()?;
        Ok(registry)
    }

    /// Loads and returns a [`Registry`] based on the given [`Path`]
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("registry.json");
    /// fs::write(&path, "{\"packages\": [], \"modules\": []}").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let registry = Registry::load(path).unwrap();
    /// assert!(registry.is_empty())
    /// ```
    ///
    /// # Errors
    /// Returns a [`NoRegistryFound`] error when there is no file at given [`Path`]
    /// ```
    /// # use std::env;
    /// # use knapsac_lib::error::RegistryError::NoRegistryFound;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("nonexistent.json");
    /// assert!(!path.exists());
    /// let err = Registry::load(path).unwrap_err();
    /// assert_eq!(err, NoRegistryFound)
    /// ```
    /// Returns an [`InvalidRegistry`] when the given JSON file is not valid JSON
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::error::RegistryError::{InvalidRegistry};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("invalid.json");
    /// fs::write(&path, "{").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let contents = fs::read_to_string(&path);
    /// # assert_eq!(contents.unwrap(), String::from("{"));
    /// let err = Registry::load(path).unwrap_err();
    /// assert_eq!(err, InvalidRegistry)
    /// ```
    /// Returns an [`InvalidRegistry`] error when JSON cannot be parsed to a valid [`Registry`]
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::error::RegistryError::InvalidRegistry;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("invalid.json");
    /// fs::write(&path, "{ \"packages\": 12, \"modules\": []}").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// # let contents = fs::read_to_string(&path);
    /// let err = Registry::load(path).unwrap_err();
    /// assert_eq!(err, InvalidRegistry)
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, RegistryError> {
        if let Ok(data) = read_to_string(&path) {
            let res = serde_json::from_str(data.as_str());
            if res.is_err() {
                return Err(InvalidRegistry);
            }
            let mut registry: Registry = res.unwrap();
            registry.location = path.as_ref().to_path_buf();
            return Ok(registry);
        }
        Err(NoRegistryFound)
    }

    // TODO write docs
    pub fn search_by_module_identifiers(&self, module_identifiers: &[String]) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|p| {
                match p.has_modules_with_identifiers(module_identifiers) {
                    Ok(b) => b,
                    Err(_e) => panic!("Failed to load package manifest.")
                }
            })
            .collect()
    }

    /// Checks if the [`Registry`] contains a certain [`Package`]
    ///
    /// # Arguments
    /// * `package` - A reference to a [`Package`] that needs to be checked
    pub fn contains_package(&self, package: &Package) -> bool {
        self.packages.contains(package)
    }

    /// Checks if the [`Registry`] contains a certain [`StandaloneModule`]
    ///
    /// # Arguments
    /// * `package` - A reference to a [`StandaloneModule`] that needs to be checked
    pub fn contains_module(&self, module: &StandaloneModule) -> bool {
        self.modules.contains(module)
    }

    /// Returns how many [`Package`]s are in the [`Registry`]
    pub fn count_packages(&self) -> usize {
        self.packages.len()
    }

    /// Returns how many [`StandaloneModule`]s are in the [`Registry`]
    pub fn count_modules(&self) -> usize {
        self.modules.len()
    }

    /// Check whether the total number of items registered in the [`Registry`] is 0
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty() && self.modules.is_empty()
    }

    /// Serializes the [`Registry`] to a JSON file located at the [`Registry`]'s `location`
    /// This overwrites the file located at that location
    ///
    /// # Errors
    /// Returns a [`RegistryPathNotAbsolute`] error when path is relative
    /// Returns a [`RegistryPathNotFile`] error when path is a directory
    /// Returns a [`RegistryPathNotJSON`] error when file pointed to is not a JSON file
    pub(crate) fn save(&self) -> Result<(), RegistryError> {
        let path = self.location.to_path_buf();

        if path.is_relative() {
            return Err(RegistryPathNotAbsolute);
        }

        if let Some(ext) = path.extension() {
            if ext != "json" {
                return Err(RegistryPathNotJSON);
            }
        } else {
            return Err(RegistryPathNotFile);
        }

        let contents = serde_json::to_string(self).unwrap();
        write(path, contents).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RegistryError::{
        RegistryPathNotAbsolute, RegistryPathNotFile, RegistryPathNotJSON,
    };
    use crate::registry::Registry;
    use std::collections::HashSet;
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn test_save() {
        let path = env::temp_dir().join("registry.json");
        let res = fs::remove_file(&path);

        assert!(res.is_ok());

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };
        assert!(registry.save().is_ok());
    }

    #[test]
    /// When [`Registry`]'s `location` points to an existing file, overwrite it.
    fn test_save_overwrite() {
        let path = env::temp_dir().join("registry.json");
        let res = fs::write(&path, "hello");

        assert!(res.is_ok());
        assert!(path.is_file());

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };

        assert!(registry.save().is_ok());
    }

    #[test]
    /// Should panic when [`Registry`]´s `location` does not point to a JSON file
    fn test_save_panic_not_json() {
        let path = env::temp_dir().join("registry.txt");

        fs::write(&path, "test").unwrap();

        assert!(path.exists());

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };
        assert_eq!(registry.save().err(), Some(RegistryPathNotJSON));
    }

    #[test]
    /// Should panic when [`Registry`]´s `location` points to a directory
    fn test_save_panic_is_a_file() {
        let path = env::temp_dir().join("registry");

        fs::create_dir_all(&path).unwrap();

        assert!(path.is_dir());

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };
        let res = registry.save();
        assert_eq!(res.err(), Some(RegistryPathNotFile));
    }

    #[test]
    /// Should panic when [`Registry`]'s `location` is a relative [`Path`]
    fn test_save_panic_is_relative() {
        let path = PathBuf::from("./registry.json");

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
            modules: HashSet::new(),
        };
        assert_eq!(registry.save().err(), Some(RegistryPathNotAbsolute));
    }
}
