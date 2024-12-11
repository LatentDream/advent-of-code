use core::panic;

use super::Coord;

pub type Grid<T> = Vec<Vec<T>>;

// Grid utility functions
pub trait GridOps<T> {
    fn get_dimensions(&self) -> (usize, usize);
    fn get_at(&self, coord: &Coord) -> Option<&T>;
    fn get_at_mut(&mut self, coord: &Coord) -> Option<&mut T>;
    fn set_at(&mut self, coord: &Coord, value: T);
    fn create_empty(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone;
}

impl<T> GridOps<T> for Grid<T> {
    fn get_dimensions(&self) -> (usize, usize) {
        if self.is_empty() {
            (0, 0)
        } else {
            (self.len(), self[0].len())
        }
    }

    fn get_at(&self, coord: &Coord) -> Option<&T> {
        if coord.y >= 0 && coord.x >= 0 {
            self.get(coord.y as usize)?.get(coord.x as usize)
        } else {
            None
        }
    }

    fn get_at_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        if coord.y >= 0 && coord.x >= 0 {
            self.get_mut(coord.y as usize)?.get_mut(coord.x as usize)
        } else {
            None
        }
    }

    fn set_at(&mut self, coord: &Coord, value: T) {
        if coord.y >= 0
            && (coord.y as usize) < self.len()
            && coord.x >= 0
            && (coord.x as usize) < self[0].len()
        {
            self[coord.y as usize][coord.x as usize] = value;
        } else {
            panic!("Set At out of Pound");
        }
    }

    fn create_empty(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        vec![vec![default; cols]; rows]
    }
}
