### Bevy Collision 2D

A simple Bevy library designed for 2D kinematic-body collision. Support is only planned for points and AABBs. Uses `glam` vector types like Bevy does.

### Supported Functionality

See the [pinned issue](https://github.com/cloud303-cholden/bevy-collision-2d/issues/1) for current progress in planned functionality.

### Usage

This library is designed to be as simple as possible. Take note that AABB representation is different than other libraries. Below is an example of how collision is detected:

```rs
use bevy_collision_2d::prelude::*;
use glam::{IVec2, Vec2};

let point = KinematicBody::point(
    Vec2::ZERO, // Position
    Vec2::ONE,  // Motion
);
let aabb = KinematicBody::aabb(
    Vec2::ONE,          // Size
    Vec2::new(0.5, 0.), // Position
    Vec2::ZERO,         // Motion
);
let actual = point.collision(&aabb);
let expected = Some(Collision {
    position: Vec2::splat(0.5),
    normal: Some(-IVec2::X), // Relative to the body passed to `collision()`
});
assert_eq!(actual, expected);
```
