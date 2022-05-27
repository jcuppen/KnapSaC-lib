mod remove;
mod add;
mod get;

use std::collections::HashSet;
use std::path::PathBuf;
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct Module {
    pub output_directory: PathBuf,
    dependencies: HashSet<String>
}

impl Module {
    pub(crate) fn create(output_directory: PathBuf) -> Self {
        Module {
            output_directory,
            dependencies: HashSet::new(),
        }
    }
}
