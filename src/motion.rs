use glam::Vec2;

use crate::point::Point;

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
