use std::marker::PhantomData;
use bevy::prelude::*;

/// Marker component for indicating that this entity should be saved. This marker does nothing on its own.
/// Combine with either a GameSession or UserSession marker component to indicate how it should be saved.
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Save<T> 
    where
    T: TypePath + Default + Send + Sync + 'static
{
    #[reflect(ignore)]
    _pd: PhantomData<T>,
}

impl<T> Component for Save<T> 
    where
    T: TypePath + Default + Send + Sync + 'static,
{
    type Storage = bevy_ecs::component::SparseStorage;
}