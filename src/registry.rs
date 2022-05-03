use crate::package::{create_package, Package};
use crate::utils::{discover_git_repository, infer_working_directory};
use crate::dependency::Dependency;
use crate::package::RegistrationStatus::{Known, Registered};

use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::Path;
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
#[derive(PartialEq)]
/// A [Registry] represents all [Package]s managed by the Package Manager.
pub struct Registry {
    pub(crate) packages: Vec<Package>,
}

impl Registry {
    fn find_by_local_location<P: AsRef<Path>>(&self, local_location: P) -> Option<&Package> {
        let inferred_working_directory = infer_working_directory(&local_location);
        self.packages
            .iter()
            .find(|p| p.local_location == inferred_working_directory)
    }

    /// Searches the given Path for a package and checks whether it is considered a 'registered' package
    ///
    /// A package is considered [Registered] when it has at least one git remote
    /// # Examples
    /// ```rust
    /// use std::env::temp_dir;
    /// use git2::Repository;
    /// use url::Url;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("mock_packages_registered");
    /// let url = Url::parse("https://github.com/jcuppen/mock-package-registered.git").unwrap();
    /// let repo = Repository::init(&path).unwrap();
    /// repo.remote("origin", url.to_string().as_str());
    ///
    /// assert!(path.is_dir());
    /// assert!(registry.is_empty());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.is_registered(&path));
    /// ```
    pub fn is_registered<P: AsRef<Path>>(&self, local_location: P) -> bool {
        if let Some(package) = self.find_by_local_location(local_location) {
            return package.registration_status == Registered;
        }
        panic!()
    }

    /// Searches the given [Path] for a package and checks whether it is considered a 'known' package
    ///
    /// A package is considered [Known] when it has no git remotes
    ///
    /// # Examples
    /// ```rust
    /// use std::env::temp_dir;
    /// use git2::Repository;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    /// let repo = Repository::init(&path);
    /// let mut registry = initialize_registry();
    ///
    /// assert!(repo.is_ok());
    /// assert!(path.is_dir());
    /// assert!(registry.is_empty());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.is_known(&path));
    /// ```
    pub fn is_known<P: AsRef<Path>>(&self, local_location: P) -> bool {
        if let Some(package) = self.find_by_local_location(local_location) {
            return package.registration_status == Known;
        }
        panic!()
    }

    /// Searches the given [Path] for a package and checks whether
    /// it is contained in the [Registry]
    ///
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    pub fn contains<P: AsRef<Path>>(&self, local_location: P) -> bool {
        self.find_by_local_location(local_location).is_some()
    }

    /// Check whether the number of packages in the [Registry] is 0
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Serializes the [Registry] to a JSON file located at the given [Path]
    ///
    /// # Examples
    /// ```
    /// use std::env::temp_dir;
    /// use knapsac_lib::{initialize_registry, load_registry};
    ///
    /// let registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("registry.json");
    ///
    /// registry.save(&path);
    ///
    /// assert!(path.exists());
    /// assert_eq!(registry, load_registry(path));
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
    /// let registry = initialize_registry();
    ///
    /// registry.save(&path);
    /// ```
    ///
    /// Panics when path is relative
    /// ```rust,should_panic
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("dir");
    /// path.push("..");
    /// path.push("nonexistent.json");
    ///
    /// assert!(path.exists());
    /// assert!(path.is_relative());
    ///
    /// registry.save(path);
    /// ```
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Some(ext) = path.as_ref().extension() {
            if ext != "json" {
                panic!("Path does not point to a JSON file @ {}", path.as_ref().display());
            }
        } else {
            panic!("Path does not point to a JSON file @ {}", path.as_ref().display());
        }

        if path.as_ref().is_relative() {
            panic!("Path is relative");
        }

        let contents = serde_json::to_string(self).unwrap();

        write(path, contents).unwrap()
    }

    /// Searches the given [Path] for a package and adds it to the [Registry]
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    ///
    /// # Examples
    /// ```rust
    /// use std::env::temp_dir;
    /// use git2::Repository;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    /// let mut registry = initialize_registry();
    /// let repo = Repository::init(&path);
    ///
    /// assert!(repo.is_ok());
    /// assert!(path.is_dir());
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
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    ///
    /// assert!(path.is_dir());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    ///
    /// registry.add(&path);
    /// ```
    pub fn add<P: AsRef<Path>>(&mut self, path: P) {
        let local_repository_root = infer_working_directory(path);

        if self.contains(&local_repository_root) {
            panic!()
        }

        let repository = discover_git_repository(&local_repository_root);
        let package = create_package(local_repository_root, repository);

        self.packages.push(package);
    }

    /// Adds a [Dependency] to the package located at given [Path]
    ///
    /// If not manifest file named 'dependencies.json' is found one is created.
    ///
    /// # Examples
    /// ```
    /// use std::env::temp_dir;
    /// use url::Url;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut registry = initialize_registry();
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    ///
    /// assert!(path.is_dir());
    /// assert!(registry.is_empty());
    ///
    /// registry.add(&path);
    ///
    /// assert!(!registry.is_empty());
    ///
    /// registry.add_dependency(&path, url);
    /// ```
    ///
    /// # Panics
    /// Panics when given [Path] does not point to a valid package
    /// ```rust, should_panic
    /// use std::env::temp_dir;
    /// use url::Url;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let path = temp_dir();
    /// let registry = initialize_registry();
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.add_dependency(path, url);
    /// ```
    pub fn add_dependency<P: AsRef<Path>>(&self, local_location: P, url: Url) {
        if let Some(package) = self.find_by_local_location(&local_location) {
            package.add_dependency(url);
            return;
        }
        panic!("No package found @ {}", local_location.as_ref().display());
    }

    /// Returns a list of all [Dependency] items required by the package at given [Path]
    /// Returns an empty list if no manifest file called 'dependencies.json' was found.
    ///
    /// # Panics
    /// Panics when given [Path] does not point to a valid package
    /// ```rust, should_panic
    /// use std::env::temp_dir;
    /// use url::Url;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let path = temp_dir();
    /// let registry = initialize_registry();
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.get_dependencies(path);
    /// ```
    pub fn get_dependencies<P: AsRef<Path>>(&self, local_location: P) -> Vec<Dependency> {
        if let Some(package) = self.find_by_local_location(&local_location) {
            return package.get_dependencies()
        }
        panic!("No package found @ {}", local_location.as_ref().display());
    }

    /// Removes a [Dependency] from the package at given [Path]
    ///
    /// # Panics
    /// Panics when given [Path] does not point to a valid package
    /// ```rust, should_panic
    /// use std::env::temp_dir;
    /// use url::Url;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let path = temp_dir();
    /// let registry = initialize_registry();
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.remove_dependency(path, url);
    /// ```
    pub fn remove_dependency<P: AsRef<Path>>(&self, local_location: P, value: Url) {
        if let Some(package) = self.find_by_local_location(&local_location) {
            package.remove_dependency(value);
            return;
        }
        panic!("No package found @ {}", local_location.as_ref().display());
    }

    /// Searches the given [Path] for a package and removes it from the [Registry]
    /// Criteria for being detected as a package are:
    /// - Being in a git repository
    ///
    /// # Examples
    /// ```
    /// use std::env::temp_dir;
    /// use std::fs::create_dir_all;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    ///
    /// create_dir_all(&path);
    ///
    /// assert!(path.is_dir());
    ///
    /// registry.add(&path);
    ///
    /// assert!(registry.contains(&path));
    ///
    /// registry.remove(&path);
    ///
    /// assert!(registry.is_empty());
    /// ```
    ///
    /// If the registry does not contain the package found it does nothing.
    /// ```
    /// use std::env::temp_dir;
    /// use knapsac_lib::initialize_registry;
    ///
    /// let mut registry = initialize_registry();
    /// let mut path = temp_dir();
    /// path.push("mock_packages_known");
    ///
    /// assert!(path.is_dir());
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
    /// use knapsac_lib::initialize_registry;
    ///
    /// let path = temp_dir();
    /// let mut registry = initialize_registry();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.remove(&path);
    /// ```
    pub fn remove<P: AsRef<Path>>(&mut self, local_location: P) {
        let local_repository_root = infer_working_directory(local_location);
        self.packages
            .retain(|p| p.local_location != local_repository_root);
    }
}
