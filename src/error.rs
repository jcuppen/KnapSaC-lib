#[derive(Debug, PartialEq)]
pub struct NotAPackageError;

#[derive(Debug, PartialEq)]
pub(crate) enum RepositoryError {
    BareRepository,
    RepositoryDiscoveryFailed,
}

#[derive(Debug, PartialEq)]
pub enum PackageError {
    NoRemoteLocation,
    NotARepository,
    PackageRootNotADirectory,
    DownloadFailed,
    InvalidManifest,
}

#[derive(Debug, PartialEq)]
pub enum RegistryError {
    RegistryPathNotAbsolute,
    RegistryPathNotJSON,
    RegistryPathNotFile,
    NoRegistryFound,
    InvalidRegistry,
    ModuleAlreadyInRegistry,
}

#[derive(Debug, PartialEq)]
pub enum ModuleError {
    // SourceLocationNotAbsolute,
    OutputLocationNotAbsolute,
    // SourceLocationNotRelative,
    OutputLocationNotRelative,
    // SourceLocationDoesNotExist,
    OutputLocationDoesNotExist,
    OutputLocationNotADirectory,
    // InvalidManifest,
    CyclicDependency,
    NoSuchDependency,
    RegistryError(RegistryError),
}

#[derive(Debug)]
pub enum DependencyError {
    LocationNotRelative,
    DoesNotExist,
    LocationNotAbsolute,
}
//
// pub(crate) enum ManifestError {
//     InvalidManifest,
// }
