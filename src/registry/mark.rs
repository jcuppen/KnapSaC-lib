use std::path::{Path};
use crate::registry::Registry;

impl Registry {
    pub fn mark_as_module(&mut self, source_file: &Path, identifier: String) {
        let item = self.get_item_mut(source_file).unwrap();
        item.identifier = Some(identifier);
        self.save()
    }
}