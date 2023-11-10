//! Plugin for adding save/load functionality to the game

mod events;
mod components;
mod traits;
mod paths;
mod system_sets;

use std::{fs::File, io::Write, marker::PhantomData};
use bevy::{prelude::*, tasks::IoTaskPool};
pub use crate::save_load::{components::*, events::*, paths::*, traits::*, system_sets::*};



/// SaveLoad plugin for adding a save/load system to the game.
/// The system looks for objects tagged with the Save<T> component. 
/// Saving and loading is triggered by SaveEvent<T> and LoadEvent<T> respectively.
/// Supplying SaveEvent<T>'s and LoadEvent<T>'s new function with a trait object that implements SaveLoadPath specifies the save/load path.
/// This plugin will panic if Save<T> is attached to an entity containing components that implement Reflect, but not ReflectSerialize (such as Handle<M>)
#[derive(Default)]
pub struct SaveLoad<T> 
    where
    T: TypePath + Send + Sync + 'static,
{
    _pd: PhantomData<T>,
}

impl<T> Plugin for SaveLoad<T> 
    where
    T: TypePath + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.register_type::<Save<T>>()
            .add_event::<SaveEvent<T>>()
            .add_event::<SaveResultEvent<T>>()
            .add_event::<LoadEvent<T>>()
            .add_event::<LoadResultEvent<T>>()
            .add_systems(Last, save::<T>.in_set(SaveSet))
            .add_systems(First, load::<T>.in_set(LoadSet));
    }
}

fn save<T: TypePath + Default + Send + Sync + 'static>(
    (world, mut ev_save): (&World, EventReader<SaveEvent<T>>),
    query_filter: Query<Entity, With<Save<T>>>,
) {
    // If there are any new save events,
    if !ev_save.is_empty() {
        // Go through each of them,
        for ev in ev_save.read() {
            // Find all entities in the world containing the Save<T> component, remove the Save<T> component, and then remove empty entities and build the scene.
            // (Without removing the Save<T> component, we would be saving entities that contain no information except that they should be saved, which is useless
            // So we remove the component and can then remove empty entities. The Save<T> will be readded on in load::<T>())
            let scene_builder = DynamicSceneBuilder::from_world(&world)
                .deny::<Save<T>>()
                .extract_entities(query_filter.iter())
                .remove_empty_entities();
            let dynamic_scene = scene_builder.build();
            // Serialize the data. This will panic if there are any components that implement Reflect, but not ReflectSerialize (such as Handle<M>).
            let save_data = dynamic_scene
                .serialize_ron(&world.resource::<AppTypeRegistry>())
                .expect("We should be able to serialize our scene.");

            println!("{}", save_data.clone()); //Debug, remove later.
            
            // Save the file contents based on the path and file name passed in with the event.
            save_file(format!("{}{}", ev.get_path(world), ev.get_file(world)), save_data);
        }
    }
}

fn load<T: TypePath + Default + Send + Sync + 'static>(
    (mut commands, asset_server, mut ev_load_game_session): (
        Commands,
        Res<AssetServer>,
        EventReader<LoadEvent<T>>,
    ),
) {
}

fn save_file(file_path_name: String, file_contents: String) {
    #[cfg(target_arch = "wasm32")]
    {
        todo!("Wasm saving not yet implemented")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        IoTaskPool::get()
            .spawn(async move {
                File::create(file_path_name)
                    .and_then(|mut file| file.write(file_contents.as_bytes()))
            })
            .detach();
    }
}

fn load_file(file_path_name: &String) {
    #[cfg(target_arch = "wasm32")]
    {
        todo!("Wasm saving not yet implemented")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        todo!()
    }
}



/// Module containing save/load types that are specific to this current game jam game.
pub mod game_types {
    use bevy::prelude::*;

    /// Marker component for indicating that this entity should be saved and loaded with the game session.
    /// This should be used for entities that are local to the current game session, such as the player, units, tiles, etc.
    #[derive(Reflect, Default)]
    #[reflect(Component)]
    pub struct GameSession;

    impl Component for GameSession {
        type Storage = bevy_ecs::component::SparseStorage;
    }

    /// Marker component for indicating that this entity should be saved and loaded with the user session.
    /// This should be used for entities that are permanent between game sessions, such as unlock progress.
    #[derive(Reflect, Default)]
    #[reflect(Component)]
    pub struct UserSession;

    impl Component for UserSession {
        type Storage = bevy_ecs::component::SparseStorage;
    }
}


#[cfg(test)]
mod tests {
    use super::{*, game_types::*};
    use std::time::Duration;

    #[derive(Component, Default, Reflect)]
    #[reflect(Component)]
    struct ComponentA {
        x: u32,
        y: u32,
    }

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    struct ComponentB {
        value: String,
        #[reflect(skip_serializing)]
        _time_since_startup: Duration,
    }

    impl FromWorld for ComponentB {
        fn from_world(world: &mut World) -> Self {
            let time = world.resource::<Time>();
            ComponentB {
                _time_since_startup: time.elapsed(),
                value: "Default Value".to_string(),
            }
        }
    }

    #[derive(Component)]
    struct ComponentC {
        _rot_x: f32,
        _rot_y: f32,
        _rot_z: f32,
    }

    #[derive(Resource, Reflect, Default)]
    #[reflect(Resource)]
    struct ResourceA {
        score: u32,
    }

    fn test_runner(mut app: App) {
        for _ in 0..500000 {
            app.update();
        }
    }

    #[test]
    fn test_save_load_game_session() {
        //Create a new app for our test
        let mut app = App::new();
        //Use the minimum amount of setup to get it ready to test our system.
        //In this case we must register reflected components/resources and add the event.
        app.set_runner(test_runner)
            .register_type::<ComponentA>()
            .register_type::<ComponentB>()
            .register_type::<ResourceA>()
            .add_plugins(TaskPoolPlugin::default())
            .add_event::<SaveEvent<GameSession>>()
            .add_systems(Last, save::<GameSession>);

        {
            //Setup our world with some example resources and entities.
            let world = &mut app.world;
            world.insert_resource::<ResourceA>(ResourceA { score: 100 });
            world.spawn((
                ComponentA { x: 10, y: 11 },
                ComponentB {
                    value: "Entity 1's component B!".to_string(),
                    _time_since_startup: Duration::from_secs(2),
                },
                ComponentC {
                    _rot_x: 32.5,
                    _rot_y: 52.3,
                    _rot_z: 22.2,
                },
                Save::<GameSession>::default(),
            ));
            world.spawn((
                ComponentC {
                    _rot_x: 32.5,
                    _rot_y: 52.3,
                    _rot_z: 22.2,
                },
                Save::<GameSession>::default(),
            ));
            world.spawn((ComponentA { x: 10, y: 11 },));
            world.send_event(SaveEvent::<GameSession>::new(StaticPath::new("test/".to_string(), "test_file.scn.ron".to_string())));
        }
        app.run();
    }
}
