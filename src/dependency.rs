use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
pub struct PackageDependency {
    pub(crate) git_url: Url,
}

impl PackageDependency {
    pub fn create(url: Url) -> Self {
        PackageDependency { git_url: url }
    }
}

// #[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
// pub struct ModuleDependency {
//     pub(crate) location: PathBuf,
// }
//
// impl ModuleDependency {
//     pub fn create(module: &StandaloneModule) -> Result<Self, DependencyError> {
//         let p = &module.location;
//
//         if !p.exists() {
//             return Err(DoesNotExist);
//         }
//         if !p.is_absolute() {
//             return Err(LocationNotAbsolute);
//         }
//
//         Ok(ModuleDependency {
//             location: module..to_path_buf(),
//         })
//     }
// }
