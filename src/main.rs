mod debug;
mod yorkshire;
mod movement;
mod camera;

use bevy::prelude::*;

use yorkshire::YorkshirePlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YorkshirePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
