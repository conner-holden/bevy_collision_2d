use bevy::{prelude::*, window::WindowResolution};
use bevy_collision_2d::prelude::*;
use glam::Vec2;

const TILE_SIZE: f32 = 100.;
const PLAYER_SPEED: f32 = 3.;
const PLAYER_POSITION: (i32, i32) = (0, 0);
const WALL_POSITIONS: [(i32, i32); 27] = [
    (-3, 2),
    (-3, 1),
    (-3, 0),
    (-3, -1),
    (-3, -2),
    (-3, -3),
    (-2, -3),
    (-1, -3),
    (0, -3),
    (1, -3),
    (2, -3),
    (3, -3),
    (4, -3),
    (4, -2),
    (4, -1),
    (4, 0),
    (4, 1),
    (4, 2),
    (-3, 3),
    (-2, 3),
    (-1, 3),
    (0, 3),
    (1, 3),
    (2, 3),
    (3, 3),
    (4, 3),
    (2, 0),
];

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1000., 1000.),
            title: "Single Moving AABB Example".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(CollisionPlugin {
        chunk_size: TILE_SIZE,
        debug: true,
    })
    .add_systems(Startup, setup)
    .add_systems(Update, movement.in_set(Kinematics::Motion))
    .run();
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let position = Vec2::new(PLAYER_POSITION.0 as f32, PLAYER_POSITION.1 as f32) * TILE_SIZE;
    commands.spawn((
        Sprite {
            color: Color::srgb(100., 100., 100.),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        Transform::default(),
        Player,
        KinematicBody::aabb(Vec2::splat(TILE_SIZE), position, Vec2::ZERO),
    ));

    for (x, y) in WALL_POSITIONS {
        let position = Vec2::new(TILE_SIZE * x as f32, TILE_SIZE * y as f32);
        commands.spawn((
            Sprite {
                color: Color::srgb(0., 0., 0.),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_xyz(position.x, position.y, -1.),
            KinematicBody::aabb(Vec2::splat(TILE_SIZE), position, Vec2::ZERO),
        ));
    }
}

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicBody, With<Player>>,
) {
    let t = time.delta_secs();
    for mut k in &mut query {
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
            move_delta *= t;
        }

        k.motion = move_delta * TILE_SIZE * PLAYER_SPEED;
    }
}
