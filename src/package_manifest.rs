use std::fs::{read_to_string, write};
use std::path::{PathBuf};
use crate::version::{SemVerIncrement, Version};
use crate::version::Version::SemVer;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Deserialize, Serialize, Debug, Default)]
pub(crate) struct PackageManifest {
    pub(crate) version: Version,
    pub(crate) remote_location: Option<Url>,
}

impl PackageManifest {
    pub(crate) fn create() -> Self {
        PackageManifest {
            version: Version::NotVersioned,
            remote_location: None,
        }
    }

    pub(crate) fn load(manifest_path: PathBuf) -> PackageManifest {
        if let Ok(data) = read_to_string(manifest_path) {
            match serde_json::from_str(data.as_str()) {
                Err(e) => panic!("{}", e.to_string()),
                Ok(i) => return i,
            }
        }
        PackageManifest::create()
    }

    pub(crate) fn increment_version(&mut self, version_increment: SemVerIncrement) {
        let new_version = match self.version {
            Version::NotVersioned => {
                match version_increment {
                    SemVerIncrement::Major => SemVer(1,0,0),
                    SemVerIncrement::Minor => SemVer(0,1,0),
                    SemVerIncrement::Patch => SemVer(0,0,1),
                }
            }
            SemVer(major, minor, patch) => {
                match version_increment {
                    SemVerIncrement::Major => SemVer(major + 1,0,0),
                    SemVerIncrement::Minor => SemVer(major,minor + 1,0),
                    SemVerIncrement::Patch => SemVer(major,minor,patch + 1),
                }
            }
        };
        self.version = new_version;
    }

    pub(crate) fn save(&self, manifest_path: PathBuf) {
        let contents = serde_json::to_string(self).unwrap();
        write(manifest_path, contents).unwrap();
    }
}
