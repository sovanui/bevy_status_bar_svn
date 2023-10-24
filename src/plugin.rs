use std::marker::PhantomData;
use bevy::prelude::*;
use crate::definition::{Orientation, StatusBarDefinition, Direction};
use crate::material::StatusBarMaterial;
use crate::percentage::AsPercentage;


pub trait PercentageComponent: Component + AsPercentage {}
impl<T: Component + AsPercentage> PercentageComponent for T {}


pub struct StatusBarPlugin<T: PercentageComponent>(PhantomData<T>);


impl <T: PercentageComponent> Default for StatusBarPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl <T: PercentageComponent> Plugin for StatusBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn::<T>)
            .add_systems(Update, update::<T>)
            .add_systems(PostUpdate, follow_owner)
            .add_plugins(MaterialPlugin::<StatusBarMaterial>::default());
    }
}

#[derive(Component)]
pub struct StatusBarOwner(Entity);


#[derive(Bundle)]
pub struct StatusBarBundle {
    material_mesh_bundle: MaterialMeshBundle<StatusBarMaterial>,
    owner: StatusBarOwner
}


fn spawn<T: PercentageComponent>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,

    owner_query: Query<(&StatusBarDefinition, &T, Entity)>,
    camera_query: Query<&Transform, With<Camera3d>>
) {

    let camera = camera_query.single();

    owner_query.for_each(|(status_bar_definition, percentageComponent, entity)| {


        let orientation_rotation = match status_bar_definition.orientation {
            Orientation::FacingCamera => { camera.rotation }
        };

        let direction_rotation = match status_bar_definition.direction {
            Direction::Horizontal => { Quat::from_rotation_z((90.0f32).to_radians()) }
        };

        commands.spawn(StatusBarBundle {
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(
                    Vec2::new(
                        status_bar_definition.size.width(),
                        status_bar_definition.size.height()
                    )))),
                material: status_bar_materials.add(StatusBarMaterial {
                    foreground_color: status_bar_definition.foreground_color,
                    background_color: status_bar_definition.background_color,
                    percent: percentageComponent.percentage().value(),
                }),
                transform: Transform::from_translation(status_bar_definition.offset)
                    .with_rotation(orientation_rotation)
                    .with_rotation(direction_rotation),
                ..default()
            },
            owner: StatusBarOwner(entity),
        });
    });
}

fn update<T: PercentageComponent>(
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
    mut status_bar_query: Query<(&Handle<StatusBarMaterial>, &StatusBarOwner), Without<StatusBarDefinition>>,
    owner_percentage_component_query: Query<&T>
) {
    status_bar_query.for_each(|(material_handle, &StatusBarOwner(owner_entity))| {

        let material = status_bar_materials.get_mut(material_handle).expect("StatusBarMaterial missing");
        let health = owner_percentage_component_query.get(owner_entity).expect("No owner found");
        material.percent = health.percentage().value();

    });
}

fn follow_owner(
    mut bar_query: Query<(&mut Transform, &StatusBarOwner), Without<StatusBarDefinition>>,
    owner_query: Query<(&Transform, &StatusBarDefinition)>
) {

    bar_query.for_each_mut(|(mut transform, &StatusBarOwner(owner_entity))| {
        let (owner_transform, owner_bar_definition) = owner_query.get(owner_entity).expect("No owner found");
        let new_translation = owner_transform.translation + owner_bar_definition.offset;
        transform.translation = new_translation;
    });

}