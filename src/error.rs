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
}

#[derive(Debug, PartialEq)]
pub enum ModuleError {
    LocationNotRelative,
    LocationNotAbsolute,
    DoesNotExist,
    InvalidManifest,
    CyclicDependency,
}

#[derive(Debug)]
pub enum DependencyError {
    LocationNotRelative,
    DoesNotExist,
    LocationNotAbsolute,
}

pub(crate) enum ManifestError {
    InvalidManifest,
}
