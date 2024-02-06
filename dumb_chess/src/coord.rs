use std::{ops, cmp::max};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn new(row: i32, col: i32) -> Self {
        Coord { row , col }
    }

    pub fn all_coords() -> impl Iterator<Item=Coord> {
        (0..8).flat_map(|row| {
            (0..8).map(move |col| {
                Self::new(row, col)
            })
        })
    }

    // manhattan distance
    pub fn man_dist(&self, other: Coord) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }

    pub fn cheb_dist(&self, other: Coord) -> i32 {
        max(self.row - other.row, self.col - other.col)
    }

    pub fn dist(&self, other: Coord) -> f32 {
        (((self.row - other.row).pow(2) + (self.col - other.col).pow(2)) as f32).sqrt()
    }

}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = Self::new(self.row + rhs.row, self.col + rhs.col);
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Coord> for (i32, i32) {
    fn from(value: Coord) -> Self {
        (value.row, value.col)
    }
}

