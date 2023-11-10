use std::marker::PhantomData;
use bevy::prelude::*;
use crate::save_load::traits::*;

/// Event type for saving the current game session.
#[derive(Event)]
pub struct SaveEvent<T> 
    where
    T: Send + Sync + 'static
{
    path_getter: Box<dyn SaveLoadPath + Send + Sync>,
    _pd: PhantomData<T>,
}

impl<T> SaveEvent<T> 
    where
    T: Send + Sync + 'static,
{
    /// Constructor for the SaveEvent::<T> event.
    pub fn new<P: SaveLoadPath + Send + Sync + 'static>(path: P) -> Self {
        Self { path_getter: Box::new(path), _pd: PhantomData }
    }
}

impl<T> SaveLoadPath for SaveEvent<T>
    where
    T: Send + Sync + 'static,
{
    fn get_path(&self, world: &World) -> String {
        self.path_getter.get_path(world)
    }

    fn get_file(&self, world: &World) -> String {
        self.path_getter.get_file(world)
    }
}

/// Event used to broadcast the result of a save attempt.
#[derive(Event)]
pub struct SaveResultEvent<T>
    where
    T: Send + Sync + 'static,
{
    _pd: PhantomData<T>,
}

/// Event type for loading the saved game session.
#[derive(Event)]
pub struct LoadEvent<T> 
    where
    T: Send + Sync + 'static
{
    path_getter: Box<dyn SaveLoadPath + Send + Sync>,
    _pd: PhantomData<T>,
}

impl<T> SaveLoadPath for LoadEvent<T>
    where
    T: Send + Sync + 'static,
{
    fn get_path(&self, world: &World) -> String {
        self.path_getter.get_path(world)
    }

    fn get_file(&self, world: &World) -> String {
        self.path_getter.get_file(world)
    }
}

/// Event used to broadcast the result of a load attempt.
#[derive(Event)]
pub struct LoadResultEvent<T>
    where
    T: Send + Sync + 'static,
{
    _pd: PhantomData<T>,
}