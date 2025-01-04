use bevy_math::bounding::BoundingVolume;
use glam::{IVec2, Vec2};

use crate::{
    aabb::Aabb,
    collision::{Collides, Collision},
};

pub trait Movable {
    fn position(&self) -> Vec2;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// An object's position and displacement
pub struct Motion<O: Movable> {
    /// The object's initial position
    pub object: O,
    /// The object's displacement
    pub delta: Vec2,
}

pub type Point = Vec2;

impl Movable for Point {
    fn position(&self) -> Vec2 {
        *self
    }
}

impl Motion<Point> {
    pub fn new(object: Point, delta: Vec2) -> Self {
        Self { object, delta }
    }

    /// The object's final position
    pub fn pos_f(&self) -> Vec2 {
        self.object.position() + self.delta
    }

    /// Scale the motion by a `scalar`, returning a new `Motion`
    pub fn scale(&self, scalar: f32) -> Self {
        Self::new(self.object, scalar * self.delta)
    }

    /// Update the delta given a final position
    pub fn to_position(&mut self, pos_f: Vec2) -> &mut Self {
        self.delta = pos_f - self.object.position();
        self
    }
}

impl Collides<Motion<Point>> for Motion<Point> {
    fn collision(&self, other: &Motion<Point>) -> Option<Collision> {
        let cross = self.delta.perp_dot(other.delta);
        if cross == 0.0 {
            return None; // Lines are parallel or collinear
        }

        let pos_displacement = other.object.position() - self.object.position();

        let self_ratio = pos_displacement.perp_dot(other.delta) / cross;
        let other_ratio = pos_displacement.perp_dot(self.delta) / cross;

        // Check if the collision point lies on both line segments
        if self_ratio >= 0.0 && self_ratio <= 1.0 && other_ratio >= 0.0 && other_ratio <= 1.0 {
            Some(Collision {
                position: self.scale(self_ratio).pos_f(),
                ..Default::default()
            })
        } else {
            None // No collision within the line segments
        }
    }
}

impl Collides<Aabb> for Motion<Point> {
    fn collision(&self, aabb: &Aabb) -> Option<Collision> {
        // Calculate the inverse of the displacement to avoid repeated divisions
        let inv_displacement = Vec2::new(1.0 / self.delta.x, 1.0 / self.delta.y);

        // Compute the t-values for intersections with the AABB's boundaries
        let t_min = (aabb.min - self.object.position()) * inv_displacement;
        let t_max = (aabb.max - self.object.position()) * inv_displacement;

        // Determine the near and far t-values for each axis
        let t_near = t_min.min(t_max);
        let t_far = t_min.max(t_max);

        // Find the largest t_near and smallest t_far
        let t_entry = t_near.x.max(t_near.y);
        let t_exit = t_far.x.min(t_far.y);

        // If the line misses the AABB or the collision is outside the line segment, return None
        if t_entry > t_exit || t_exit < 0.0 || t_entry > 1.0 {
            return None;
        }

        // Determine the collision side based on the t_entry axis
        let normal = if t_entry == t_near.x {
            Some(IVec2::new(-self.delta.x.signum() as i32, 0))
        } else {
            Some(IVec2::new(0, -self.delta.y.signum() as i32))
        };

        // Compute the collision point
        let position = self.scale(t_entry).pos_f();

        Some(Collision { position, normal })
    }
}

impl Collides<Motion<Aabb>> for Motion<Point> {
    fn collision(&self, maabb: &Motion<Aabb>) -> Option<Collision> {
        let aabb = Aabb::new(
            maabb.object.position() + maabb.delta,
            maabb.object.half_size(),
        );
        println!("{:?}", aabb);
        // Calculate the inverse of the displacement to avoid repeated divisions
        let inv_displacement = Vec2::new(1.0 / self.delta.x, 1.0 / self.delta.y);

        // Compute the t-values for intersections with the AABB's boundaries
        let t_min = (aabb.min - self.object.position()) * inv_displacement;
        let t_max = (aabb.max - self.object.position()) * inv_displacement;

        // Determine the near and far t-values for each axis
        let t_near = t_min.min(t_max);
        let t_far = t_min.max(t_max);

        // Find the largest t_near and smallest t_far
        let t_entry = t_near.x.max(t_near.y);
        let t_exit = t_far.x.min(t_far.y);

        // If the line misses the AABB or the collision is outside the line segment, return None
        if t_entry > t_exit || t_exit < 0.0 || t_entry > 1.0 {
            return None;
        }

        // Determine the collision side based on the t_entry axis
        let normal = if t_entry == t_near.x {
            Some(IVec2::new(-self.delta.x.signum() as i32, 0))
        } else {
            Some(IVec2::new(0, -self.delta.y.signum() as i32))
        };

        // Compute the collision point
        let position = self.scale(t_entry).pos_f();

        Some(Collision { position, normal })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_motion_collision() {
        let motion_1 = Motion::new(Vec2::ZERO, Vec2::ONE);
        let motion_2 = Motion::new(Vec2::new(0.5, 0.), Vec2::new(0., 1.));
        let actual = motion_1.collision(&motion_2);
        let expected = Some(Collision {
            position: Vec2::splat(0.5),
            ..Default::default()
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_motion_non_collision() {
        let motion_1 = Motion::new(Vec2::ZERO, Vec2::ONE);
        let motion_2 = Motion::new(Vec2::new(0.5, 0.), Vec2::ONE);
        let actual = motion_1.collision(&motion_2);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_aabb_collision() {
        let motion = Motion::new(Vec2::ZERO, Vec2::ONE);
        let aabb = Aabb {
            min: Vec2::new(0.5, 0.),
            max: Vec2::new(1.5, 1.),
        };
        let actual = motion.collision(&aabb);
        let expected = Some(Collision {
            position: Vec2::splat(0.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_aabb_non_collision() {
        let motion = Motion::new(Vec2::ZERO, Vec2::ONE);
        let aabb = Aabb {
            min: Vec2::new(0.5, 0.),
            max: Vec2::new(1.5, 0.25),
        };
        let actual = motion.collision(&aabb);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_moving_aabb_collision() {
        let motion = Motion::new(Vec2::ZERO, Vec2::ONE);
        let maabb = Motion {
            object: Aabb {
                min: Vec2::new(0.5, 0.),
                max: Vec2::new(1.5, 1.),
            },
            delta: -Vec2::X,
        };
        let actual = motion.collision(&maabb);
        let expected = Some(Collision {
            position: Vec2::splat(0.25),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }
}
