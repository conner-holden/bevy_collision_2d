use bevy_ecs::component::Component;
use bitflags::bitflags;
use glam::Vec2;

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

#[derive(Component, Clone, Default)]
pub struct KinematicBody<Object: Send + Sync> {
    pub object: Object,
    pub position: Vec2,
    pub motion: Vec2,
    pub mask: Flags,
    pub layer: Flags,
}
