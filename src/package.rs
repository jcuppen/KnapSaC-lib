use std::fmt::{Display, Formatter};
use crate::manifest::Manifest;
use crate::dependency::Dependency;
use crate::module::Module;
use crate::utils::{discover_git_repository, infer_working_directory};

use std::fs::create_dir;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use nanoid::nanoid;
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
/// A [`Package`] represents an package managed by KnapSaC
pub struct Package {
    pub(crate) local_location: PathBuf,
    pub(crate) remote_location: Option<Url>,
}

impl Package {
    /// Creates a new [`Package`] based on the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - A [`Path`] or reference to one that points to somewhere inside the [`Package`]s git repository.
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    ///
    /// let path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&path);
    /// let package = Package::create(&path);
    /// ```
    ///
    /// # Panics
    /// Panics when the given [`Path`] does not point to a valid [`Repository`]
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    ///
    /// let path = env::temp_dir().join("not_a_repository");
    /// assert!(Repository::discover(&path).is_err());
    /// let package = Package::create(&path);
    /// ```
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let local_repository_root = infer_working_directory(path);
        let repository = discover_git_repository(&local_repository_root);
        let remotes = repository.remotes().unwrap();

        let package = match remotes.is_empty() {
            true => {
                Package {
                    local_location: local_repository_root,
                    remote_location: None,
                }
            },
            false => {
                let remote_name_str = remotes.get(0).unwrap();
                let remote = repository.find_remote(remote_name_str).unwrap();

                Package {
                    local_location: local_repository_root,
                    remote_location: Url::parse(remote.url().unwrap()).ok(),
                }
            },
        };

        Manifest::initialize().save(package.manifest_location());

        package
    }

    /// Downloads a [`Package`] located at given [`Url`] to given [`Path`]
    ///
    /// # Arguments
    /// * `url` - An [`Url`] pointing to the remote location of a git repository
    /// * `path` - An [`Path`] or reference to one that points to where packages need to be downloaded to
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// let path = env::temp_dir();
    /// # assert!(url.is_ok());
    /// # assert!(path.exists());
    /// let package = Package::download(url.unwrap(), path);
    /// ```
    ///
    /// # Panics
    /// Panics when no directory exists at given [`Path`]
    /// ```rust, should_panic
    /// # use std::env;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// let path = env::temp_dir().join("invalid_dir");
    /// # assert!(url.is_ok());
    /// assert!(!path.exists());
    /// let package = Package::download(url.unwrap(), &path);
    /// ```
    /// Panics when given [`Path`] points to a file
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let path = env::temp_dir().join("invalid.txt");
    /// # fs::write(&path, "hello");
    /// assert!(path.is_file());
    /// let package = Package::download(url, path);
    /// ```
    pub fn download<P: AsRef<Path>>(url: Url, path: P) -> Self {
        if !path.as_ref().is_dir() {
            panic!("No directory found @ {}", path.as_ref().display());
        }
        let mut repository_path = path.as_ref().to_path_buf();
        repository_path.push(nanoid!());
        create_dir(&repository_path).unwrap();
        if Repository::clone(url.as_str(), &repository_path).is_err() {
            panic!(
                "Failed to download package from `{}` to `{}`",
                url,
                path.as_ref().display()
            )
        }
        Package::create(repository_path)
    }

    fn load_manifest(&self) -> Manifest {
        Manifest::load(self.manifest_location())
    }

    fn manifest_location(&self) -> PathBuf {
        let mut path: PathBuf = self.local_location.clone();
        path.push("manifest");
        path.set_extension("json");
        path
    }

    /// Strips the [`Package`]'s `local_location` from the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - [`Path`] that needs to be stripped
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use std::path::{Path, PathBuf};
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    ///
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// assert!(package_path.exists());
    /// let package = Package::create(&package_path);
    /// assert_eq!(package.strip_prefix(package_path.join("example.txt")), PathBuf::from("example.txt"))
    /// ```
    ///
    /// # Panics
    /// Panics when the given [`Path`] is not a prefix of the [`Package`]'s `local_location`
    /// ```rust, should_panic
    /// # use std::env;
    /// # use git2::Repository;
    /// # use knapsac_lib::package::Package;
    ///
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// assert!(package_path.exists());
    /// let other_path = env::temp_dir().join("mock_package_registered").join("example.txt");
    /// Package::create(&package_path).strip_prefix(&other_path);
    /// ```
    pub fn strip_prefix<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref().strip_prefix(self.local_location.clone()).unwrap().to_path_buf()
    }

    /// Adds a [`Dependency`] to a [`Package`]
    ///
    /// # Arguments
    /// * `dependency` - A [`Dependency`] that needs to be added
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    /// # use knapsac_lib::dependency::Dependency;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// let path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&path);
    /// # assert!(url.is_ok());
    /// assert!(path.exists());
    /// let package = Package::create(path);
    /// let dependency = Dependency::create(url.unwrap());
    /// package.add_dependency(dependency.clone());
    /// assert!(package.has_dependency(&dependency));
    /// ```
    pub fn add_dependency(&self, dependency: Dependency) {
        let mut manifest = self.load_manifest();
        manifest.add_dependency(dependency);
        manifest.save(self.manifest_location());
    }

    /// Checks the [`Package`] if it depends on the given [`Dependency`]
    ///
    /// # Arguments
    /// * `dependency` - A reference to a [`Dependency`] that needs to be checked
    pub fn has_dependency(&self, dependency: &Dependency) -> bool {
        let manifest = self.load_manifest();
        manifest.has_dependency(dependency)
    }

    /// Removes a [`Dependency`] from a [`Package`]
    ///
    /// # Arguments
    /// * `dependency` - A reference to a [`Dependency`] that needs to be removed
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::dependency::Dependency;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// let path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&path);
    /// # assert!(url.is_ok());
    /// assert!(path.exists());
    /// let package = Package::create(path);
    /// let dependency = Dependency::create(url.unwrap());
    /// package.add_dependency(dependency.clone());
    /// assert!(package.has_dependency(&dependency));
    /// package.remove_dependency(&dependency);
    /// assert!(!package.has_dependency(&dependency))
    /// ```
    pub fn remove_dependency(&self, dependency: &Dependency) {
        let mut manifest = self.load_manifest();

        manifest.remove_dependency(dependency);
        manifest.save(self.manifest_location());
    }

    /// Adds a [`Module`] to a [`Package`]
    ///
    /// # Arguments
    /// * `module` - A [`Module`] that needs to be added
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// # assert!(url.is_ok());
    /// let package = Package::download(url.unwrap(), env::temp_dir());
    /// let module_path: PathBuf = ["src","JSON.sac"].iter().collect();
    /// let module = Module::create(module_path, None);
    /// package.add_module(module.clone());
    /// assert!(package.has_module(&module));
    /// ```
    ///
    /// # Panics
    /// Panics when module_path does not point to an existing file
    /// ```rust, should_panic
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use git2::Repository;
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::package::Package;
    ///
    /// let package_path = env::temp_dir().join("mock_package_known");
    /// Repository::init(&package_path);
    /// assert!(package_path.exists());
    /// let package = Package::create(&package_path);
    /// let module_path: PathBuf = ["nonexistent.c"].iter().collect();
    /// assert!(!package_path.join(&module_path).exists());
    /// let module = Module::create(&module_path, None);
    /// package.add_module(module);
    /// ```
    pub fn add_module(&self, module: Module) {
        let mut manifest = self.load_manifest();

        let full_module_path = self.local_location.join(&module.location);
        if full_module_path.exists() && full_module_path.is_file() {
            manifest.add_module(module);
            manifest.save(self.manifest_location());
        } else {
            panic!("Module does not point to existing file");
        }
    }

    /// Searches the [`Package`] for a [`Module`] that is located at the given [`Path`]
    ///
    /// # Arguments
    /// * `location` - [`Path`] pointing to a location where a [`Module`] should be located.
    ///
    /// # Examples
    /// //TODO
    pub fn get_module_by_location<P: AsRef<Path>>(&self, location: P) -> Option<Module> {
        self.load_manifest().get_module_by_location(location).cloned()
    }

    /// Checks the [`Package`] if it provides a given [`Module`]
    ///
    /// # Arguments
    /// * `module` - A reference to a [`Module`] that needs to be checked
    ///
    /// # Examples
    /// //TODO
    pub fn has_module(&self, module: &Module) -> bool {
        self.load_manifest().modules.contains(module)
    }

    /// Checks the [`Package`] if it has any [`Module`] with a given `identifier`
    ///
    /// # Arguments
    /// * `identifier` - The identifier to check for
    ///
    pub(crate) fn has_modules_with_identifiers(&self, identifiers: &[String]) -> bool {
        self.load_manifest().modules.iter().any(|m|identifiers.contains(&m.identifier))
    }

    /// Removes a [`Module`] from a [`Package`]
    ///
    /// # Arguments
    /// * `module` - A reference to a [`Module`] that needs to be removed
    ///
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON");
    /// # assert!(url.is_ok());
    /// let package = Package::download(url.unwrap(), env::temp_dir());
    /// let module_path: PathBuf = ["src", "JSON.sac"].iter().collect();
    /// let module = Module::create(&module_path, None);
    /// package.add_module(module.clone());
    /// assert!(package.has_module(&module));
    /// package.remove_module(&module);
    /// assert!(!package.has_module(&module))
    /// ```
    pub fn remove_module(&self, module: &Module) {
        let mut manifest = self.load_manifest();
        manifest.remove_module(module);
        manifest.save(self.manifest_location());
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "local location : {}\nremote location : {}",
            self.local_location.display(),
            self.local_location.display()
        )
    }
}
