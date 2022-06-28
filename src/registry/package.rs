use crate::dependency::{Dependency, HasDependencies};
use crate::language::Language;
use crate::module::Module;
use crate::package::Package;
use crate::registry::Registry;
use crate::version::{SemVerIncrement, Version};
use git2::Repository;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

impl Registry {
    fn modules_to_package_modules(&mut self, package_root: &Path, package: &mut Package) {
        self.search_modules_by_source_prefix(package_root)
            .iter()
            .for_each(|&(k, m)| {
                if !m.has_only_package_module_dependencies() {
                    panic!(
                        "Module '{}' @ '{}' still depends on modules that are not Package Modules",
                        &m.identifier.clone().unwrap(),
                        k.display()
                    )
                }

                let relative_source_path = k.strip_prefix(package_root).unwrap().to_path_buf();

                let mut package_module = m.clone();
                package_module.output_path =
                    PathBuf::from(m.identifier.clone().unwrap()).join("output");
                create_dir(package_root.join(&package_module.output_path)).unwrap();

                package.add_module(relative_source_path, package_module);
            });
    }

    fn module_dependencies_to_package_module_dependencies(
        &mut self,
        removed_modules: Vec<Module>,
        identifier: &str,
    ) {
        self.items.values_mut().for_each(|v| {
            removed_modules.iter().for_each(|rm| {
                match v.get_dependency(&rm.identifier.clone().unwrap()) {
                    None | Some(Dependency::Stray(_, _)) | Some(Dependency::Package(_, _)) => {}
                    Some(Dependency::Standalone(_)) => {
                        let module_identifier = rm.identifier.clone().unwrap();
                        let dependency =
                            Dependency::Package(identifier.to_string(), module_identifier.clone());
                        v.add_dependency(module_identifier, dependency);
                    }
                }
            })
        });
    }

    pub fn package(
        &mut self,
        identifier: &str,
        package_root: &Path,
        compiler_command_name: String,
        output_option: String,
    ) {
        if self.has_package(identifier) {
            panic!("Package with package_root '{}' already exists!", identifier);
        }

        let mut package = Package::create(
            identifier.to_string(),
            Language {
                compiler_command_name,
                output_option,
            },
        );

        Repository::open(package_root)
            .or_else(|_| Repository::init(package_root))
            .unwrap();

        self.modules_to_package_modules(package_root, &mut package);

        // remove and collect old standalone modules;
        let paths_to_modules_to_remove: Vec<PathBuf> = self
            .search_modules_by_source_prefix(package_root)
            .iter()
            .map(|&(k, _)| k.clone())
            .collect();

        let removed_modules = self.remove_and_collect(paths_to_modules_to_remove);

        // replace module dependencies with package module dependencies;
        self.module_dependencies_to_package_module_dependencies(
            removed_modules,
            identifier,
        );

        package.build(package_root);

        self.packages.insert(package_root.to_path_buf(), package);
        self.save();
    }

    fn add_files_to_git(package_root: &Path, package: &Package) {
        Command::new("git")
            .current_dir(package_root)
            .arg("add")
            .arg("manifest.json")
            .output()
            .expect("failed to commit changes");

        for (source_path, module) in package.modules.values() {
            Command::new("git")
                .current_dir(package_root)
                .arg("add")
                .arg(source_path)
                .output()
                .expect("failed to commit changes");

            Command::new("git")
                .current_dir(package_root)
                .arg("add")
                .arg(&module.output_path)
                .output()
                .expect("failed to commit changes");
        }
    }

    pub fn publish(&mut self, identifier: &str, increment: SemVerIncrement) {
        if let Some((package_root,package)) = self.get_package_mut(identifier) {
            package.increment_version(increment);
            Repository::open(&package_root).unwrap();

            assert_ne!(package.version, Version::NotVersioned);

            let msg = format!("updated to version: {}", package.version);

            Self::add_files_to_git(package_root, package);

            Command::new("git")
                .current_dir(package_root)
                .arg("commit")
                .arg("-m")
                .arg(msg)
                .output()
                .expect("failed to commit changes");

            Command::new("git")
                .current_dir(package_root)
                .arg("tag")
                .arg(package.version.to_string())
                .output()
                .expect("failed to tag commit");
        }
        self.save();
    }

    pub fn upload(&mut self, identifier: &str, git_url: Option<Url>) {
        if let Some((package_root, package)) = self.get_package_mut(identifier) {
            if package.remote_location.is_none() {
                package.remote_location = Some(git_url.unwrap())
            }

            Command::new("git")
                .current_dir(package_root)
                .arg("add")
                .arg("manifest.json")
                .output()
                .expect("failed to add manifest");
            Command::new("git")
                .current_dir(package_root)
                .arg("commit")
                .arg("--amend")
                .arg("--no-edit")
                .output()
                .expect("failed to commit changes");

            Repository::open(package_root).unwrap();
            Command::new("git")
                .current_dir(package_root)
                .arg("remote")
                .arg("add")
                .arg("origin")
                .arg(package.remote_location.as_ref().unwrap().as_str())
                .output()
                .expect("failed add remote");

            Command::new("git")
                .current_dir(package_root)
                .arg("push")
                .arg("-u")
                .arg("origin")
                .arg("master")
                .output()
                .expect("failed to push");
        }
        self.save();
    }
}
