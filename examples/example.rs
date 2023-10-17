use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_destination_svn::destination::{Destination, DestinationBundle, DestinationPlugin, RotationSpeed, Speed};

fn main() {
    App::new().
        add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            DestinationPlugin
    ))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, set_next_destination)
        .add_systems(Update, update_speed)
        .add_systems(Update, update_rotation_speed)
        .run();
}


fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Spawn platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0, subdivisions: 0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Spawn object
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::default())
        .insert(DestinationBundle {
            destination: Destination::Reached,
            speed: Speed::default(),
            rotation_speed: RotationSpeed::default()
        });


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
        directional_light: DirectionalLight { illuminance: 10000.0, ..default() },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

}

fn set_next_destination(
    mut query: Query<(&mut Destination, &Transform)>,
) {

    query.for_each_mut(|(mut destination, transform)| {
        match &mut *destination {

            Destination::Target(_) => {}

            // set new destination when reached previous
            Destination::Reached => {
                *destination = Destination::new(
                    // from
                    transform.translation,
                    // to
                    Vec3::new(
                            rand::random::<f32>() * 20.0 - 10.0,
                            0.5,
                            rand::random::<f32>() * 20.0 - 10.0
                        )
                );
            }
        }
    });

}


fn update_speed(
    keyboard_inputs: Res<Input<KeyCode>>,
    mut query: Query<&mut Speed>
) {
    query.for_each_mut(|mut speed| {
        if keyboard_inputs.just_pressed(KeyCode::Up) {
            speed.0 += 1.0;
        }

        if keyboard_inputs.just_pressed(KeyCode::Down) {
            speed.0 -= 1.0;
        }
    });
}

fn update_rotation_speed(
    keyboard_inputs: Res<Input<KeyCode>>,
    mut query: Query<&mut RotationSpeed>
) {
    query.for_each_mut(|mut rotation_speed| {
        if keyboard_inputs.just_pressed(KeyCode::Right) {
            rotation_speed.0 += 2.0;
        }

        if keyboard_inputs.just_pressed(KeyCode::Left) {
            rotation_speed.0 -= 2.0;
        }
    });
}
