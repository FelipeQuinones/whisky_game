use bevy::prelude::*;
use crate::movement::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_move);
    }
}

fn debug_move(
    movement: Query<(&Velocity, &Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.get_pressed().next().is_some() {
        for (velocity, transform) in movement.iter() {
            println!("Velocity: {:?}, Transform: {:?}\n", velocity, transform);
        }
    }
}
