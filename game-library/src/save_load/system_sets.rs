use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;



#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct SaveSet;

#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct LoadSet;