use glam::{IVec2, Vec2};

pub trait Intersects<T>
where
    Self: Sized,
{
    fn intersection(&self, other: &T) -> Option<Intersection>;
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// A point of intersection between two objects
pub struct Intersection {
    /// The global position at which the objects intersection
    pub position: Vec2,
    /// The optional normal unit vector of the intersection
    pub normal: Option<IVec2>,
}
