use crate::core::components::fly::*;
use bevy::prelude::*;

pub fn flying_spheres(time: Res<Time>, mut query: Query<&mut Transform, With<Flying>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y =
            ops::sin(time.elapsed_secs())*10.0;
    }
}
