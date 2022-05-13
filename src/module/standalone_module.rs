use crate::error::ModuleError;
use crate::error::ModuleError::DoesNotExist;
use crate::error::ModuleError::LocationNotAbsolute;
use crate::module::Module;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Hash, Eq, PartialEq, Clone, Debug)]
pub struct StandaloneModule {
    // pub identifier: String,
    pub(crate) location: PathBuf,
}

impl StandaloneModule {
    /// Creates a new [`StandaloneModule`] based on the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - An absolute [`Path`] that points to a module file
    /// //* `identifier` - the identifier of the [`StandaloneModule`], defaults to the file stem
    ///
    /// # Examples
    /// ```
    /// # use std::{env, fs};
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    /// # use knapsac_lib::module::Module;
    ///
    /// let module_path = env::temp_dir().join("a.sac");
    /// # fs::write(&module_path, "hello");
    /// # assert!(module_path.exists());
    /// let module = StandaloneModule::create(module_path).unwrap();
    /// // assert_eq!(module.identifier, "a");
    /// ```
    ///
    /// # Errors
    /// Returns a [`LocationDoesNotExist`] error when [`Path`] does not point to and existing file
    /// ```
    /// # use std::env;
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::error::ModuleError::{DoesNotExist, LocationNotRelative};
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    ///
    /// let module_path = env::temp_dir().join("nonexistent.sac");
    /// let err = StandaloneModule::create(module_path).unwrap_err();
    /// assert_eq!(err, DoesNotExist);
    /// ```
    /// Returns a [`LocationNotAbsolute`] error when [`Path`] is not absolute
    /// ```
    /// # use std::{env, fs};
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::error::ModuleError::{LocationNotAbsolute, LocationNotRelative};
    /// # use knapsac_lib::module::standalone_module::StandaloneModule;
    ///
    /// let module_path: PathBuf = ["src", "a.sac"].iter().collect();
    /// let err = StandaloneModule::create(module_path).unwrap_err();
    /// assert_eq!(err, LocationNotAbsolute);
    /// ```
    pub fn create<P: AsRef<Path>>(
        path: P,
        // identifier: Option<String>,
    ) -> Result<Self, ModuleError> {
        if path.as_ref().is_relative() {
            return Err(LocationNotAbsolute);
        }
        if !path.as_ref().exists() {
            return Err(DoesNotExist);
        }

        let (_identifier, path) = StandaloneModule::prepare(path, None);

        Ok(StandaloneModule {
            // identifier,
            location: path,
        })
    }
}

impl Module for StandaloneModule {}
