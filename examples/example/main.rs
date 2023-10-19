mod health;


use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_status_bar_svn::plugin::StatusBarPlugin;
use crate::health::Health;


fn main() {
    App::new().
        add_plugins((DefaultPlugins, StatusBarPlugin::<Health>::default()))
        .add_systems(Startup, spawn_scene)
        .run();
}



#[derive(Bundle)]
pub struct PlayerBundle {
    pbr_bundle: PbrBundle,
    health: Health,
    percentage_bar_definition: PercentageBarDefinition,
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

        //
        .insert(Health::new(100));


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



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_and_player_cam)
            .add_systems(Update, update_health)
            .add_plugins(PercentageBarPlugin::<Health>::default());
        // .add_systems(PostUpdate, camera_follow_player);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    _player: Player,
    pbr_bundle: PbrBundle,
    rigid_body: RigidBody,
    velocity: Velocity,
    destination_bundle: DestinationBundle,
    health: Health,
    percentage_bar_definition: PercentageBarDefinition,
}


fn spawn_player_and_player_cam(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_transform = Transform::from_xyz(0.0, 0.5, 0.0);

    commands.spawn(PlayerBundle {
        _player: Player,
        pbr_bundle: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: player_transform,
            ..default()
        },
        rigid_body: RigidBody::KinematicVelocityBased,
        velocity: Velocity::zero(),
        destination_bundle: DestinationBundle {
            destination: Destination::Reached,
            speed: Default::default(),
            rotation_speed: Default::default(),
        },
        health: Health::new(100),
        percentage_bar_definition: PercentageBarDefinition::default(),
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, PLAYER_CAMERA_Y_OFFSET, PLAYER_CAMERA_Z_OFFSET)
            .looking_at(player_transform.translation, Vec3::Y),
        ..default()
    });
}


fn update_health(mut health: Query<&mut Health, With<Player>>) {
    health.for_each_mut(|mut health| {
        health.add(1);
        if health.get_current() == 100 {
            health.remove(100);
        }
    });
}
