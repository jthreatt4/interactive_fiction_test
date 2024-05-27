use bevy::prelude::*;

use super::MainCamera;

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, camera));
}
