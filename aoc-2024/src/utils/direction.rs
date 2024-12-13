use std::hash::{Hash, Hasher};

use super::Coord;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    pub fn delta(&self) -> Coord {
        match self {
            Direction::UP => Coord::new(0, -1),
            Direction::DOWN => Coord::new(0, 1),
            Direction::RIGHT => Coord::new(1, 0),
            Direction::LEFT => Coord::new(-1, 0),
        }
    }

    // Get direction from one coordinate to another
    pub fn from_coords(from: &Coord, to: &Coord) -> Option<Direction> {
        let delta = to.clone() - from.clone();

        match (delta.x, delta.y) {
            (0, -1) => Some(Direction::UP),
            (0, 1) => Some(Direction::DOWN),
            (1, 0) => Some(Direction::RIGHT),
            (-1, 0) => Some(Direction::LEFT),
            _ => None, // Return None for diagonal or non-adjacent coordinates
        }
    }
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
