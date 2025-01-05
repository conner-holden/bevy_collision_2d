use glam::{IVec2, Vec2};

use crate::{
    aabb::Aabb,
    collision::{Collides, Collision},
    kinematics::KinematicBody,
};

pub type Point = Vec2;

impl Collides<KinematicBody<Point>> for KinematicBody<Point> {
    fn collision(&self, other: &KinematicBody<Point>) -> Option<Collision> {
        let cross = self.motion.perp_dot(other.motion);
        if cross == 0.0 {
            return None; // Lines are parallel or collinear
        }

        let pos_displacement = other.position - self.position;

        let self_ratio = pos_displacement.perp_dot(other.motion) / cross;
        let other_ratio = pos_displacement.perp_dot(self.motion) / cross;

        // Check if the collision point lies on both line segments
        if self_ratio >= 0.0 && self_ratio <= 1.0 && other_ratio >= 0.0 && other_ratio <= 1.0 {
            Some(Collision {
                position: self.position + self_ratio * self.motion,
                ..Default::default()
            })
        } else {
            None // No collision within the line segments
        }
    }
}

impl Collides<KinematicBody<Aabb>> for KinematicBody<Point> {
    fn collision(&self, other: &KinematicBody<Aabb>) -> Option<Collision> {
        // Calculate the inverse of the displacement to avoid repeated divisions
        let inv_displacement = Vec2::new(1.0 / self.motion.x, 1.0 / self.motion.y);

        // Compute the t-values for intersections with the AABB's boundaries
        let t_min = (other.object.min + other.position - self.position) * inv_displacement;
        let t_max = (other.object.max + other.position - self.position) * inv_displacement;

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
            Some(IVec2::new(-self.motion.x.signum() as i32, 0))
        } else {
            Some(IVec2::new(0, -self.motion.y.signum() as i32))
        };

        // Compute the collision point
        let position = self.position + t_entry * self.motion;

        Some(Collision { position, normal })
    }
}

#[cfg(test)]
mod tests {
    use crate::kinematics::Flags;

    use super::*;

    #[test]
    fn test_point_point_collision() {
        let position_1 = Vec2::ZERO;
        let point_1 = KinematicBody {
            object: position_1,
            position: position_1,
            motion: Vec2::ONE,
            ..Default::default()
        };
        let position_2 = Vec2::new(0.5, 0.);
        let point_2 = KinematicBody {
            object: position_2,
            position: position_2,
            motion: Vec2::Y,
            ..Default::default()
        };
        let actual = point_1.collision(&point_2);
        let expected = Some(Collision {
            position: Vec2::splat(0.5),
            ..Default::default()
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_point_non_collision() {
        let position_1 = Vec2::ZERO;
        let point_1 = KinematicBody {
            object: position_1,
            position: position_1,
            motion: Vec2::ONE,
            ..Default::default()
        };
        let position_2 = Vec2::new(0.5, 0.);
        let point_2 = KinematicBody {
            object: position_2,
            position: position_2,
            motion: Vec2::ONE,
            ..Default::default()
        };
        let actual = point_1.collision(&point_2);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_aabb_collision() {
        let position_1 = Vec2::ZERO;
        let point = KinematicBody {
            object: position_1,
            position: position_1,
            motion: Vec2::ONE,
            ..Default::default()
        };
        let position_2 = Vec2::new(0.5, 0.);
        let aabb = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::ONE,
            },
            position: position_2,
            motion: Vec2::ZERO,
            mask: Flags::A,
            layer: Flags::A,
        };
        let actual = point.collision(&aabb);
        let expected = Some(Collision {
            position: Vec2::splat(0.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_aabb_non_collision() {
        let position_1 = Vec2::ZERO;
        let point = KinematicBody {
            object: position_1,
            position: position_1,
            motion: Vec2::ONE,
            ..Default::default()
        };
        let position_2 = Vec2::new(0.5, 0.);
        let aabb = KinematicBody {
            object: Aabb {
                min: Vec2::ZERO,
                max: Vec2::new(1., 0.25),
            },
            position: position_2,
            motion: Vec2::ZERO,
            mask: Flags::A,
            layer: Flags::A,
        };
        let actual = point.collision(&aabb);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
