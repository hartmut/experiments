use crate::core::systems::startup::*;
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera);
        app.add_systems(Startup, grid::setup_grid);
    }
}
