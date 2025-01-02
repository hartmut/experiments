use bevy::prelude::*;

#[derive(Component)]
struct Camera;

pub fn setup_camera(mut commands: Commands) {
    let camera = commands.spawn(Camera3d::default());
}
