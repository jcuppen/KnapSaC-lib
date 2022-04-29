use crate::package::{create_package, Package};
use crate::utils::{discover_git_repository, infer_working_directory};
use serde::Deserialize;
use serde::Serialize;
use std::fs::write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
/// A [Registry] represents all [Package]s managed by the Package Manager.
pub struct Registry {
    pub(crate) packages: Vec<Package>,
}

impl Registry {
    fn find_by_local_location(&self, local_location: &PathBuf) -> Option<&Package> {
        let inferred_working_directory = infer_working_directory(&local_location);
        self.packages
            .iter()
            .find(|p| p.local_location == inferred_working_directory)
    }

    /// Searches the given [Path] for a package and checks whether
    /// it is contained in the [Registry]
    ///
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    pub fn contains(&self, local_location: &PathBuf) -> bool {
        self.find_by_local_location(local_location).is_some()
    }

    /// Check whether the number of packages in the [Registry] is zero
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Serializes the [Registry] to a JSON file located at the given [Path]
    ///
    /// # Examples
    /// ```
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("registry.json");
    ///
    /// registry.save(&path);
    /// ```
    ///
    /// This overwrites the file located at [Path]
    ///
    /// # Panics
    /// Panics when [Path] does not point to a JSON file
    /// ```rust,should_panic
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut path = temp_dir();
    /// path.push("registry.txt");
    ///
    /// let registry = initialize_registry();
    ///
    /// registry.save(&path);
    /// ```
    /// ```rust,should_panic
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut path = temp_dir();
    /// path.push("registry");
    ///
    /// let registry = initialize_registry();
    ///
    /// registry.save(&path);
    /// ```
    ///
    /// Panics when path is relative
    /// ```rust,should_panic
    /// use std::env::temp_dir;
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut path = temp_dir();
    /// path.push(".");
    /// path.push("nonexistent.json");
    ///
    /// assert!(!path.exists());
    ///
    /// let registry = initialize_registry();
    ///
    /// registry.save(&mut path);
    /// ```
    pub fn save(&self, path: &Path) {
        if let Some(ext) = path.extension() {
            if ext != "json" {
                panic!("Path does not point to a JSON file @ {}", path.display());
            }
        } else {
            panic!("Path does not point to a JSON file @ {}", path.display());
        }

        if path.is_relative() {
            panic!("Path is relative");
        }

        let contents = serde_json::to_string(self).unwrap();

        write(path, contents).unwrap()
    }

    /// Searches the given [Path] for a package and adds it to the [Registry]
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    ///
    /// # Registration Status
    /// Packages are marked by a [RegistrationStatus]
    /// This is either [Known] or [Registered]
    /// ## Known
    /// If a package does not have any remotes it is marked as [Known]
    /// ## Registered
    /// If a package has at least one git remote it is marked as [Registered]
    ///
    /// # Examples
    /// Adding a package that is marked as [Known].
    /// ```rust
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path: PathBuf = [
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "test_data",
    ///     "mock_package_known"
    /// ].iter().collect();
    /// let mut registry = initialize_registry();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    /// ```
    ///
    /// Adding a package that is marked as [Registered].
    /// ```rust
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path: PathBuf = [
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "test_data",
    ///     "mock_package_registered"
    /// ].iter().collect();
    /// let mut registry = initialize_registry();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    /// ```
    ///
    /// # Panics
    /// Panics when the discovered package already exists.
    /// ```rust,should_panic
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path: PathBuf = [
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "test_data",
    ///     "mock_package_known"
    /// ].iter().collect();
    ///
    /// let mut registry = initialize_registry();
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    ///
    /// registry.add(&path);
    /// ```
    pub fn add(&mut self, path: &PathBuf) {
        let local_repository_root = infer_working_directory(&path);

        if self.contains(&path) {
            panic!()
        }

        let repository = discover_git_repository(&path);

        self.packages
            .push(create_package(local_repository_root, repository));
    }

    // fn add_dependency(&self, local_location: &Path, value: String) {
    //     if let Some(package) = self.find_by_local_location(local_location) {
    //         package.add_dependency(value);
    //         return;
    //     }
    //     panic!("No package found @ {}", local_location.display());
    // }

    // fn remove_dependency(&self, local_location: &Path, value: String) {
    //     if let Some(package) = self.find_by_local_location(local_location) {
    //         package.remove_dependency(value);
    //         return;
    //     }
    //     panic!("No package found @ {}", local_location.display());
    // }

    /// Searches the given [Path] for a package and removes it from the [Registry]
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    ///
    /// # Examples
    /// ```
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path: PathBuf = [
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "test_data",
    ///     "mock_package_known"
    /// ].iter().collect();
    ///
    /// let mut registry = initialize_registry();
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    ///
    /// registry.remove(&path);
    ///
    /// assert!(registry.is_empty());
    /// ```
    /// If the registry does not contain the package found it does nothing.
    /// ```
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path: PathBuf = [
    ///     env!("CARGO_MANIFEST_DIR"),
    ///     "test_data",
    ///     "mock_package_known"
    /// ].iter().collect();
    ///
    /// let mut registry = initialize_registry();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.remove(&path);
    ///
    /// assert!(registry.is_empty());
    /// ```
    ///
    /// # Panics
    /// Panics if the given [Path] does not point to a valid git repository
    /// ```rust,should_panic
    /// use std::env::temp_dir;
    /// use std::path::PathBuf;
    /// use knapsac_lib::initialize_registry;
    /// let path = temp_dir();
    ///
    /// let mut registry = initialize_registry();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.remove(&path);
    /// ```
    pub fn remove(&mut self, local_location: &PathBuf) {
        let local_repository_root = infer_working_directory(local_location);
        self.packages
            .retain(|p| p.local_location != local_repository_root);
    }
}
