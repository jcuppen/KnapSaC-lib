use crate::package::Package;
use crate::utils::infer_working_directory;

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(PartialEq)]
/// A [`Registry`] represents all [`Package`]s managed by KnapSaC
pub struct Registry {
    #[serde(skip)]
    pub(crate) location: PathBuf,
    pub(crate) packages: HashSet<Package>,
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
    /// # assert!(Registry::initialize(path).is_empty())
    /// ```
    pub fn initialize<P: AsRef<Path>>(path: P) -> Self {
        let registry = Registry {
            location: path.as_ref().to_path_buf(),
            packages: HashSet::new()
        };
        registry.save().unwrap();
        registry
    }

    /// Loads and returns a [`Registry`] based on the given [`Path`]
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("registry.json");
    /// fs::write(&path, "{\"packages\": []}").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let registry = Registry::load(path);
    /// assert!(registry.is_empty())
    /// ```
    ///
    /// # Panics
    /// Panics when there is no file at given [`Path`]
    /// ```rust, should_panic
    /// # use std::env;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("nonexistent.json");
    /// assert!(!path.exists());
    /// let registry = Registry::load(path);
    /// ```
    /// Panics when there is no JSON file at given [`Path`]
    /// ```rust, should_panic
    /// # use std::env;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("registry.txt");
    /// assert!(path.exists());
    /// assert!(path.is_file());
    /// let registry = Registry::load(path);
    /// ```
    /// Panics when the given JSON file is not valid JSON
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("invalid.json");
    /// fs::write(&path, "{").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// let contents = fs::read_to_string(&path);
    /// # assert_eq!(contents.unwrap(), String::from("{"));
    /// let registry = Registry::load(path);
    /// ```
    /// Panics when JSON cannot be parsed to a valid [`Registry`]
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let path = env::temp_dir().join("invalid.json");
    /// fs::write(&path, "{ \"packages\": 12 }").unwrap();
    /// # assert!(path.exists());
    /// # assert!(path.is_file());
    /// # let contents = fs::read_to_string(&path);
    /// # assert_eq!(contents.unwrap(), String::from("{ \"packages\": 12 }"));
    /// let registry = Registry::load(path);
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(data) = read_to_string(&path) {
            let mut registry: Registry = serde_json::from_str(data.as_str()).unwrap();
            registry.location = path.as_ref().to_path_buf();
            return registry
        }
        panic!("No registry found @ {}", path.as_ref().display())
    }

    /// Retrieves the [`Package`] that is registered at the given [`Path`]
    ///
    /// # Arguments
    /// * `local_location` - [`Path`] pointing to a location for which a [`Package`] should be registered.
    ///
    /// # Examples
    /// An example of a [`Package`] being found
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// # assert!(package_path.is_dir());
    /// let package = Package::create(&package_path);
    /// registry.add(package.clone());
    /// assert_eq!(registry.get_by_local_location(package_path), Some(&package));
    /// ```
    /// An example of a [`Package`] not being found
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// let package = Package::create(&package_path);
    /// assert!(registry.is_empty());
    /// assert!(registry.get_by_local_location(&package_path).is_none());
    /// ```
    /// # Panics
    /// Panics when given [`Path`] does not point to a git repository
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("not_a_repository");
    /// fs::remove_dir_all(&package_path);
    /// assert!(!package_path.exists());
    /// assert!(registry.is_empty());
    /// assert!(registry.get_by_local_location(&package_path).is_none());
    /// ```
    pub fn get_by_local_location<P: AsRef<Path>>(&self, local_location: P) -> Option<&Package> {
        let inferred_working_directory = infer_working_directory(local_location);
        self.packages.iter().find(|p|p.local_location == inferred_working_directory)
    }

    pub fn search_by_module_identifiers(&self, module_identifiers: &[String]) -> Vec<&Package> {
        self.packages.iter().filter(|p|p.has_modules_with_identifiers(module_identifiers)).collect()
    }

    /// Checks if the [`Registry`] contains a certain [`Package`]
    ///
    /// # Arguments
    /// * `package` - A reference to a [`Package`] that needs to be checked
    pub fn contains(&self, package: &Package) -> bool {
        self.packages.contains(package)
    }

    /// Returns how many [`Package`]s are in the [`Registry`]
    pub fn count_packages(&self) -> usize {
        self.packages.len()
    }

    /// Check whether the number of packages in the [`Registry`] is 0
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Adds a [`Package`] to the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `package` - A [`Package`] that needs to be added
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// let package = Package::create(&package_path);
    /// assert!(registry.is_empty());
    /// registry.add(package.clone());
    /// assert!(registry.contains(&package));
    /// ```
    /// Adding the same [`Package`] twice does not create duplicate entries
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// # assert!(package_path.is_dir());
    /// let package = Package::create(package_path);
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add(package.clone());
    ///
    /// assert!(registry.contains(&package));
    /// assert_eq!(registry.count_packages(), 1);
    ///
    /// registry.add(package.clone());
    ///
    /// assert!(registry.contains(&package));
    /// assert_eq!(registry.count_packages(), 1);
    /// ```
    pub fn add(&mut self, package: Package) {
        self.packages.insert(package);
        self.save().unwrap();
    }

    /// Removes a [`Package`] from the [`Registry`] and saves the [`Registry`]
    ///
    /// # Arguments
    /// * `package` - A reference to a [`Package`] that needs to removed
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    ///
    /// assert!(package_path.exists());
    ///
    /// let package = Package::create(package_path);
    /// registry.add(package.clone());
    ///
    /// assert!(registry.contains(&package));
    ///
    /// registry.remove(&package);
    ///
    /// assert!(registry.is_empty());
    /// ```
    /// If the [`Registry`] does not contain the [`Package`] referenced, it does nothing.
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::registry::Registry;
    ///
    /// let mut registry = Registry::initialize(env::temp_dir().join("registry.json"));
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    ///
    /// assert!(package_path.is_dir());
    /// assert!(registry.is_empty());
    ///
    /// let package = Package::create(package_path);
    /// registry.remove(&package);
    ///
    /// assert!(registry.is_empty());
    /// ```
    pub fn remove(&mut self, package: &Package) {
        self.packages.remove(package);
        self.save().unwrap();
    }

    /// Serializes the [`Registry`] to a JSON file located at the [`Registry`]'s `location`
    /// This overwrites the file located at that location
    pub(crate) fn save(&self) -> Result<(), &str> {
        let path = self.location.to_path_buf();

        if path.is_relative() {
            return Err("Path is relative")
        }

        if let Some(ext) = path.extension() {
            if ext != "json" {
                return Err("Path does not point to a JSON file")
            }
        } else {
            return Err("Path does not point to a file")
        }

        let contents = serde_json::to_string(self).unwrap();

        write(path, contents).unwrap();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::{env, fs};
    use std::path::PathBuf;
    use crate::registry::Registry;

    #[test]
    fn test_save() {
        let path = env::temp_dir().join("registry.json");

        let res = fs::remove_file(&path);

        assert!(res.is_ok());

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
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
        };
        assert_eq!(registry.save().err(), Some("Path does not point to a JSON file"));
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
        };
        let res = registry.save();
        assert_eq!(res.err(), Some("Path does not point to a file"));
    }

    #[test]
    /// Should panic when [`Registry`]'s `location` is a relative [`Path`]
    fn test_save_panic_is_relative() {
        let path = PathBuf::from("./registry.json");

        let registry = Registry {
            location: path,
            packages: HashSet::new(),
        };
        assert_eq!(registry.save().err(), Some("Path is relative"));
    }
}
