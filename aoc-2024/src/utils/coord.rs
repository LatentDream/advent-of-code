use std::{
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: TryInto<i32>,
        T::Error: std::fmt::Debug,
    {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }

    /// Returns a vector of adjacent coordinates in the order: up, right, down, left
    pub fn adjacents(&self) -> Vec<Coord> {
        vec![
            Coord { x: self.x, y: self.y - 1 }, // up
            Coord { x: self.x + 1, y: self.y }, // right
            Coord { x: self.x, y: self.y + 1 }, // down
            Coord { x: self.x - 1, y: self.y }, // left
        ]
    }

}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}
impl Eq for Coord {}
impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn add(self, other: &'b Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Coord> for &'a Coord {
    type Output = Coord;

    fn sub(self, other: &'b Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
