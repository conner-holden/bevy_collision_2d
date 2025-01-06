use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    entity::Entity,
    schedule::{IntoSystemConfigs, SystemSet},
    system::{In, IntoSystem, Query},
};
use bevy_transform::components::Transform;
use glam::Vec2;

use crate::{kinematics::KinematicBody, utils::chunk_map::ChunkMap};

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kinematics {
    Motion,
    Collision,
    Effect,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_collisions
                .pipe(apply_motion)
                .after(Kinematics::Motion)
                .in_set(Kinematics::Collision),
        );
    }
}

pub fn detect_collisions(query: Query<(Entity, &KinematicBody)>) -> Vec<(Entity, Vec2)> {
    let mut chunks = ChunkMap::new(0);

    for (entity, body) in query.iter() {
        chunks.insert(body.position, (entity, body));
    }

    let mut solutions = Vec::new();

    while let Some((id1, (e1, k1))) = chunks.pop() {
        let mut min_motion_1 = k1.motion;
        if min_motion_1 == Vec2::ZERO {
            continue;
        }
        let mut min_distance_1 = min_motion_1.length();
        chunks.iter_neighbors(id1, |_id2, (_e2, k2)| {
            if let Some(collision) = k1.collision(k2) {
                let motion_1 = collision.position - k1.position;
                let distance_1 = motion_1.length();
                if distance_1 < min_distance_1 {
                    min_distance_1 = distance_1;
                    min_motion_1 = motion_1;
                }
            }
        });
        solutions.push((e1, min_motion_1))
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
