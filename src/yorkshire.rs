use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Yorkshire;

#[derive(Bundle)]
struct YorkshireBundle {
    name: Yorkshire,
    velocity: Velocity,
    transform: Transform,
    global_transform: GlobalTransform,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}

pub struct YorkshirePlugin;

impl Plugin for YorkshirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_yorkshire)
            .add_systems(Update, move_yorkshire);
    }
}

fn spawn_yorkshire(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(YorkshireBundle {
        name: Yorkshire,
        velocity: Velocity { speed: 400.0 },
        transform: Transform::from_translation(STARTING_POSITION),
        global_transform: GlobalTransform::default(),
        texture: asset_server.load("bevy_bird_dark.png"),
        sprite: Sprite::default(),
        visibility: Visibility::default(),
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
    });
}

// Modify the move_yorkshire system to move the Yorkshire entity based on the actions
fn move_yorkshire(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Velocity, &mut Transform, &Yorkshire)>
) {
    for (velocity, mut transform, _yorkshire) in query.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += time.delta_seconds() * velocity.speed;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= time.delta_seconds() * velocity.speed;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += time.delta_seconds() * velocity.speed;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= time.delta_seconds() * velocity.speed;
        }
    }
}
