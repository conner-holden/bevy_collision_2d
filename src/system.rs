use bevy_app::{App, Plugin, Update};
use bevy_color::Srgba;
use bevy_ecs::{
    entity::Entity,
    schedule::{IntoSystemConfigs, SystemSet},
    system::{In, IntoSystem, Query, Res, Resource},
};
use bevy_gizmos::gizmos::Gizmos;
use bevy_math::Isometry2d;
use bevy_transform::components::Transform;
use glam::Vec2;

use crate::{kinematics::KinematicBody, utils::chunk_map::ChunkMap};

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kinematics {
    Motion,
    Collision,
    Effect,
}

#[derive(Resource)]
pub struct CollisionConfig {
    pub chunk_size: f32,
    pub debug: bool,
}

pub struct CollisionPlugin {
    pub chunk_size: f32,
    pub debug: bool,
}

impl Default for CollisionPlugin {
    fn default() -> Self {
        Self {
            chunk_size: 1.,
            debug: false,
        }
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionConfig {
            chunk_size: self.chunk_size,
            debug: self.debug,
        })
        .add_systems(
            Update,
            detect_collisions
                .pipe(apply_motion)
                .after(Kinematics::Motion)
                .in_set(Kinematics::Collision),
        );

        if self.debug {
            app.add_systems(Update, draw_debug_rects.after(Kinematics::Effect));
        }
    }
}

pub fn detect_collisions(
    query: Query<(Entity, &KinematicBody)>,
    config: Res<CollisionConfig>,
) -> Vec<(Entity, Vec2)> {
    let mut chunks = ChunkMap::new(0, config.chunk_size);

    for (entity, body) in query.iter() {
        chunks.insert(body.position, (entity, body));
    }

    let mut solutions = Vec::new();

    for (id1, values) in chunks.map.iter() {
        for (e1, k1) in values.iter() {
            let mut min_motion_1 = k1.motion;
            if min_motion_1 == Vec2::ZERO {
                continue;
            }
            let mut min_distance_1 = min_motion_1.length();
            chunks.iter_neighbors(*id1, |_id2, (e2, k2)| {
                if e1 == e2 {
                    return;
                }
                if let Some(collision) = k1.collision(k2) {
                    let motion_1 = collision.motion;
                    let distance_1 = motion_1.length();
                    if distance_1 < min_distance_1 {
                        min_distance_1 = distance_1;
                        min_motion_1 = motion_1;
                    }
                }
            });
            solutions.push((*e1, min_motion_1))
        }
    }

    return solutions;
}

pub fn apply_motion(
    In(solutions): In<Vec<(Entity, Vec2)>>,
    mut query: Query<(&mut Transform, &mut KinematicBody)>,
) {
    for (e, m) in solutions {
        let Ok((mut t, mut kb)) = query.get_mut(e) else {
            continue;
        };
        t.translation += m.extend(0.);
        kb.position = t.translation.truncate();
        kb.motion = Vec2::ZERO;
    }
}

pub fn draw_debug_rects(query: Query<&KinematicBody>, mut gizmos: Gizmos) {
    for k in query.iter() {
        if let Some(size) = k.size {
            gizmos.rect_2d(
                Isometry2d::from_xy(k.position.x, k.position.y),
                size,
                Srgba::RED,
            );
        }
    }
}
