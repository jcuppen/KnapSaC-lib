use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Clone)]
pub struct Module {
    pub identifier: String,
    pub(crate) location: PathBuf,
}

impl Module {
    /// Creates a new [`Module`] based on the given [`Path`]
    ///
    /// # Arguments
    /// * `path` - A relative [`Path`] that points to a file within a [`Package`]
    /// * `identifier` - the identifier (name) of the [`Module`], defaults to the file stem
    ///
    /// # Examples
    /// ```
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::module::Module;
    ///
    /// let module_path: PathBuf = ["src", "a.sac"].iter().collect();
    /// let module = Module::create(module_path, Some("a".to_string()));
    /// assert_eq!(module.identifier, "a");
    /// ```
    /// if no identifier is provides it will default to the file stem
    /// ```
    /// # use std::path::PathBuf;
    /// # use knapsac_lib::module::Module;
    ///
    /// let module_path: PathBuf = ["src", "a.sac"].iter().collect();
    /// let module = Module::create(module_path, None);
    /// assert_eq!(module.identifier, "a");
    /// ```
    ///
    /// # Panics
    /// Panics when received path is absolute
    /// ```rust, should_panic
    /// # use std::env;
    /// # use knapsac_lib::module::Module;
    ///
    /// let module_path = env::temp_dir();
    /// Module::create(module_path, Some("a".to_string()));
    /// ```
    pub fn create<P: AsRef<Path>>(path: P, id: Option<String>) -> Self {
        if path.as_ref().is_absolute() {
            panic!("Path is absolute")
        }
        let identifier: String = match id {
            None => {
                String::from(path.as_ref().file_stem().unwrap().to_str().unwrap())
            }
            Some(id) => id,
        };
        Module {
            identifier,
            location: path.as_ref().to_path_buf(),
        }
    }
}
