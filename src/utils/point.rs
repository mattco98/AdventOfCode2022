extern crate derive_more;

use std::convert::From;
use derive_more::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::utils::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn moved(&self, dir: Direction, n: isize) -> Self {
        Point {
            x: self.x + dir.dx() * n,
            y: self.y + dir.dy() * n,
        }
    }

    pub fn unit(&self) -> Self {
        let mut unit = *self;

        if unit.x < 0 {
            unit.x = -1;
        } else if unit.x > 0 {
            unit.x = 1;
        }

        if unit.y < 0 {
            unit.y = -1;
        } else if unit.y > 0 {
            unit.y = 1;
        }

        unit
    }
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}
