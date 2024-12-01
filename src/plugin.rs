use crate::definition::{Direction, Orientation, StatusBarDefinition};
use crate::material::{StatusBarMaterial, BAR_SHADER_HANDLE};
use crate::percentage::AsPercentage;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use std::marker::PhantomData;

pub trait PercentageComponent: Component + AsPercentage {}
impl<T: Component + AsPercentage> PercentageComponent for T {}

pub struct StatusBarPlugin<T: PercentageComponent>(PhantomData<T>);

impl<T: PercentageComponent> Default for StatusBarPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: PercentageComponent> Plugin for StatusBarPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MaterialPlugin<StatusBarMaterial>>() {
            app.add_plugins(MaterialPlugin::<StatusBarMaterial>::default());
            load_internal_asset!(
                app,
                BAR_SHADER_HANDLE,
                "../assets/bar.wgsl",
                Shader::from_wgsl
            );
        }

        app.add_systems(PostStartup, spawn::<T>)
            .add_systems(PostUpdate, (follow_owner::<T>, update::<T>));
    }
}

type StatusBarMeshMaterial = MeshMaterial3d<StatusBarMaterial>;

#[derive(Component)]
#[require(StatusBarMeshMaterial)]
pub struct StatusBarOwner(Entity);

fn spawn<T: PercentageComponent>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,

    owner_query: Query<(&StatusBarDefinition<T>, &T, Entity, &Transform)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let camera = camera_query.single();

    owner_query.iter().for_each(
        |(status_bar_definition, percentage_component, entity, transform)| {
            let orientation_rotation = match status_bar_definition.orientation {
                Orientation::FacingCamera => camera.rotation,
            };

            let direction_rotation = match status_bar_definition.direction {
                Direction::Horizontal => Quat::default(),
                Direction::Vertical => Quat::from_rotation_z((90.0f32).to_radians()),
            };

            commands.spawn((
                StatusBarOwner(entity),
                Mesh3d(meshes.add(Rectangle::new(
                    status_bar_definition.size.width(),
                    status_bar_definition.size.height(),
                ))),
                MeshMaterial3d(status_bar_materials.add(StatusBarMaterial {
                    foreground_color: status_bar_definition.foreground_color.into(),
                    background_color: status_bar_definition.background_color.into(),
                    percent: percentage_component.percentage().value(),
                })),
                Transform::from_translation(transform.translation + status_bar_definition.offset)
                    .with_rotation(orientation_rotation * direction_rotation),
            ));
        },
    );
}

fn update<T: PercentageComponent>(
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
    status_bar_query: Query<
        (&StatusBarMeshMaterial, &StatusBarOwner),
        Without<StatusBarDefinition<T>>,
    >,
    owner_percentage_component_query: Query<&T>,
) {
    status_bar_query
        .iter()
        .for_each(|(material_handle, &StatusBarOwner(owner_entity))| {
            let material = status_bar_materials
                .get_mut(material_handle)
                .expect("StatusBarMaterial missing");
            let health = owner_percentage_component_query
                .get(owner_entity)
                .expect("No owner found");
            material.percent = health.percentage().value();
        });
}

fn follow_owner<T: PercentageComponent>(
    mut bar_query: Query<(&mut Transform, &StatusBarOwner), Without<StatusBarDefinition<T>>>,
    owner_query: Query<(&Transform, &StatusBarDefinition<T>)>,
) {
    bar_query
        .iter_mut()
        .for_each(|(mut transform, &StatusBarOwner(owner_entity))| {
            let (owner_transform, owner_bar_definition) =
                owner_query.get(owner_entity).expect("No owner found");
            let new_translation = owner_transform.translation + owner_bar_definition.offset;
            transform.translation = new_translation;
        });
}
