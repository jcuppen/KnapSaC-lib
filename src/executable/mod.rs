mod get;
mod add;
mod remove;

use std::collections::HashSet;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Executable {
    dependencies: HashSet<String>
}

impl Executable {
    pub(crate) fn create() -> Self {
        Executable {
            dependencies: HashSet::new(),
        }
    }
}