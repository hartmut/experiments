use bevy::prelude::*;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::RemotePlugin;
use bevy_3dtest::core::plugins::*;
use bevy_panorbit_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_3dtest::core::systems::fly::*;

fn main() {
    let height: f32 = 700.0;
    let resolution = bevy::window::WindowResolution::new(height * (16.0 / 9.0), height);

    App::new()
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin { // create window
            primary_window: Some(Window {
                resolution,
                title: "experiment".to_string(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(startup::StartupPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, flying_spheres)
        .run();
}
