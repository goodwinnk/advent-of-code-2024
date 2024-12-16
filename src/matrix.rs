use std::ops::Add;
use array2d::Array2D;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    pub row: isize,
    pub column: isize,
}

impl Add<(isize, isize)> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: (isize, isize)) -> Coordinate {
        Coordinate {
            row: self.row + other.0,
            column: self.column + other.1,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Array2DErrorExt {
    InvalidCoordinate(Coordinate),
    Base(array2d::Error)
}

pub trait Array2DExt<T> {
    fn get_safe(&self, coordinate: &Coordinate) -> Option<&T>;
    fn set_coord(&mut self, coordinate: &Coordinate, value: T) -> anyhow::Result<(), Array2DErrorExt>;
}

// Implement the trait for Array2D
impl<T> Array2DExt<T> for Array2D<T> {
    #[inline(always)]
    fn get_safe(&self, coordinate: &Coordinate) -> Option<&T> {
        if coordinate.row >= 0 && coordinate.column >= 0 {
            self.get(coordinate.row as usize, coordinate.column as usize)
        } else {
            None
        }
    }

    #[inline(always)]
    fn set_coord(&mut self, coordinate: &Coordinate, value: T) -> anyhow::Result<(), Array2DErrorExt> {
        if coordinate.row >= 0 && coordinate.column >= 0 {
            self.set(coordinate.row as usize, coordinate.column as usize, value)
                .map_err(|e| Array2DErrorExt::Base(e))
        } else {
            Err(Array2DErrorExt::InvalidCoordinate(coordinate.clone()))
        }
    }
}