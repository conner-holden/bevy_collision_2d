### Bevy Collision 2D

A simple Bevy library designed for 2D kinematic-/rigid-body collision. Support is only planned for points and AABBs.

### Supported Collisions

| Collider 1      | Collider 2      | Supported |
| --------------- | --------------- | --------- |
| `Motion<Point>` | `Motion<Point>` | &#x2611;  |
| `Motion<Point>` | `Aabb`          | &#x2611;  |
| `Motion<Point>` | `Motion<Aabb>`  | &#x2610;  |
| `Motion<Aabb>`  | `Motion<Aabb>`  | &#x2610;  |
