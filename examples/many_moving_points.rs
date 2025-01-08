use bevy::{prelude::*, window::WindowResolution};
use bevy_collision_2d::prelude::*;
use glam::Vec2;
use rand::Rng;

const TILE_SIZE: f32 = 100.;
const PROJECTILE_SPEED: f32 = 3.;
const PROJECTILE_SPAWN_POSITION: (i32, i32) = (0, 0);
const WALL_POSITIONS: [(i32, i32); 26] = [
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
];

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1000., 1000.),
            title: "Many Moving Points Example".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(CollisionPlugin {
        chunk_size: TILE_SIZE,
    })
    .add_systems(Startup, setup)
    .add_systems(Update, spawn_projectile)
    .add_systems(Update, movement.in_set(Kinematics::Motion))
    .run();
}

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    for (x, y) in WALL_POSITIONS {
        let size = Vec2::splat(TILE_SIZE);
        let position = Vec2::new(x as f32, y as f32) * size;
        commands.spawn((
            Sprite {
                color: Color::srgb(0., 0., 0.),
                custom_size: Some(size),
                ..Default::default()
            },
            Transform::from_xyz(position.x, position.y, -1.),
            KinematicBody::aabb(
                size,
                position - Vec2::splat(TILE_SIZE / 2.) + Vec2::splat(10.),
                Vec2::ZERO,
            ),
        ));
    }
}

pub fn spawn_projectile(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let direction = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();
    let position = Vec2::new(
        PROJECTILE_SPAWN_POSITION.0 as f32,
        PROJECTILE_SPAWN_POSITION.1 as f32,
    ) * TILE_SIZE;
    let size = Vec2::splat(TILE_SIZE / 5.);
    commands.spawn((
        Sprite {
            color: Color::srgb(100., 100., 100.),
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_xyz(position.x, position.y, 0.),
        Projectile { direction },
        KinematicBody::aabb(size, position + direction * size * 2., Vec2::ZERO),
    ));
}

pub fn movement(time: Res<Time>, mut query: Query<(&mut KinematicBody, &Projectile)>) {
    let t = time.delta_secs();
    for (mut k, p) in &mut query {
        k.motion = p.direction * TILE_SIZE * PROJECTILE_SPEED * t;
    }
}
