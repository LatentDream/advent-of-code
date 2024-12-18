use std::fmt::{Debug, Display};

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
    fn iter_coords(&self) -> GridCoordIterator;
    fn print(&self)
    where
        T: Display;
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
            panic!("Set At out of Bound");
        }
    }

    fn create_empty(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        vec![vec![default; cols]; rows]
    }

    fn iter_coords(&self) -> GridCoordIterator {
        let (rows, cols) = self.get_dimensions();
        GridCoordIterator {
            current_pos: Coord { x: 0, y: 0 },
            dimensions: (rows, cols),
        }
    }

    fn print(&self)
    where
        T: Display,
    {
        for row in self.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub struct GridCoordIterator {
    current_pos: Coord,
    dimensions: (usize, usize),
}

impl Iterator for GridCoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've reached the end of the grid, return None
        if self.current_pos.y as usize >= self.dimensions.0 {
            return None;
        }

        // Store the current position to return
        let current = self.current_pos.clone();

        // Move to the next position
        self.current_pos.x += 1;
        if self.current_pos.x as usize >= self.dimensions.1 {
            self.current_pos.x = 0;
            self.current_pos.y += 1;
        }

        Some(current)
    }
}
