use crate::error::RepositoryError;
use crate::error::RepositoryError::{BareRepository, RepositoryDiscoveryFailed};
use git2::Repository;
use std::path::{Path, PathBuf};

pub(crate) fn infer_working_directory<P: AsRef<Path>>(path: P) -> Result<PathBuf, RepositoryError> {
    if let Ok(repository) = Repository::discover(&path) {
        return match repository.workdir() {
            None => Err(BareRepository),
            Some(p) => Ok(p.to_path_buf().canonicalize().unwrap()),
        };
    }
    Err(RepositoryDiscoveryFailed)
}
