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
