use bevy_ecs::{entity::Entity, schedule::SystemSet, system::Query};

use crate::kinematics::KinematicBody;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kinematics {
    Motion,
    Collision,
    Effect,
}

pub fn detect_collisions(_query: Query<(Entity, &KinematicBody)>) {}
