use bevy::prelude::*;

/// Trait for getting the file path for saving and loading.
pub trait SaveLoadPath {
    /// Specifies the save / load path.
    fn get_path(&self, world: &World) -> String;
    /// Specifies the save / load file.
    fn get_file(&self, world: &World) -> String;
}