#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use game_library::save_load::{game_types::*, *};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(main))]
fn main() {
    setup_global_tracing_subscriber();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(SaveLoad::<GameSession>::default())
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, hello_world);
    app.world.send_event(SaveEvent::<GameSession>::new(StaticPath::new("test/".to_string(), "main_test.scn.ron".to_string())));
    app.run();
}

fn setup_global_tracing_subscriber() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry()
                .with(tracing_tracy::TracyLayer::new()),
        ).expect("Successfully set global tracing subscriber");
    }
}

fn hello_world() {
    // On wasm this won't work, but it won't throw an error either. 
    println!("Hello Bevy!");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Name::new("Box 1"),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Cube::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
            transform: Transform::from_xyz(-50., -50., 0.).with_scale(Vec3::new(100.0, 100.0, 100.0)).with_rotation(Quat::from_axis_angle(Vec3::new(0.5, 0.5, 0.5), 0.8)),
            ..MaterialMesh2dBundle::default()
        },
    ));

    commands.spawn((
        Name::new("Box 2"),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Cube::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.0, 1.0, 0.0))),
            transform: Transform::from_xyz(50., 50., 0.).with_scale(Vec3::new(100.0, 100.0, 100.0)).with_rotation(Quat::from_axis_angle(Vec3::new(0.5, 0.5, 0.5), 0.8)),
            ..MaterialMesh2dBundle::default()
        },
    ));

    commands.spawn((
        Name::new("Save test entity 1"),
        Save::<GameSession>::default(),
    ));

    
}