use bevy_ecs::component::Component;
use bitflags::bitflags;
use glam::{IVec2, Vec2};

bitflags! {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Flags: u8 {
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const C = 0b0000_0100;
        const D = 0b0000_1000;
        const E = 0b0001_0000;
        const F = 0b0010_0000;
        const G = 0b0100_0000;
        const H = 0b1000_0000;
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// A point of collision between two objects
pub struct Collision {
    /// The resolved motion of the collision
    pub motion: Vec2,
    /// The global position at which the objects collision
    pub position: Vec2,
    /// The optional normal unit vector of the collision
    pub normal: Option<IVec2>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct KinematicBody {
    pub size: Option<Vec2>,
    pub position: Vec2,
    pub motion: Vec2,
    pub mask: Flags,
    pub layer: Flags,
}

impl KinematicBody {
    pub fn point(position: Vec2, motion: Vec2) -> Self {
        Self {
            position,
            motion,
            ..Default::default()
        }
    }

    pub fn aabb(size: Vec2, position: Vec2, motion: Vec2) -> Self {
        Self {
            size: Some(size),
            position,
            motion,
            ..Default::default()
        }
    }

    pub fn collision(&self, other: &Self) -> Option<Collision> {
        let result = match (self.size, other.size) {
            // Point-point collision
            (None, None) => {
                let cross = self.motion.perp_dot(other.motion);
                if cross == 0.0 {
                    return None; // Lines are parallel or collinear
                }

                let pos_displacement = other.position - self.position;

                let self_ratio = pos_displacement.perp_dot(other.motion) / cross;
                let other_ratio = pos_displacement.perp_dot(self.motion) / cross;

                // Check if the collision point lies on both line segments
                if self_ratio >= 0.0
                    && self_ratio <= 1.0
                    && other_ratio >= 0.0
                    && other_ratio <= 1.0
                {
                    let motion = self_ratio * self.motion;
                    Some(Collision {
                        motion,
                        position: self.position + motion,
                        ..Default::default()
                    })
                } else {
                    None // No collision within the line segments
                }
            }
            // Point-AABB collision
            (None, Some(other_size)) => {
                // Calculate the inverse of the displacement to avoid repeated divisions
                let inv_displacement = Vec2::new(
                    if self.motion.x != 0.0 {
                        1.0 / self.motion.x
                    } else {
                        f32::INFINITY
                    },
                    if self.motion.y != 0.0 {
                        1.0 / self.motion.y
                    } else {
                        f32::INFINITY
                    },
                );

                // Compute the t-values for intersections with the AABB's boundaries
                let mut t_min =
                    (other.position - 0.5 * other_size - self.position) * inv_displacement;
                let mut t_max =
                    (other.position + 0.5 * other_size - self.position) * inv_displacement;

                if self.motion.y == 0.0 {
                    if self.position.y >= other.position.y
                        && self.position.y <= other.position.y + other_size.y
                    {
                        t_min.y = f32::NEG_INFINITY; // Inside the bounds
                        t_max.y = f32::INFINITY; // No real exit since not moving in y
                    } else {
                        t_min.y = f32::INFINITY; // Never enters
                        t_max.y = f32::INFINITY; // Consistently outside
                    }
                }

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
                let motion = t_entry * self.motion;
                let position = self.position + motion;
                // println!("{:?}", position);

                Some(Collision {
                    motion,
                    position,
                    normal,
                })
            }
            // AABB-AABB collision
            (Some(self_size), Some(_other_size)) => {
                if other.motion != Vec2::ZERO {
                    return None;
                }
                let mut min_collision: Option<Collision> = None;
                let mut min_distance = f32::INFINITY;
                let corners = [
                    self.position - 0.5 * self_size,
                    self.position + 0.5 * Vec2::new(-self_size.x, self_size.y),
                    self.position + 0.5 * self_size,
                    self.position + 0.5 * Vec2::new(self_size.x, -self_size.y),
                ];
                for corner in corners {
                    let point = KinematicBody::point(corner, self.motion);
                    if let Some(collision) = point.collision(other) {
                        let distance = collision.position.distance(corner);
                        if distance < min_distance {
                            min_collision = Some(collision);
                            min_distance = distance;
                        }
                    }
                }
                return min_collision;
            }
            // TODO: AABB-point collision
            _ => None,
        };
        return result;
    }
}

#[cfg(test)]
mod tests {
    use glam::{IVec2, Vec2};

    use super::*;

    #[test]
    fn test_moving_aabb_aabb_collision() {
        let aabb_1 = KinematicBody::aabb(Vec2::ONE, Vec2::ZERO, Vec2::ONE);
        let aabb_2 = KinematicBody::aabb(Vec2::ONE, Vec2::new(1.5, 0.75), Vec2::ZERO);
        let actual = aabb_1.collision(&aabb_2);
        let expected = Some(Collision {
            motion: Vec2::splat(0.5),
            position: Vec2::splat(1.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_moving_aabb_aabb_perfect_collision() {
        let aabb_1 = KinematicBody::aabb(Vec2::ONE, Vec2::ZERO, Vec2::X);
        let aabb_2 = KinematicBody::aabb(Vec2::ONE, Vec2::new(1.5, 0.), Vec2::ZERO);
        let actual = aabb_1.collision(&aabb_2);
        let expected = Some(Collision {
            motion: Vec2::new(0.5, 0.),
            position: Vec2::new(1.5, 0.),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_moving_aabb_aabb_non_collision() {
        let aabb_1 = KinematicBody::aabb(Vec2::ONE, Vec2::ZERO, Vec2::ONE);
        let aabb_2 = KinematicBody::aabb(Vec2::ONE, Vec2::new(2.5, 0.75), Vec2::ZERO);
        let actual = aabb_1.collision(&aabb_2);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_point_collision() {
        let point_1 = KinematicBody::point(Vec2::ZERO, Vec2::ONE);
        let point_2 = KinematicBody::point(Vec2::new(0.5, 0.), Vec2::Y);
        let actual = point_1.collision(&point_2);
        let expected = Some(Collision {
            motion: Vec2::splat(0.5),
            position: Vec2::splat(0.5),
            ..Default::default()
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_point_non_collision() {
        let point_1 = KinematicBody::point(Vec2::ZERO, Vec2::ONE);
        let point_2 = KinematicBody::point(Vec2::new(0.5, 0.), Vec2::ONE);
        let actual = point_1.collision(&point_2);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_aabb_collision() {
        let point = KinematicBody::point(Vec2::ZERO, Vec2::ONE);
        let aabb = KinematicBody::aabb(Vec2::ONE, Vec2::new(0.5, 0.), Vec2::ZERO);
        let actual = point.collision(&aabb);
        let expected = Some(Collision {
            motion: Vec2::splat(0.5),
            position: Vec2::splat(0.5),
            normal: Some(-IVec2::X),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_aabb_non_collision() {
        let point = KinematicBody::point(Vec2::ZERO, Vec2::ONE);
        let aabb = KinematicBody::aabb(Vec2::new(1., 0.25), Vec2::new(0.5, 0.), Vec2::ZERO);
        let actual = point.collision(&aabb);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
