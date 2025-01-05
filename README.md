### Bevy Collision 2D

A simple Bevy library designed for 2D kinematic-/rigid-body collision. Support is only planned for points and AABBs. Uses `glam` vector types like Bevy does.

### Supported Collisions

| Body 1  | Body 2          | Supported |
| ------- | --------------- | --------- |
| `Point` | `Point`         | &#x2611;  |
| `Point` | `Aabb`          | &#x2611;  |
| `Point` | `Aabb` (moving) | &#x2610;  |
| `Aabb`  | `Aabb` (moving) | &#x2610;  |
| `Aabb`  | `Point`         | &#x2610;  |

### Usage

This library is designed to be as simple as possible. Take note that AABB representation is different than other libraries. Below is an example:

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
