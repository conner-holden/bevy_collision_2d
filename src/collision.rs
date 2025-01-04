use glam::{IVec2, Vec2};

pub trait Collides<T>
where
    Self: Sized,
{
    fn collision(&self, other: &T) -> Option<Collision>;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// A point of collision between two objects
pub struct Collision {
    /// The global position at which the objects collision
    pub position: Vec2,
    /// The optional normal unit vector of the collision
    pub normal: Option<IVec2>,
}
