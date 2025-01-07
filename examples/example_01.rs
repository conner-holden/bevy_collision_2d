use bevy::{prelude::*, window::WindowResolution};
use bevy_collision_2d::prelude::*;
use glam::Vec2;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1000., 1000.),
            title: "Example 1".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(CollisionPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, movement.in_set(Kinematics::Motion))
    .run();
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            color: Color::srgb(100., 100., 100.),
            custom_size: Some(Vec2::splat(100.)),
            ..Default::default()
        },
        Transform::default(),
        Player,
        KinematicBody::aabb(Vec2::splat(100.), Vec2::ZERO, Vec2::ZERO),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0., 0., 0.),
            custom_size: Some(Vec2::splat(100.)),
            ..Default::default()
        },
        Transform::from_xyz(200., 0., -1.),
        KinematicBody::aabb(Vec2::splat(100.), Vec2::new(200., 0.), Vec2::ZERO),
    ));
}

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut KinematicBody, With<Player>>,
) {
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
        }

        k.motion = move_delta * 10.;
    }
}
