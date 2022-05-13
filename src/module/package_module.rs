use crate::error::ModuleError;
use crate::error::ModuleError::{DoesNotExist, LocationNotRelative};
use crate::module::Module;
use crate::package::Package;
use serde::Deserialize;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Clone, Eq, Hash, PartialEq, Deserialize, Serialize, Debug)]
pub struct PackageModule {
    pub(crate) identifier: String,
    pub(crate) location: PathBuf,
}

impl Module for PackageModule {}

impl PackageModule {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    /// Creates a new [`PackageModule`] based on the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - A relative [`Path`] that points to a file within a [`Package`]
    /// * `identifier` - the identifier (name) of the [`PackageModule`], defaults to the file stem
    /// * `package` - the reference to the package the module is meant for
    /// # Examples
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use url::Url;
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::module::package_module::PackageModule;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let module_path: PathBuf = ["src", "JSON.sac"].iter().collect();
    /// let module = PackageModule::create(module_path, Some("a".to_string()), &package).unwrap();
    /// assert_eq!(module.get_identifier(), "a");
    /// ```
    /// if no identifier is provided it will default to the file stem
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use url::Url;
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::module::package_module::PackageModule;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let module_path: PathBuf = ["src", "JSON.sac"].iter().collect();
    /// let module = PackageModule::create(&module_path, None, &package).unwrap();
    /// assert_eq!(module.get_identifier(), "JSON");
    /// ```
    ///
    /// # Errors
    /// Returns a [`LocationNotRelative`] error when [`Path`] is not relative
    /// ```
    /// # use std::env;
    /// # use url::Url;
    /// # use knapsac_lib::error::ModuleError::LocationNotRelative;
    /// # use knapsac_lib::module::package_module::PackageModule;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let err = PackageModule::create(env::temp_dir(), None, &package).unwrap_err();
    /// assert_eq!(err, LocationNotRelative);
    /// ```
    /// Returns a [`DoesNotExist`] error when [`Package`] does not contain module at [`Path`]
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use url::Url;
    /// # use knapsac_lib::error::ModuleError::{DoesNotExist};
    /// # use knapsac_lib::module::Module;
    /// # use knapsac_lib::module::package_module::PackageModule;
    /// # use knapsac_lib::package::Package;
    ///
    /// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
    /// let package = Package::download(url, env::temp_dir()).unwrap();
    /// let module_path: PathBuf = ["src", "nonexistent.sac"].iter().collect();
    /// let err = PackageModule::create(&module_path, None, &package).unwrap_err();
    /// assert_eq!(err, DoesNotExist);
    /// ```
    pub fn create<P: AsRef<Path>>(
        path: P,
        id: Option<String>,
        package: &Package,
    ) -> Result<Self, ModuleError> {
        let (identifier, path) = PackageModule::prepare(path, id);
        if path.is_absolute() {
            return Err(LocationNotRelative);
        }
        if !package.has_file(&path) {
            return Err(DoesNotExist);
        }
        Ok(PackageModule {
            identifier,
            location: path,
        })
    }
}
