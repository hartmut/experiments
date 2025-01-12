use bevy::prelude::*;
use bevy_panorbit_camera::*;
use std::f32::consts::PI;

#[derive(Component)]
struct Camera;

pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // https://bevy-cheatbook.github.io/fundamentals/coords.html?highlight=handed#2d-and-3d-scenes-and-cameras
    // X
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(PI * 0.5)),
    ));
    // TODO let it glow
    // Y
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
    ));
    // Z
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(PI * 0.5)),
    ));
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0).normal(Dir3::Y))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 64))),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 4000.0,
            ..default()
        },
        Transform::from_xyz(40.0, 40.0, 100.0),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(40.0, 40.0, 80.0)),
        PanOrbitCamera::default(),
    ));
}
