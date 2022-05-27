use std::path::{Path, PathBuf};

mod manifest;
pub mod package_module;
pub mod standalone_module;

pub(crate) trait Module {
    fn prepare(
        identifier: &str,
        output_location: &Path,
    ) -> (String, PathBuf) {
        (
            identifier.to_string(),
            output_location.to_path_buf(),
        )
    }
}
