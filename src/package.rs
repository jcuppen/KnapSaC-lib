use crate::package::RegistrationStatus::{Known, Registered};
use git2::string_array::StringArray;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub(crate) enum RegistrationStatus {
    Registered,
    Known,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Package {
    pub(crate) registration_status: RegistrationStatus,
    pub(crate) local_location: PathBuf,
    pub(crate) remote_location: Option<String>,
}

pub(crate) fn create_package(local_repository_root: PathBuf, repository: Repository) -> Package {
    let string_array = repository.remotes().unwrap();

    if string_array.is_empty() {
        return create_known_package(local_repository_root);
    }

    create_registered_package(local_repository_root, repository, string_array)
}

fn create_registered_package(
    local_repository_root: PathBuf,
    repository: Repository,
    remotes: StringArray,
) -> Package {
    let remote_name_str = remotes.get(0).unwrap();
    let remote = repository.find_remote(remote_name_str).unwrap();

    Package {
        registration_status: Registered,
        local_location: local_repository_root,
        remote_location: Some(String::from(remote.url().unwrap())),
    }
}

fn create_known_package(local_repository_root: PathBuf, /*manifest_location: PathBuf*/) -> Package {
    Package {
        registration_status: Known,
        local_location: local_repository_root,
        remote_location: None,
    }
}

impl Package {
    // fn manifest_location(&self) -> PathBuf {
    //     let mut p = self.local_location.clone();
    //     p.push("dependencies");
    //     p.set_extension("json");
    //     p
    // }

    // pub(crate) fn add_dependency(&self, value: String) {
    //     let new_dep = Dependency { git_url: value };
    //     if let Ok(data) = read_to_string(self.manifest_location()) {
    //         let mut dependencies: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
    //         dependencies.push(new_dep);
    //         dependencies.sort();
    //         dependencies.dedup();
    //         let contents = serde_json::to_string(&dependencies).unwrap();
    //         write(self.manifest_location(), contents).unwrap()
    //     }
    // }

    // pub(crate) fn remove_dependency(&self, value: String) {
    //     let dep_to_remove = Dependency { git_url: value };
    //     if let Ok(data) = read_to_string(self.manifest_location()) {
    //         let mut dependencies: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
    //         if let Some(index) = dependencies.iter().position(|d| d == &dep_to_remove) {
    //             dependencies.remove(index);
    //         }
    //         let contents = serde_json::to_string(&dependencies).unwrap();
    //         write(self.manifest_location(), contents).unwrap()
    //     }
    // }
}
