mod plugin;
mod definition;
mod material;
mod percentage;
mod health;

use bevy::prelude::*;
use crate::definition::StatusBarDefinition;
use crate::health::Health;
use crate::plugin::StatusBarPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatusBarPlugin::<Health>::default()))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, (move_cube, update_health))
        .run();
}


#[derive(Component)]
struct Cube;


fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1000.0,
    });


    // Spawn platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
        material: materials.add(Color::SEA_GREEN),
        ..default()
    });


    // Spawn cube with status bar
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::BEIGE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },

        StatusBarDefinition::<Health>::default(),
        Health::new(100),
        Cube
    ));

}


fn move_cube(
    mut cube_query: Query<&mut Transform, With<Cube>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {

    let mut cube_transform = cube_query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        cube_transform.translation.z -= 0.1;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        cube_transform.translation.z += 0.1;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        cube_transform.translation.x += 0.1;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cube_transform.translation.x -= 0.1;
    }

}

fn update_health(mut health: Query<&mut Health>) {
    health.iter_mut().for_each(|mut health| {
        health.add(1);
        if health.get_current() == 100 {
            health.remove(100);
        }
    });
}


