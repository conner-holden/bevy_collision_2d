### Bevy Collision 2D

A simple Bevy library designed for 2D kinematic-/rigid-body collision. Support is only planned for points and AABBs.

### Supported Collisions

| Body 1                 | Body 2                         | Supported |
| ---------------------- | ------------------------------ | --------- |
| `KinematicBody<Point>` | `KinematicBody<Point>`         | &#x2611;  |
| `KinematicBody<Point>` | `KinematicBody<Aabb>`          | &#x2611;  |
| `KinematicBody<Point>` | `KinematicBody<Aabb>` (moving) | &#x2610;  |
| `KinematicBody<Aabb>`  | `KinematicBody<Aabb>` (moving) | &#x2610;  |
