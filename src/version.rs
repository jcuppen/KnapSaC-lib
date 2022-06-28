use std::fmt::{Display, Formatter};
use serde::Serialize;
use serde::Deserialize;
use crate::version::Version::NotVersioned;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) enum Version {
    NotVersioned,
    SemVer(usize, usize, usize),
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Version::NotVersioned => write!(f, "not_versioned"),
            Version::SemVer(major, minor, patch) => write!(f, "{}.{}.{}", major, minor, patch),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        NotVersioned
    }
}


pub enum SemVerIncrement {
    Major,
    Minor,
    Patch,
}
