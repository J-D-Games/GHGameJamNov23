#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(main))]
fn main() {
    setup_global_tracing_subscriber();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, hello_world)
        .run();
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

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Cube::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
            transform: Transform::default().with_scale(Vec3::new(100.0, 100.0, 100.0)).with_rotation(Quat::from_axis_angle(Vec3::new(0.5, 0.5, 0.5), 0.8)),
            ..MaterialMesh2dBundle::default()
        }
    );

}