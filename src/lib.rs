extern crate core;

use std::fs;
use crate::registry::Registry;
use git2::Repository;
use std::fs::read_to_string;
use std::path::{Path};
use nanoid::nanoid;
use url::Url;

mod dependency;
mod package;
pub mod registry;
mod utils;

/// Loads a [Registry] based on the given [Path]
///
/// # Examples
/// ```
/// use std::env::temp_dir;
/// use std::fs::write;
/// use std::path::Path;
/// use knapsac_lib::load_registry;
///
/// let mut path = temp_dir();
/// path.push("registry.json");
///
/// write(&path, "{\"packages\": []}").unwrap();
///
/// assert!(path.exists());
/// assert!(path.is_file());
///
/// let registry = load_registry(&path);
/// ```
///
/// # Panics
///
/// Panics when there is no file at given [Path]
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use knapsac_lib::load_registry;
/// let mut path = temp_dir();
///
/// path.push("nonexistent.json");
/// assert!(!path.exists());
///
/// let registry = load_registry(&path);
/// ```
///
/// Panics when there is no JSON file at given [Path]
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use knapsac_lib::load_registry;
/// let mut path = temp_dir();
/// path.push("registry.txt");
/// assert!(path.exists());
/// assert!(path.is_file());
///
/// let registry = load_registry(&path);
/// ```
///
/// Panics when the given JSON file is not valid JSON
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use std::fs::{read_to_string, write};
/// use knapsac_lib::load_registry;
///
/// let mut path = temp_dir();
/// path.push("invalid.json");
///
/// write(&path, "{").unwrap();
///
/// assert!(path.exists());
/// assert!(path.is_file());
///
/// let contents = read_to_string(&path);
///
/// assert!(contents.is_ok());
/// assert_eq!(contents.unwrap(), String::from("{"));
///
/// let registry = load_registry(&path);
/// ```
///
/// Panics when JSON cannot be parsed to a valid [Registry]
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use std::fs::{read_to_string, write};
/// use knapsac_lib::load_registry;
///
/// let mut path = temp_dir();
/// path.push("invalid.json");
///
/// write(&path, "{ \"packages\": 12 }").unwrap();
///
/// assert!(path.exists());
/// assert!(path.is_file());
///
/// let contents = read_to_string(&path);
///
/// assert!(contents.is_ok());
/// assert_eq!(contents.unwrap(), String::from("{ \"packages\": 12 }"));
///
/// let registry = load_registry(&path);
/// ```
pub fn load_registry<P: AsRef<Path>>(path: P) -> Registry {
    if let Ok(data) = read_to_string(&path) {
        return serde_json::from_str(data.as_str()).unwrap();
    }
    panic!("No registry found @ {}", path.as_ref().display())
}

/// Creates a new empty [Registry].
///
/// # Examples
/// ```
/// use knapsac_lib::initialize_registry;
/// use knapsac_lib::registry::Registry;
///
/// let registry = initialize_registry();
/// assert!(registry.is_empty())
/// ```
pub fn initialize_registry() -> Registry {
    Registry { packages: vec![] }
}

/// Downloads the package located at given [Url] to given [Path]
/// and add the new package to the [Registry]
///
/// # Examples
/// ```
/// use std::env::temp_dir;
/// use std::fs;
/// use std::fs::{read_to_string, write};
/// use url::Url;
/// use knapsac_lib::{download, initialize_registry, load_registry};
///
/// let mut registry = initialize_registry();
/// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
/// let mut path = temp_dir();
///
/// assert!(path.exists());
/// assert!(registry.is_empty());
///
/// download(&mut registry, url, &path);
///
/// assert!(!registry.is_empty());
/// ```
///
/// # Panics
/// Panics when no directory exists at given [Path]
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use std::fs;
/// use std::fs::{read_to_string, write};
/// use url::Url;
/// use knapsac_lib::{download, initialize_registry, load_registry};
///
/// let mut registry = initialize_registry();
/// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
/// let mut path = temp_dir();
/// path.push("invalid_dir");
///
/// assert!(!path.exists());
///
/// download(&mut registry, url, &path);
/// ```
/// Panics when given [Path] points to a file
/// ```rust,should_panic
/// use std::env::temp_dir;
/// use std::fs;
/// use std::fs::{read_to_string, write};
/// use url::Url;
/// use knapsac_lib::{download, initialize_registry, load_registry};
///
/// let mut registry = initialize_registry();
/// let url = Url::parse("https://github.com/jcuppen/JSON").unwrap();
/// let mut path = temp_dir();
/// path.push("invalid.txt");
///
/// write(&path, "hello");
///
/// assert!(path.is_file());
///
/// download(&mut registry, url, path);
/// ```
pub fn download<P: AsRef<Path>>(registry: &mut Registry, url: Url, path: P) {
    if !path.as_ref().is_dir() {
        panic!("No directory found @ {}", path.as_ref().display());
    }
    let mut repository_path = path.as_ref().to_path_buf();
    repository_path.push(nanoid!());
    fs::create_dir(&repository_path).unwrap();
    if Repository::clone(url.as_str(), &repository_path).is_err() {
        panic!("Failed to download package from `{}` to `{}`", url, path.as_ref().display())
    };
    registry.add(&repository_path);
}
