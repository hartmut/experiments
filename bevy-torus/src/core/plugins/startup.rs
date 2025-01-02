use bevy::prelude::*;
use crate::core::systems::startup::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera);
    }
}