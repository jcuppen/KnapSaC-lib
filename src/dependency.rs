use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Hash)]
pub struct Dependency {
    pub(crate) git_url: Url,
}

impl Dependency {
    pub fn create(url: Url) -> Self {
        Dependency {
            git_url: url,
        }
    }
}
