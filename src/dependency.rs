use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub struct Dependency {
    pub(crate) git_url: Url,
}
