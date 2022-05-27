mod add;
pub mod get;
mod remove;

use crate::error::RegistryError;
use crate::error::RegistryError::{
    InvalidRegistry, RegistryPathNotAbsolute, RegistryPathNotFile,
    RegistryPathNotJSON,
};
use crate::module::standalone_module::StandaloneModule;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};


#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// A [`Registry`] represents all [`Package`]s and [`Module`]s managed by KnapSaC
pub struct Registry {
    #[serde(skip)]
    pub(crate) location: PathBuf,
    pub(crate) modules: HashMap<String, StandaloneModule>,
}

impl Registry {
    // Private Functions



    // Crate Public Functions

    /// Serializes the [`Registry`] to a JSON file located at the [`Registry`]'s `location`
    /// This overwrites the file located at that location
    ///
    /// # Errors
    /// Returns a [`RegistryPathNotAbsolute`] error when path is relative
    /// Returns a [`RegistryPathNotFile`] error when path is a directory
    /// Returns a [`RegistryPathNotJSON`] error when file pointed to is not a JSON file
    pub(crate) fn save(&self) -> Result<(), RegistryError> {
        println!("D: {}", self.location.display());
        if self.location.is_relative() {
            return Err(RegistryPathNotAbsolute);
        }

        if let Some(ext) = self.location.extension() {
            if ext != "json" {
                return Err(RegistryPathNotJSON);
            }
        } else {
            return Err(RegistryPathNotFile);
        }

        let contents = serde_json::to_string(self).unwrap();
        write(&self.location, contents).unwrap();
        Ok(())
    }

    // Public Functions

    /// Creates a new empty [`Registry`] and writes it to the given [`Path`]
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let registry = Registry::initialize(env::temp_dir().join("registry.json")).unwrap();
    /// # assert!(registry.is_empty());
    /// ```
    pub fn initialize<P: AsRef<Path>>(path: P) -> Result<Self, RegistryError> {
        let registry = Registry {
            modules: HashMap::new(),
            location: path.as_ref().to_path_buf(),
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
    /// # let path = env::temp_dir().join("registry.json");
    /// fs::write(&path, "{\"modules\": {}}").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let registry = Registry::load(&path).unwrap();
    /// assert!(registry.is_empty())
    /// ```
    ///
    /// # Errors
    /// Returns an [`InvalidRegistry`] when the given JSON file is not valid JSON
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::error::RegistryError::{InvalidRegistry};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// # let path = env::temp_dir().join("invalid_json.json");
    /// fs::write(&path, "{").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let err = Registry::load(&path).unwrap_err();
    /// assert_eq!(err, InvalidRegistry)
    /// ```
    /// Returns an [`InvalidRegistry`] error when JSON cannot be parsed to a valid [`Registry`]
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::error::RegistryError::InvalidRegistry;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// # let path = env::temp_dir().join("invalid_registry.json");
    /// fs::write(&path, "{ \"packages\": 12, \"modules\": []}").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let err = Registry::load(&path).unwrap_err();
    /// assert_eq!(err, InvalidRegistry)
    /// ```
    pub fn load(path: &Path) -> Result<Self, RegistryError> {
        if let Ok(data) = read_to_string(&path) {
            let res = serde_json::from_str(data.as_str());
            if res.is_err() {
                return Err(InvalidRegistry);
            }
            let registry: Registry = res.unwrap();
            return Ok(registry)
        }
        Registry::initialize(path)
    }

    /// Check whether the total number of items registered in the [`Registry`] is 0
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::Registry;
    use std::collections::{HashMap};
    use std::{env, fs};
    use std::path::PathBuf;
    use crate::error::RegistryError::{RegistryPathNotAbsolute, RegistryPathNotFile, RegistryPathNotJSON};


    #[test]
    fn test_save() {
        let path = env::temp_dir().join("registry.json");
        let res = fs::remove_file(&path);

        assert!(res.is_ok());

        let registry = Registry {
            // packages: HashSet::new(),
            modules: HashMap::new(),
            location: path,
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
            // packages: HashSet::new(),
            modules: HashMap::new(),
            location: path,
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
            // packages: HashSet::new(),
            modules: HashMap::new(),
            location: path,
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
            // packages: HashSet::new(),
            modules: HashMap::new(),
            location: path,
        };
        assert_eq!(registry.save().err(), Some(RegistryPathNotFile));
    }

    #[test]
    /// Should panic when [`Registry`]'s `location` is a relative [`Path`]
    fn test_save_panic_is_relative() {
        let path = PathBuf::from("./registry.json");

        let registry = Registry {
            location: path,
            // packages: HashSet::new(),
            modules: HashMap::new(),
        };
        assert_eq!(registry.save().err(), Some(RegistryPathNotAbsolute));
    }
}
