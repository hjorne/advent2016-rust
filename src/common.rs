use derive_more::*;
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone)]
pub enum LR {
    Left,
    Right,
}

#[derive(Debug, Add, Sub, Copy, Clone, Eq, PartialEq, Hash, AddAssign)]
pub struct Coords2D {
    pub x: i64,
    pub y: i64,
}

impl Coords2D {
    pub fn new(x: i64, y: i64) -> Coords2D {
        Coords2D { x, y }
    }
    pub fn rotate(&self, dir: LR) -> Coords2D {
        match dir {
            LR::Left => Coords2D::new(-self.y, self.x),
            LR::Right => Coords2D::new(self.y, -self.x),
        }
    }
}

impl Mul<i64> for Coords2D {
    type Output = Coords2D;

    fn mul(self, rhs: i64) -> Coords2D {
        Coords2D::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i64> for Coords2D {
    type Output = Coords2D;

    fn div(self, rhs: i64) -> Coords2D {
        Coords2D::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<Coords2D> for i64 {
    type Output = Coords2D;

    fn mul(self, rhs: Coords2D) -> Coords2D {
        rhs * self
    }
}
