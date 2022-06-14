use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Hash, Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub enum Dependency {
    Stray(String, PathBuf),
    Standalone(PathBuf),
    Package,
}
