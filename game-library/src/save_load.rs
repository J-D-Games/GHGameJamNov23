//! Plugin for adding save/load functionality to the game
 
use bevy::prelude::*;

struct SaveLoad {

}

impl Plugin for SaveLoad {
    fn build(&self, app: &mut App) {
        let test = Save {group: Test::Var1};
        todo!()
    }
}

enum Test {
    Var1,
}

pub struct Save<G> 
    where
    G: Send + Sync + 'static,
{
    group: G,
}


impl<G> Component for Save<G> 
    where
    G: Send + Sync + 'static,
{
    type Storage = bevy_ecs::component::SparseStorage;
}

