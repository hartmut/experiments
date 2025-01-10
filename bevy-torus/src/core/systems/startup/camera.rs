use bevy::prelude::*;
use bevy_panorbit_camera::*;

#[derive(Component)]
struct Camera;

pub fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // X
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 1.0, 0.1))),
        Transform::from_xyz(25.0, 0.0, 0.0),
    ));
    // Y
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
    ));
    // Z
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 255))),
        Transform::from_xyz(0.0, 0.0, 25.0),
    ));
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
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

    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(80.0, 10.0, 80.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));
    commands.spawn((
        Transform::from_translation(Vec3::new(80.0, 10.0, 80.0)),
        PanOrbitCamera::default(),
    ));
}
