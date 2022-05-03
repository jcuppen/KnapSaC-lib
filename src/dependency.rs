use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
#[derive(Hash)]
pub struct Dependency {
    pub(crate) git_url: Url,
}
