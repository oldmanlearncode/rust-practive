use crate::RenameError;
use std::collections::HashMap;

pub struct WikiNotes {
    pages: HashMap<String, String>,
}

impl WikiNotes {
    pub fn new() -> Self { todo!() }
    pub fn len(&self) -> usize { todo!() }
    pub fn create(&mut self, name: String, body: String) -> Result<(), RenameError> { todo!() }
    pub fn body(&self, name: &str) -> Option<&str> { todo!() }
    pub fn rename(&mut self, old: &str, new: String) -> Result<(), RenameError> { todo!() }
    pub fn search(&self, keyword: &str) -> Vec<String> { todo!() }
    pub fn length_counts(&self) -> HashMap<usize, usize> { todo!() }
}
