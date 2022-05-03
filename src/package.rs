
use crate::package::RegistrationStatus::{Known, Registered};
use crate::manifest::Manifest;

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

    let package = match string_array.is_empty() {
        true => create_known_package(local_repository_root.as_ref().to_path_buf()),
        false => create_registered_package(local_repository_root.as_ref().to_path_buf(), repository, string_array),
    };

    Manifest::initialize().save(package.manifest_location());
    package
}

fn create_registered_package(
    local_repository_root: PathBuf,
    repository: Repository,
    remotes: StringArray,
) -> Package {
    let remote_name_str = remotes.get(0).unwrap();
    let remote = repository.find_remote(remote_name_str).unwrap();

    let package = Package {
        registration_status: Registered,
        local_location: local_repository_root,
        remote_location: Url::parse(remote.url().unwrap()).ok(),
    };

    Manifest::initialize().save(package.manifest_location());

    package
}

fn create_known_package(local_repository_root: PathBuf) -> Package {
    let package = Package {
        registration_status: Known,
        local_location: local_repository_root,
        remote_location: None,
    };

    Manifest::initialize().save(package.manifest_location());

    package
}

impl Package {
    fn load_manifest(&self) -> Manifest {
        Manifest::load(self.manifest_location())
    }

    fn manifest_location(&self) -> PathBuf {
        let mut path: PathBuf = self.local_location.clone();
        path.push("dependencies");
        path.set_extension("json");
        path
    }

    pub(crate) fn add_dependency(&self, url: Url) {
        let mut manifest = self.load_manifest();
        manifest.add_dependency(url);
        manifest.save(self.manifest_location());
    }

    // pub(crate) fn get_dependencies(&self) -> Vec<Dependency> {
    //     if let Ok(data) = read_to_string(self.manifest_location())  {
    //         let d: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
    //         return d
    //     }
    //     return vec![];
    // }

    pub(crate) fn remove_dependency(&self, url: Url) {
        let mut manifest = self.load_manifest();
        manifest.remove_dependency(url);
        manifest.save(self.manifest_location());
    }
}
