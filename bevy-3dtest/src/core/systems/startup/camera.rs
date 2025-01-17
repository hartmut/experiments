use bevy::prelude::*;
use bevy_panorbit_camera::*;

#[derive(Component)]
struct Camera;

pub fn setup_camera(mut commands: Commands) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 4000.0,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 100.0),
    ));

    // camera
    commands.spawn((
        Transform::from_translation(Vec3::new(40.0, 40.0, 80.0)),
        PanOrbitCamera::default(),
    ));
}
