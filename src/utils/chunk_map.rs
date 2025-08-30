// Adapted from https://github.com/Aunmag/shooter-rs

use bevy_utils::HashMap;
use glam::Vec2;

#[rustfmt::skip]
const OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 0,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkId {
    pub x: i32,
    pub y: i32,
}

impl From<Vec2> for ChunkId {
    fn from(position: Vec2) -> Self {
        Self {
            x: position.x.floor() as i32,
            y: position.y.floor() as i32,
        }
    }
}

impl From<ChunkId> for Vec2 {
    fn from(value: ChunkId) -> Self {
        Vec2::new(value.x as f32, value.y as f32)
    }
}

#[derive(Clone)]
pub struct ChunkMap<T: std::fmt::Debug> {
    pub map: HashMap<ChunkId, Vec<T>>,
    pub chunk_size: Vec2,
}

impl<T: std::fmt::Debug> ChunkMap<T> {
    pub fn new(capacity: usize, chunk_size: f32) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            chunk_size: Vec2::splat(chunk_size),
        }
    }

    pub fn insert(&mut self, position: Vec2, value: T) {
        self.map
            .entry(ChunkId::from(position / self.chunk_size))
            .or_insert_with(Vec::new)
            .push(value);
    }

    pub fn pop(&mut self) -> Option<(ChunkId, T)> {
        let mut value = None;
        let mut empty_chunk_id = None;

        for (id, values) in self.map.iter_mut() {
            value = values.pop().map(|v| (*id, v));

            if value.is_none() || values.is_empty() {
                empty_chunk_id = Some(*id);
            }

            if value.is_some() {
                break;
            }
        }

        if let Some(id) = empty_chunk_id {
            self.map.remove(&id);
        }

        value
    }

    pub fn iter_neighbors<F: FnMut(ChunkId, &T)>(&self, center: ChunkId, mut f: F) {
        for offset in OFFSETS {
            let chunk_id = ChunkId {
                x: center.x + offset.0,
                y: center.y + offset.1,
            };
            let Some(chunk) = self.map.get(&chunk_id) else {
                continue;
            };

            for item in chunk.iter() {
                f(chunk_id, item);
            }
        }
    }
}
