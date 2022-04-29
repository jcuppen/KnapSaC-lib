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

// ```
// use knapsac_lib;
// use std::path::Path;
// use knapsac_lib::utils;
// let path = Path::new(format!("{}/test_data/mock_package_a", env!("CARGO_MANIFEST_DIR")).as_str());
// let repository = utils::discover_git_repository(&path);
// assert!(repository.is_empty.is_ok());
// ```
// ```rust,should_panic
// #use std::path::Path;
// #use knapsac_lib::utils;
// #let path = Path::new("/etc");
// #let repository = utils::discover_git_repository(&path);
// ```

// ```
// use std::path::Path;
// use knapsac_lib::utils;
// let path = Path::new(format!("{}/test_data/mock_package_a", env!("CARGO_MANIFEST_DIR")).as_str());
// let working_dir = utils::infer_working_dir(&path);
// assert_eq!(working_dir, path.to_path_buf());
// ```
// ```rust,should_panic
// use knapsac_lib::utils;
// use std::path::Path;
// let path = Path::new("/etc");
// let repository = discover_git_repository(&path);
// ```
