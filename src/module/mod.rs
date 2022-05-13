use std::path::{Path, PathBuf};

pub mod package_module;
pub mod standalone_module;

pub trait Module {
    fn prepare<P: AsRef<Path>>(path: P, identifier: Option<String>) -> (String, PathBuf) {
        let default = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let identifier = identifier.unwrap_or(default);
        (identifier, path.as_ref().to_path_buf())
    }
}
