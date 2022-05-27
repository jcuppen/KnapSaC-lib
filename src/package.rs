// use crate::dependency::PackageDependency;
use crate::error::PackageError::{
    DownloadFailed, NoRemoteLocation, NotARepository, PackageRootNotADirectory,
};
use crate::error::{PackageError, RepositoryError};

use crate::utils::infer_working_directory;
use git2::Repository;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
/// A [`Package`] represents an package managed by KnapSaC
pub struct Package {
    pub(crate) local_location: PathBuf,
    pub(crate) remote_location: Url,
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
    /// # Errors
    /// Returns an [`NotARepository`] error when the given [`Path`] does not point to a valid [`Repository`]
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use knapsac_lib::error::PackageError::NotARepository;
    /// # use knapsac_lib::package::Package;
    ///
    /// let path = env::temp_dir().join("not_a_repository");
    /// # assert!(Repository::discover(&path).is_err());
    /// let err = Package::create(&path).unwrap_err();
    /// assert_eq!(err, NotARepository);
    /// ```
    /// Returns an [`NoRemoteLocation`] error when the [`Repository`] pointed to does not have any remotes
    /// ```
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use knapsac_lib::error::PackageError::{NoRemoteLocation, NotARepository};
    /// # use knapsac_lib::package::Package;
    ///
    /// let path = env::temp_dir().join("mock_package");
    /// Repository::init(&path);
    /// let err = Package::create(&path).unwrap_err();
    /// assert_eq!(err, NoRemoteLocation);
    /// ```
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, PackageError> {
        let local_repository_root = match infer_working_directory(&path) {
            Ok(p) => p,
            Err(err_type) => match err_type {
                RepositoryError::BareRepository => panic!("This should never happen"),
                RepositoryError::RepositoryDiscoveryFailed => return Err(NotARepository),
            },
        };
        let repository = Repository::init(&local_repository_root).unwrap();
        let remotes = repository.remotes().unwrap();

        if remotes.is_empty() {
            return Err(NoRemoteLocation);
        }

        let remote_name_str = remotes.get(0).unwrap();
        let remote = repository.find_remote(remote_name_str).unwrap();

        let package = Package {
            local_location: local_repository_root,
            remote_location: Url::parse(remote.url().unwrap()).unwrap(),
        };

        Ok(package)
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
    /// # Errors
    /// Returns an [`PackageRootNotADirectory`] when no directory exists at given [`Path`]
    /// ```
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
    /// Returns a [`PackageRootNotADirectory`] error when given [`Path`] does not point to a directory
    /// ```
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
    ///
    /// TODO! download fails error
    pub fn download<P: AsRef<Path>>(url: Url, path: P) -> Result<Self, PackageError> {
        if !path.as_ref().is_dir() {
            return Err(PackageRootNotADirectory);
        }
        let mut repository_path = path.as_ref().to_path_buf();
        repository_path.push(nanoid!());
        create_dir(&repository_path).unwrap();
        if Repository::clone(url.as_str(), &repository_path).is_err() {
            return Err(DownloadFailed);
        }
        Package::create(repository_path)
    }
/*
    pub fn get_local_location(&self) -> PathBuf {
        self.local_location.clone()
    }

    fn load_manifest(&self) -> Result<Manifest, PackageError> {
        match Manifest::load(&self.manifest_location()) {
            Ok(m) => Ok(m),
            Err(e) => match e {
                ManifestError::InvalidManifest => Err(InvalidManifest),
            },
        }
    }

    fn manifest_location(&self) -> PathBuf {
        let mut path = self.get_local_location();
        path.push("manifest");
        path.set_extension("json");
        path
    }
 */
/*
    /// Strips the [`Package`]'s `local_location` from the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - [`Path`] that needs to be stripped
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use std::path::{Path, PathBuf};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let other_path = package.get_local_location().join("example.txt");
    /// fs::write(&other_path, "hello");
    /// assert!(&other_path.exists());
    /// assert_eq!(package.strip_prefix(&other_path), PathBuf::from("example.txt"))
    /// ```
    ///
    /// # Panics
    /// Panics when the given [`Path`] is not a prefix of the [`Package`]'s `local_location`
    /// ```rust, should_panic
    /// # use std::{env, fs};
    /// # use git2::Repository;
    /// # use url::Url;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let other_path = env::temp_dir().join("other").join("example.txt");
    /// fs::write(&other_path, "hello");
    /// assert!(other_path.exists());
    /// package.strip_prefix(&other_path);
    /// ```
    pub fn strip_prefix<P: AsRef<Path>>(&self, path: &Path) -> PathBuf {
        path.canonicalize()
            .unwrap()
            .strip_prefix(self.get_local_location())
            .unwrap()
            .to_path_buf()
    }
    */
    /*
        /// TODO write docs
        /// Adds a [`PackageDependency`] to a [`Package`]
        ///
        /// # Arguments
        /// * `dependency` - A [`PackageDependency`] that needs to be added
        pub fn add_package_dependency(
            &self,
            dependency: PackageDependency,
        ) -> Result<(), PackageError> {
            let mut manifest = self.load_manifest()?;
            manifest.add_package_dependency(dependency);
            manifest.save(self.manifest_location());
            Ok(())
        }
    */

    /*
        /// Checks the [`Package`] if it depends on the given [`PackageDependency`]
        ///
        /// TODO write docs
        ///
        /// # Arguments
        /// * `dependency` - A reference to a [`PackageDependency`] that needs to be checked
        pub fn has_package_dependency(
            &self,
            dependency: &PackageDependency,
        ) -> Result<bool, PackageError> {
            Ok(self.load_manifest()?.has_package_dependency(dependency))
        }
    */

    /*
        /// Removes a [`PackageDependency`] from a [`Package`]
        ///
        /// # Arguments
        /// * `dependency` - A reference to a [`PackageDependency`] that needs to be removed
        ///
        /// # Examples
        /// ```
        /// # use std::env;
        /// # use git2::Repository;
        /// # use url::Url;
        /// # use knapsac_lib::dependency::PackageDependency;
        /// # use knapsac_lib::package::Package;
        ///
        /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
        /// let package = Package::download(url.clone(), env::temp_dir()).unwrap();
        /// let dependency = PackageDependency::create(url.clone());
        ///
        /// package.add_package_dependency(dependency.clone());
        /// assert!(package.has_package_dependency(&dependency).unwrap());
        /// package.remove_package_dependency(&dependency);
        /// assert!(!package.has_package_dependency(&dependency).unwrap())
        /// ```
        ///
        /// TODO: disallow cyclic dependencies
        pub fn remove_package_dependency(
            &self,
            dependency: &PackageDependency,
        ) -> Result<(), PackageError> {
            let mut manifest = self.load_manifest()?;
            manifest.remove_package_dependency(dependency);
            manifest.save(self.manifest_location());
            Ok(())
        }
    */
    /*
       /// Adds a [`ModuleDependency`] to a [`Package`]
       ///
       /// # Arguments
       /// * `dependency` - A [`ModuleDependency`] that needs to be added
       ///
       /// # Examples
       /// ```
       /// # use std::{env, fs};
       /// # use git2::Repository;
       /// # use url::Url;
       /// # use knapsac_lib::module::standalone_module::StandaloneModule;
       /// # use knapsac_lib::package::Package;
       ///
       /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
       /// let package = Package::download(url, env::temp_dir()).unwrap();
       ///
       /// let module_path = env::temp_dir().join("a.sac");
       /// # fs::write(&module_path, "hello");
       /// let module = StandaloneModule::create(&module_path).unwrap();
       /// package.add_module_dependency(module.clone());
       /// println!("{:?}", module);
       /// assert!(package.has_module_dependency(&module).unwrap());
       /// ```
       pub fn add_module_dependency(&self, dependency: StandaloneModule) -> Result<(), PackageError> {
           let mut manifest = self.load_manifest()?;
           manifest.add_module_dependency(dependency);
           Ok(())
       }

       /// Checks the [`Package`] if it depends on the given [`ModuleDependency`]
       ///
       /// # Arguments
       /// * `dependency` - A reference to a [`ModuleDependency`] that needs to be checked
       pub fn has_module_dependency(
           &self,
           dependency: &StandaloneModule,
       ) -> Result<bool, PackageError> {
           Ok(self.load_manifest()?.has_module_dependency(dependency))
       }

       // TODO write docs
       pub fn remove_module_dependency(
           &self,
           dependency: &StandaloneModule,
       ) -> Result<(), PackageError> {
           self.load_manifest()?.remove_module_dependency(dependency);
           Ok(())
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
       /// # use knapsac_lib::module::package_module::PackageModule;
       /// # use knapsac_lib::package::Package;
       ///
       /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
       /// let package = Package::download(url, env::temp_dir()).unwrap();
       /// let module_path: PathBuf = ["src","JSON.sac"].iter().collect();
       /// let module = PackageModule::create(module_path, None, &package).unwrap();
       /// package.add_module(module.clone());
       /// assert!(package.has_module(&module).unwrap());
       /// ```
       pub fn add_module(&self, module: PackageModule) -> Result<(), PackageError> {
           let mut manifest = self.load_manifest()?;
           manifest.add_module(module);
           manifest.save(self.manifest_location());
           Ok(())
       }

       pub(crate) fn has_file<P: AsRef<Path>>(&self, path: P) -> bool {
           self.local_location.join(path).exists()
       }

       /// Searches the [`Package`] for a [`Module`] that is located at the given [`Path`]
       ///
       /// # Arguments
       /// * `location` - [`Path`] pointing to a location where a [`Module`] should be located.
       ///
       /// # Examples
       /// TODO write docs
       pub fn get_module_by_location(
           &self,
           location: &Path,
       ) -> Result<Option<PackageModule>, PackageError> {
           Ok(self
               .load_manifest()?
               .get_module_by_location(location)
               .cloned())
       }

       /// Checks the [`Package`] if it provides a given [`Module`]
       ///
       /// # Arguments
       /// * `module` - A reference to a [`Module`] that needs to be checked
       ///
       /// # Examples
       /// TODO write docs
       pub fn has_module(&self, module: &PackageModule) -> Result<bool, PackageError> {
           Ok(self.load_manifest()?.modules.contains(module))
       }

       /// Checks the [`Package`] if it has any [`Module`] with a given `identifier`
       ///
       /// # Arguments
       /// * `identifier` - The identifier to check for
       ///
       pub(crate) fn has_modules_with_identifiers(
           &self,
           identifiers: &[String],
       ) -> Result<bool, PackageError> {
           Ok(self
               .load_manifest()?
               .modules
               .iter()
               .any(|m| identifiers.contains(&m.identifier)))
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
       /// # use knapsac_lib::module::package_module::PackageModule;
       /// # use knapsac_lib::package::Package;
       ///
       /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
       /// let package = Package::download(url, env::temp_dir()).unwrap();
       /// let module_path: PathBuf = ["src", "JSON.sac"].iter().collect();
       /// let module = PackageModule::create(&module_path, None, &package).unwrap();
       /// package.add_module(module.clone());
       /// assert!(package.has_module(&module).unwrap());
       /// package.remove_module(&module);
       /// assert!(!package.has_module(&module).unwrap());
       /// ```
       pub fn remove_module(&self, module: &PackageModule) -> Result<(), PackageError> {
           let mut manifest = self.load_manifest()?;
           manifest.remove_module(module);
           manifest.save(self.manifest_location());
           Ok(())
       }

    */
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
