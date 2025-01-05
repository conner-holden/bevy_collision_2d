use core::f32;

pub use bevy_math::bounding::Aabb2d as Aabb;
use bevy_math::bounding::BoundingVolume;
use glam::Vec2;

use crate::{
    collision::{Collides, Collision},
    kinematics::KinematicBody,
    point::Point,
};

pub trait AabbExt {
    fn corners(&self) -> [Point; 4];
}

impl AabbExt for Aabb {
    fn corners(&self) -> [Point; 4] {
        [
            self.min,
            self.min + self.half_size().y,
            self.max,
            self.min + self.half_size().x,
        ]
    }
}

impl Collides<KinematicBody<Aabb>> for KinematicBody<Aabb> {
    fn collision(&self, other: &KinematicBody<Aabb>) -> Option<Collision> {
        assert_eq!(
            other.motion,
            Vec2::ZERO,
            "Only one moving AABB is currently supported"
        );
        let mut min_collision: Option<Collision> = None;
        let mut min_distance = f32::INFINITY;
        for corner in self.object.corners() {
            let point = KinematicBody {
                object: corner + self.position,
                position: corner + self.position,
                motion: self.motion,
                ..Default::default()
            };
            if let Some(collision) = point.collision(other) {
                let distance = collision.position.distance(corner + self.position);
                if distance < min_distance {
                    min_collision = Some(collision);
                    min_distance = distance;
                }
            }
        }
        return min_collision;
    }
}

#[cfg(test)]
mod tests {
    use glam::{IVec2, Vec2};

    use crate::kinematics::Flags;

    use super::*;

    #[test]
    fn test_moving_aabb_aabb_collision() {
        let aabb_1 = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            position: Vec2::ZERO,
            motion: Vec2::ONE,
            mask: Flags::A,
            layer: Flags::A,
        };
        let aabb_2 = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            position: Vec2::new(1.5, 0.75),
            motion: Vec2::ZERO,
            mask: Flags::A,
            layer: Flags::A,
        };
        let actual = aabb_1.collision(&aabb_2);
        let expected = Some(Collision {
            position: Vec2::new(1.5, 1.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_moving_aabb_aabb_non_collision() {
        let aabb_1 = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            position: Vec2::ZERO,
            motion: Vec2::ONE,
            mask: Flags::A,
            layer: Flags::A,
        };
        let aabb_2 = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            position: Vec2::new(2.5, 0.75),
            motion: Vec2::ZERO,
            mask: Flags::A,
            layer: Flags::A,
        };
        let actual = aabb_1.collision(&aabb_2);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
