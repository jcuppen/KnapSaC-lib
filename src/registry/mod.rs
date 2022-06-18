mod add;
mod get;
mod has;
mod mark;
mod remove;
mod search;

use crate::dependency::{Dependency, HasDependencies};
use crate::module::Module;
use crate::package::Package;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Registry {
    packages: HashMap<String, Package>,
    items: HashMap<PathBuf, Module>,
}

impl Registry {
    fn registry_path() -> PathBuf {
        dirs::home_dir().unwrap().join("knapsac_registry.json")
    }

    fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(Self::registry_path(), contents).unwrap();
    }

    fn init() -> Self {
        Registry {
            packages: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn load() -> Self {
        if let Ok(data) = read_to_string(Self::registry_path()) {
            match serde_json::from_str(data.as_str()) {
                Err(e) => panic!("{}", e.to_string()),
                Ok(i) => return i,
            }
        }
        Registry::init()
    }

    pub fn dep_to_module(&self, dependency: &Dependency) -> Option<&Module> {
        if self.dependency_exists(dependency) {
            return match dependency {
                Dependency::Stray(_, _) => panic!(),
                Dependency::Standalone(s) => self.get_module(s),
                Dependency::Package(package_identifier, module_identifier) => self
                    .get_package(package_identifier)?
                    .get_module(module_identifier),
            };
        }
        None
    }

    pub fn package(&mut self, identifier: String, package_root: &PathBuf) {
        let mut package = Package::create(package_root.clone());

        // turn modules into package modules;
        self.search_modules_by_source_prefix(package_root)
            .iter()
            .for_each(|&(k, v)| {
                let rel = k.strip_prefix(package_root).unwrap().to_path_buf();
                package.add_module(rel, v);
            });

        let mut a: Vec<Module> = vec![];
        // remove and collect old standalone modules;
        for p in self
            .search_modules_by_source_prefix(package_root)
            .iter()
            .map(|&(k, _)| k.clone())
            .collect::<Vec<PathBuf>>()
        {
            a.push(self.remove_module(&p, false).unwrap());
        }

        // replace module dependencies with package module dependencies;
        self.items.values_mut().for_each(|v| {
            a.iter().for_each(
                |rm| match v.get_dependency(&rm.identifier.clone().unwrap()) {
                    None | Some(Dependency::Stray(_, _)) | Some(Dependency::Package(_, _)) => {}
                    Some(Dependency::Standalone(_)) => {
                        let module_identifier = rm.identifier.clone().unwrap();
                        let dependency = Dependency::Package(identifier.clone(), module_identifier.clone());
                        v.add_dependency(module_identifier, dependency);
                    }
                },
            )
        });

        self.packages.insert(identifier, package);
        self.save();
    }
}
