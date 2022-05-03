use crate::package::RegistrationStatus::{Known, Registered};
use crate::dependency::Dependency;

use std::fs::{read_to_string, write};
use git2::string_array::StringArray;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) enum RegistrationStatus {
    Registered,
    Known,
}

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub(crate) struct Package {
    pub(crate) registration_status: RegistrationStatus,
    pub(crate) local_location: PathBuf,
    pub(crate) remote_location: Option<Url>,
}

pub(crate) fn create_package<P: Clone + AsRef<Path>>(local_repository_root: P, repository: Repository) -> Package {
    let string_array = repository.remotes().unwrap();

    if string_array.is_empty() {
        return create_known_package(local_repository_root.as_ref().to_path_buf());
    }

    create_registered_package(local_repository_root.as_ref().to_path_buf(), repository, string_array)
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
        remote_location: Url::parse(remote.url().unwrap()).ok(),
    }
}

fn create_known_package(local_repository_root: PathBuf) -> Package {
    Package {
        registration_status: Known,
        local_location: local_repository_root,
        remote_location: None,
    }
}

impl Package {
    fn manifest_location(&self) -> PathBuf {
        let mut path: PathBuf = self.local_location.clone();
        path.push("dependencies");
        path.set_extension("json");
        path
    }

    pub(crate) fn add_dependency(&self, url: Url) {
        let dependency = Dependency { git_url: url };
        let result = read_to_string(self.manifest_location());
        let mut dependencies: Vec<Dependency> = vec![];

        if let Ok(data) = result {
            dependencies = serde_json::from_str(&*data).unwrap();
        }

        dependencies.push(dependency);
        dependencies.sort();
        dependencies.dedup();
        let contents = serde_json::to_string(&dependencies).unwrap();
        write(self.manifest_location(), contents).unwrap()
    }

    pub(crate) fn get_dependencies(&self) -> Vec<Dependency> {
        if let Ok(data) = read_to_string(self.manifest_location())  {
            let d: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
            return d
        }
        return vec![];
    }

    pub(crate) fn remove_dependency(&self, value: Url) {
        let dep_to_remove = Dependency { git_url: value };
        if let Ok(data) = read_to_string(self.manifest_location()) {
            let mut dependencies: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
            if let Some(index) = dependencies.iter().position(|d| d == &dep_to_remove) {
                dependencies.remove(index);
            }
            let contents = serde_json::to_string(&dependencies).unwrap();
            write(self.manifest_location(), contents).unwrap()
        }
    }
}
