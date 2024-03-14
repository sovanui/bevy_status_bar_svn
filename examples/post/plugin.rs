use bevy::prelude::*;
use crate::percentage::AsPercentage;
use crate::definition::StatusBarDefinition;
use crate::material::StatusBarMaterial;

pub trait PercentageComponent: Component + AsPercentage {}
impl<T: Component + AsPercentage> PercentageComponent for T {}

pub struct StatusBarPlugin<T: PercentageComponent>;

impl<T: PercentageComponent> Plugin for StatusBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StatusBarMaterial>::default())
            .add_systems(PostStartup, spawn)
            .add_systems(PostUpdate, follow_owner);
    }
}


#[derive(Component)]
pub struct StatusBarOwner(Entity);

#[derive(Bundle)]
pub struct StatusBarBundle {
    material_mesh_bundle: MaterialMeshBundle<StatusBarMaterial>,
    owner: StatusBarOwner,
}


fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,

    stat_bar_query: Query<(&StatusBarDefinition, &Transform, Entity)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {

    let camera = camera_query.single();

    stat_bar_query.iter().for_each(| (status_bar_definition, transform, entity)| {

        commands.spawn(StatusBarBundle {
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(Rectangle::new(
                    status_bar_definition.size.width(),
                    status_bar_definition.size.height(),
                )),
                material: status_bar_materials.add(StatusBarMaterial {
                    foreground_color: status_bar_definition.foreground_color,
                    background_color: status_bar_definition.background_color,
                    percent: 1.,
                }),
                transform: Transform::from_translation(transform.translation + status_bar_definition.offset)
                    .with_rotation(camera.rotation),
                ..default()
            },
            owner: StatusBarOwner(entity)
        });

    });
}


fn follow_owner(
    mut bar_query: Query<(&mut Transform, &StatusBarOwner), Without<StatusBarDefinition>>,
    owner_query: Query<(&Transform, &StatusBarDefinition)>,
) {
    bar_query.iter_mut().for_each(|(mut transform, &StatusBarOwner(owner_entity))| {

        let (owner_transform, owner_bar_definition) = owner_query
            .get(owner_entity)
            .expect("No owner found");

        transform.translation = owner_transform.translation + owner_bar_definition.offset;
    });
}
