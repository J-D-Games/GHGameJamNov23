//! Plugin for adding save/load functionality to the game

use bevy::prelude::*;

struct SaveLoad {}

impl SaveLoad {
    pub const USER_SESSION_FILE_PATH: &str = "";
    pub const GAME_SESSION_FILE_PATH: &str = "";
}

impl Plugin for SaveLoad {
    fn build(&self, app: &mut App) {
        app.register_type::<Save>()
            .register_type::<GameSession>()
            .register_type::<UserSession>()
            .add_event::<SaveGameSessionEvent>()
            .add_event::<SaveUserSessionEvent>()
            .add_event::<LoadGameSessionEvent>()
            .add_event::<LoadUserSessionEvent>()
            .add_systems(Last, (save_game_session, save_user_session))
            .add_systems(First, (load_game_session, load_user_session));
    }
}

fn save_game_session(
    (world, mut ec_save_game_session): (&World, EventReader<SaveGameSessionEvent>),
    query_filter: Query<Entity, (With<Save>, With<GameSession>, Without<UserSession>)>,
) {
    if !ec_save_game_session.is_empty() {
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        let scene_builder = DynamicSceneBuilder::from_world(&world)
            .extract_entities(query_filter.iter())
            .deny::<Save>()
            .deny::<GameSession>()
            .remove_empty_entities();
        let dynamic_scene = scene_builder.build();
        let save_data = dynamic_scene
            .serialize_ron(&type_registry)
            .expect("We should be able to serialize our scene.");
        println!("{}", save_data.clone());
        save_file("game_session", save_data);
        ec_save_game_session.clear();
    }
}

fn save_user_session(
    (world, mut ec_save_user_session): (&World, EventReader<SaveUserSessionEvent>),
    query_filter: Query<Entity, (With<Save>, With<UserSession>, Without<GameSession>)>,
) {
    if !ec_save_user_session.is_empty() {
        let type_registry = world.resource::<AppTypeRegistry>().clone();
        let scene_builder = DynamicSceneBuilder::from_world(&world)
            .extract_entities(query_filter.iter())
            .deny::<Save>()
            .deny::<UserSession>()
            .remove_empty_entities();
        let dynamic_scene = scene_builder.build();
        let save_data = dynamic_scene
            .serialize_ron(&type_registry)
            .expect("We should be able to serialize our scene.");
        println!("{}", save_data.clone());
        save_file("user_session", save_data);
        ec_save_user_session.clear();
    }
}

fn load_game_session(
    (mut commands, asset_server, mut ev_load_game_session): (
        Commands,
        Res<AssetServer>,
        EventReader<LoadGameSessionEvent>,
    ),
) {
}

fn load_user_session(
    (mut commands, asset_server, mut ev_load_user_session): (
        Commands,
        Res<AssetServer>,
        EventReader<LoadUserSessionEvent>,
    ),
) {
}

fn save_file(file_name: &str, file_contents: String) {
    // #[cfg(target_arch = "wasm32")]

    // #[cfg(not(target_arch = "wasm32"))]
}

fn load_file(file_name: &str) {}

/// Marker component for indicating that this entity should be saved. This marker does nothing on its own.
/// Combine with either a GameSession or UserSession marker component to indicate how it should be saved.
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct Save;

impl Component for Save {
    type Storage = bevy_ecs::component::SparseStorage;
}

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

/// Event type for saving the current game session.
#[derive(Event)]
pub struct SaveGameSessionEvent;

/// Event type for saving the current user session.
#[derive(Event)]
pub struct SaveUserSessionEvent;

/// Event type for loading the saved game session.
#[derive(Event)]
pub struct LoadGameSessionEvent;

/// Event type for loading the saved user session.
#[derive(Event)]
pub struct LoadUserSessionEvent;

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_save_load_game_session() {
        //Create a new app for our test
        let mut app = App::new();
        //Use the minimum amount of setup to get it ready to test our system.
        //In this case we must register reflected components/resources and add the event.
        app.register_type::<ComponentA>()
            .register_type::<ComponentB>()
            .register_type::<ResourceA>()
            .add_event::<SaveGameSessionEvent>()
            .add_systems(Last, save_game_session);

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
                Save,
                GameSession,
            ));
            world.spawn((
                ComponentC {
                    _rot_x: 32.5,
                    _rot_y: 52.3,
                    _rot_z: 22.2,
                },
                Save,
                GameSession,
            ));
            world.spawn((ComponentA { x: 10, y: 11 },));
            world.send_event(SaveGameSessionEvent);
            // This will panic if our save function fails.
            world.run_schedule(Last);
        }
    }
}
