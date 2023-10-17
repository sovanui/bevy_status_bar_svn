use std::marker::PhantomData;
use bevy::app::App;
use bevy::prelude::*;
use crate::components::health::Health;
use crate::components::percentage_bar::direction::Direction;
use crate::components::percentage_bar::orientation::Orientation;
use crate::components::percentage_bar::percentage::AsPercentage;
use crate::components::percentage_bar::shaders::PercentageBarMaterial;
use crate::components::percentage_bar::size::Size;


#[derive(Component)]
pub struct PercentageBarDefinition {
    size: Size,
    offset: Vec3,
    orientation: Orientation,
    direction: Direction,
    foreground_color: Color,
    background_color: Color,
}


impl Default for PercentageBarDefinition {
    fn default() -> Self {
        Self {
            size: Size::new(1.2, 0.20),
            offset: Vec3::new(0.0, 0.9, -0.7),
            orientation: Orientation::FacingCamera,
            direction: Direction::Horizontal,
            foreground_color: Color::GREEN,
            background_color: Color::RED,
        }
    }
}



pub struct PercentageBarPlugin<T: Component + AsPercentage>(PhantomData<T>);

impl <T: Component + AsPercentage> Default for PercentageBarPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl <T: Component + AsPercentage> Plugin for PercentageBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_percentage_bar)
            .add_systems(PostUpdate, follow_parent_position)
            .add_systems(Update, update_bar_percent::<T>)
            .add_plugins(MaterialPlugin::<PercentageBarMaterial>::default());
    }
}


#[derive(Component)]
pub struct Parent(Entity);

fn spawn_percentage_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PercentageBarMaterial>>,

    percentage_bar_query: Query<(&PercentageBarDefinition, Entity)>,
    camera_query: Query<&Transform, With<Camera3d>>
) {

    let camera = camera_query.single();

    percentage_bar_query.for_each(|(percentage_bar_definition, entity)| {
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(
                Vec2::new(
                    percentage_bar_definition.size.width(),
                    percentage_bar_definition.size.height()
                )))),
            material: materials.add(PercentageBarMaterial {
                foreground_color: percentage_bar_definition.foreground_color,
                background_color: percentage_bar_definition.background_color,
                percent: 0.5,
            }),
            transform: Transform::from_translation(percentage_bar_definition.offset)
                .with_rotation(camera.rotation),
            ..default()
        }).insert(Parent(entity));
    });
}

fn update_bar_percent<T: Component + AsPercentage>(
    mut materials: ResMut<Assets<PercentageBarMaterial>>,
    mut material_query: Query<(&Handle<PercentageBarMaterial>, &Parent), Without<PercentageBarDefinition>>,
    parent_query: Query<&Health>
) {
    material_query.for_each(|(handle, &Parent(parent_entity))| {

        if let Some(material) = materials.get_mut(handle) {
            let health = parent_query.get(parent_entity).unwrap();
            material.percent = health.percentage().values();
        }
    });
}

fn follow_parent_position(
    mut bar_query: Query<(&mut Transform, &Parent), Without<PercentageBarDefinition>>,
    parent_query: Query<(&Transform, &PercentageBarDefinition)>
) {

    bar_query.for_each_mut(|(mut transform, &Parent(entity))| {
        let (parent_transform, parent_bar_definition) = parent_query.get(entity).unwrap();
        let new_translation = parent_transform.translation + parent_bar_definition.offset;
        transform.translation = new_translation;
    });

}
