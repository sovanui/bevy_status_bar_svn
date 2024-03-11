mod plugin;
mod definition;

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};


#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StatusBarMaterial {
    #[uniform(0)] pub foreground_color: Color,
    #[uniform(0)] pub background_color: Color,
    #[uniform(0)] pub percent: f32
}

impl Material for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef { "bar.wgsl".into() }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<StatusBarMaterial>::default())
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, update_bar)
        .run();
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
) {
    let camera_translation = Vec3::new(0.0,  20.0, 10.0);

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(camera_translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1000.0,
    });

    // Spawn platform for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
        material: materials.add(Color::SEA_GREEN),
        ..default()
    });


    // Spawn bar
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Rectangle::new(5., 1.)),
        material: status_bar_materials.add(StatusBarMaterial {
            foreground_color: Color::GREEN,
            background_color: Color::RED,
            percent: 0.25,
        } ),
        transform: Transform::from_xyz(0.0, 0.5, 0.0).looking_at(-camera_translation, Vec3::Y),
        ..default()
    });

}

fn update_bar(
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
    status_bar_query: Query<&Handle<StatusBarMaterial>>
) {
    let handle = status_bar_query.single();
    let material = status_bar_materials
        .get_mut(handle)
        .expect("StatusBarMaterial missing");

    if material.percent >= 1.0 {
        material.percent = 0.0;
    } else {
        material.percent += 0.01;
    }
}




