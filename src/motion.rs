use bevy_math::bounding::Aabb2d as Aabb;
use glam::{IVec2, Vec2};

use crate::intersection::{Intersection, Intersects};

#[derive(Debug, Copy, Clone, PartialEq)]
/// An object's position and displacement
pub struct Motion {
    /// The object's initial position
    pub pos_0: Vec2,
    /// The object's displacement
    pub delta: Vec2,
}

impl Motion {
    pub fn new(pos_0: Vec2, delta: Vec2) -> Self {
        Self { pos_0, delta }
    }

    /// The object's final position
    pub fn pos_f(&self) -> Vec2 {
        self.pos_0 + self.delta
    }

    /// Scale the motion by a `scalar`, returning a new `Motion`
    pub fn scale(&self, scalar: f32) -> Self {
        Self::new(self.pos_0, scalar * self.delta)
    }

    /// Update the delta given a final position
    pub fn to_position(&mut self, pos_f: Vec2) -> &mut Self {
        self.delta = pos_f - self.pos_0;
        self
    }
}

impl Intersects<Motion> for Motion {
    fn intersection(&self, other: &Motion) -> Option<Intersection> {
        let cross = self.delta.perp_dot(other.delta);
        if cross == 0.0 {
            return None; // Lines are parallel or collinear
        }

        let pos_displacement = other.pos_0 - self.pos_0;

        let self_ratio = pos_displacement.perp_dot(other.delta) / cross;
        let other_ratio = pos_displacement.perp_dot(self.delta) / cross;

        // Check if the intersection point lies on both line segments
        if self_ratio >= 0.0 && self_ratio <= 1.0 && other_ratio >= 0.0 && other_ratio <= 1.0 {
            Some(Intersection {
                position: self.scale(self_ratio).pos_f(),
                ..Default::default()
            })
        } else {
            None // No intersection within the line segments
        }
    }
}

impl Intersects<Aabb> for Motion {
    fn intersection(&self, aabb: &Aabb) -> Option<Intersection> {
        // Calculate the inverse of the displacement to avoid repeated divisions
        let inv_displacement = Vec2::new(1.0 / self.delta.x, 1.0 / self.delta.y);

        // Compute the t-values for intersections with the AABB's boundaries
        let t_min = (aabb.min - self.pos_0) * inv_displacement;
        let t_max = (aabb.max - self.pos_0) * inv_displacement;

        // Determine the near and far t-values for each axis
        let t_near = t_min.min(t_max);
        let t_far = t_min.max(t_max);

        // Find the largest t_near and smallest t_far
        let t_entry = t_near.x.max(t_near.y);
        let t_exit = t_far.x.min(t_far.y);

        // If the line misses the AABB or the intersection is outside the line segment, return None
        if t_entry > t_exit || t_exit < 0.0 || t_entry > 1.0 {
            return None;
        }

        // Determine the intersection side based on the t_entry axis
        let normal = if t_entry == t_near.x {
            Some(IVec2::new(-self.delta.x.signum() as i32, 0))
        } else {
            Some(IVec2::new(0, -self.delta.y.signum() as i32))
        };

        // Compute the intersection point
        let position = self.scale(t_entry).pos_f();

        Some(Intersection { position, normal })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_motion_intersection() {
        let motion_1 = Motion::new(Vec2::ZERO, Vec2::ONE);
        let motion_2 = Motion::new(Vec2::new(0.5, 0.), Vec2::new(0., 1.));
        let actual = motion_1.intersection(&motion_2);
        let expected = Some(Intersection {
            position: Vec2::splat(0.5),
            ..Default::default()
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_motion_non_intersection() {
        let motion_1 = Motion::new(Vec2::ZERO, Vec2::ONE);
        let motion_2 = Motion::new(Vec2::new(0.5, 0.), Vec2::ONE);
        let actual = motion_1.intersection(&motion_2);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_aabb_intersection() {
        let motion = Motion::new(Vec2::ZERO, Vec2::ONE);
        let aabb = Aabb {
            min: Vec2::new(0.5, 0.),
            max: Vec2::new(1.5, 1.),
        };
        let actual = motion.intersection(&aabb);
        let expected = Some(Intersection {
            position: Vec2::splat(0.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_motion_aabb_non_intersection() {
        let motion = Motion::new(Vec2::ZERO, Vec2::ONE);
        let aabb = Aabb {
            min: Vec2::new(0.5, 0.),
            max: Vec2::new(1.5, 0.25),
        };
        let actual = motion.intersection(&aabb);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
