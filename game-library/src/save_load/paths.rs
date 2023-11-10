use bevy::prelude::*;
use crate::save_load::traits::*;


/// Provided implementor of SaveLoadPath that simple provides a static path.
pub struct StaticPath {
    path: String,
    file: String,
}

impl StaticPath {
    /// Create a new StaticPath with the given path.
    pub fn new(path: String, file: String) -> Self {
        Self {
            path,
            file,
        }
    }
}

impl SaveLoadPath for StaticPath {
    fn get_path(&self, _world: &World) -> String {
        self.path.clone()
    }
    fn get_file(&self, _world: &World) -> String {
        self.file.clone()
    }
}