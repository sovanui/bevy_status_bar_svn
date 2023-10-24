mod health;

use crate::health::Health;
use bevy::prelude::*;
use bevy_status_bar_svn::{definition::StatusBarDefinition, plugin::StatusBarPlugin};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StatusBarPlugin::<Health>::default()))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, update_health)
        .run();
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pbr_bundle: PbrBundle,
    health: Health,
    status_bar_definition: StatusBarDefinition<Health>,
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 40.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Spawn player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        //
        .insert(Health::new(100));

    commands.spawn(PlayerBundle {
        pbr_bundle: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        health: Health::new(100),
        status_bar_definition: StatusBarDefinition::<Health>::default(),
    });
    //

    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add global light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

fn update_health(mut health: Query<&mut Health>) {
    health.for_each_mut(|mut health| {
        health.add(1);
        if health.get_current() == 100 {
            health.remove(100);
        }
    });
}
