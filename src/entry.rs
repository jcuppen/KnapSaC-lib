use std::path::PathBuf;

pub enum Entry {
    Executable(PathBuf),
    StandaloneModule(String),
    PackageModule(String, String),
}
