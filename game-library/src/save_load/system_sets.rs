use bevy::prelude::*;


/// System set containing all the Save::<T> systems added by this plugin. Runs in the Last schedule.
#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct SaveSet;

/// System set containing all the Load::<T> systems added by this plugin. Runs in the First schedule.
#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct LoadSet;