use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub(crate) struct Module {
    pub(crate) local_location: PathBuf,
}
