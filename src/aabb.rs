use core::f32;

pub use bevy_math::bounding::Aabb2d as Aabb;
use bevy_math::bounding::BoundingVolume;
use glam::Vec2;

use crate::{
    collision::{Collides, Collision},
    motion::{Motion, Movable},
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

impl Movable for Aabb {
    fn position(&self) -> Vec2 {
        self.center()
    }
}

impl Collides<Aabb> for Motion<Aabb> {
    fn collision(&self, other: &Aabb) -> Option<Collision> {
        let mut min_collision: Option<Collision> = None;
        let mut min_distance = f32::INFINITY;
        for corner in self.object.corners() {
            let motion = Motion {
                object: corner,
                delta: self.delta,
            };
            if let Some(collision) = motion.collision(other) {
                let distance = collision.position.distance(corner);
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
    use glam::IVec2;

    use super::*;

    #[test]
    fn test_moving_aabb_aabb_collision() {
        let maabb = Motion {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            delta: Vec2::ONE,
        };
        let aabb = Aabb {
            min: Vec2::new(1.5, 0.75),
            max: Vec2::new(2.5, 1.75),
        };
        let actual = maabb.collision(&aabb);
        let expected = Some(Collision {
            position: Vec2::new(1.5, 1.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_moving_aabb_aabb_non_collision() {
        let maabb = Motion {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            delta: Vec2::ONE,
        };
        let aabb = Aabb {
            min: Vec2::new(2.5, 0.75),
            max: Vec2::new(3.5, 1.75),
        };
        let actual = maabb.collision(&aabb);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
