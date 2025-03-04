use crate::position::WorldPosition;
use bevy::input::mouse::MouseMotion;
use bevy::math::f64::DVec3;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct ThisPlayer;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    _p: Player,
    world_position: WorldPosition,
}

fn init_this_player(mut commands: Commands) {
    let camera_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    };

    let player_bundle = PlayerBundle {
        _p: Player,
        world_position: WorldPosition::from_xyz(0.0, 6.0, 12.0),
    };
    commands.spawn((camera_bundle, player_bundle, ThisPlayer));
}

fn camera_mover(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut query: Query<(&ThisPlayer, &mut WorldPosition)>,
    mut window_query: Query<&mut Window>,
) {
    // handle mouse locking
    let mut window = window_query.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
    if keys.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    // handle direction
    let (_, mut worldpos) = query.single_mut();
    let forward = worldpos.forward();
    let left = worldpos.left();

    let mut direction = DVec3::ZERO;

    if keys.pressed(KeyCode::W) {
        direction += forward;
    }
    if keys.pressed(KeyCode::S) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::A) {
        direction += left;
    }
    if keys.pressed(KeyCode::D) {
        direction -= left;
    }
    if keys.pressed(KeyCode::Space) {
        direction += DVec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= DVec3::Y;
    }

    if direction == DVec3::ZERO {
        return;
    }

    let speed = 0.1;
    worldpos.position += speed * direction.normalize();
}

fn camera_rotator(
    mut camera_query: Query<(&ThisPlayer, &mut WorldPosition)>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut window_query: Query<&mut Window>,
) {
    let (_, mut worldpos) = camera_query.single_mut();
    let window = window_query.single_mut();
    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    let mouse_vec: Vec2 = mouse_motion_event_reader.iter().map(|x| x.delta).sum();

    if mouse_vec == Vec2::ZERO {
        return;
    }
    let mouse_sensitivity = 0.005;

    worldpos.add_pitch_clamp(mouse_vec.y * mouse_sensitivity);
    worldpos.add_yaw(-mouse_vec.x * mouse_sensitivity);
}

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_this_player)
            .add_systems(Update, (camera_mover, camera_rotator));
    }
}
