use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;

pub struct VariablMap {
    variable : HashMap<(String, Option<String>), String>
}

impl VariablMap {
    pub fn new() -> Self {
        Self {
            variable : HashMap::new()
        }
    }
}