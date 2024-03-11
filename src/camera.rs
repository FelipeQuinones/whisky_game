use bevy::prelude::*;
use crate::movement::Velocity;

#[derive(Bundle)]
struct Camera{
    velocity: Velocity,
    cam: Camera2dBundle,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        velocity: Velocity { speed: -90.0 },
        cam: Camera2dBundle::default(),
    });
}

