use bevy::prelude::*;
use std::f32::consts::PI;
use crate::core::components::fly::*;

#[derive(Component)]
struct Grid;

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // https://bevy-cheatbook.github.io/fundamentals/coords.html?highlight=handed#2d-and-3d-scenes-and-cameras

    // X
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: LinearRgba::rgb(0.5, 0.0, 0.0),
            base_color: Color::srgb_u8(255, 0, 0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(PI * 0.5)),
    ));

    // Y
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: LinearRgba::rgb(0.0, 0.5, 0.0),
            base_color: Color::srgb_u8(0, 255, 0),
            ..default()
        })),
    ));

    // Z
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 200.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: LinearRgba::rgb(0.0, 0.0, 0.5),
            base_color: Color::srgb_u8(0, 0, 255),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(PI * 0.5)),
    ));

    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0).normal(Dir3::Y))),
        MeshMaterial3d(materials.add(Color::srgba_u8(255, 255, 255, 64))),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(2.0)))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(0.0, 0.0, 1.5),
            base_color: Color::srgb_u8(10, 10, 200),
            ..default()}
        )),
        Transform::from_xyz(25.0, 25.0, 20.0),
        Flying,
    ));
}
