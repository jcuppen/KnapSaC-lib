use git2::Repository;
use std::path::{Path, PathBuf};

pub(crate) fn discover_git_repository<P: AsRef<Path>>(path: P) -> Repository {
    Repository::discover(&path).expect(&*format!(
        "Failed to discover repository @ {}",
        path.as_ref().display()
    ))
}

pub(crate) fn infer_working_directory<P: AsRef<Path>>(path: P) -> PathBuf {
    discover_git_repository(&path)
        .workdir()
        .expect(&*format!(
            "Failed to find root of local repository for path '{}'",
            path.as_ref().display(),
        ))
        .to_path_buf()
}
