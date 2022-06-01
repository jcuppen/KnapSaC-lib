extern crate core;

use std::borrow::BorrowMut;
use std::collections::{HashMap};
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;

pub mod registry;
pub(crate) mod module;
pub(crate) mod executable;

#[derive(Hash)]
#[derive(Deserialize, Serialize)]
#[derive(Eq, PartialEq, Clone)]
pub enum Dependency {
    StrayModule(PathBuf),
    StandaloneModule,
    PackageModule
}

pub(crate) trait HasDependencies {
    fn dependencies(&self) -> &HashMap<String, Dependency>;
    fn dependencies_mut(&mut self) -> &mut HashMap<String, Dependency>;

    fn add_dependency(&mut self, identifier: String, dependency: Dependency) {
        self.dependencies_mut().borrow_mut().insert(identifier, dependency);
    }

    // fn has_dependency(&self, dependency_identifier: &str) -> bool {
    //     self.dependencies().iter().any(|d|{
    //         match d {
    //             Dependency::StrayModule(id, _) => dependency_identifier == id,
    //             Dependency::StandaloneModule(id) => dependency_identifier == id,
    //             Dependency::PackageModule => { panic!() }
    //         }
    //     })
    // }

    fn get_dependency(&self, identifier: &str) -> Option<&Dependency> {
        self.dependencies().get(identifier)
    }

    fn remove_dependency(&mut self, identifier: &str) {
        self.dependencies_mut().borrow_mut().remove(identifier);
    }
}