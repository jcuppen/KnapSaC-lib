use std::path::PathBuf;
use crate::module::Module;
use crate::registry::Registry;

impl Registry {
    pub fn get_modules(&self, identifier: &str) -> Vec<(&PathBuf, &Module)> {
        self.items
            .iter()
            .filter(|(_,v)| {
                if let Some(id) = &v.identifier {
                    id == identifier
                } else {
                    false
                }
            })
            .collect()
    }
}
