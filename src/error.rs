#[derive(Debug)]
pub struct FailedToLoadManifestError;

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
}

#[derive(Debug)]
pub enum DependencyError {
    LocationNotRelative,
    DoesNotExist,
    LocationNotAbsolute,
}

pub(crate) enum ManifestError {
    NoManifestFound,
}