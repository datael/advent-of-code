use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
}

impl Add<Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Offset> for Offset {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Sub<Offset> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Offset> for Offset {
    fn sub_assign(&mut self, rhs: Offset) {
        *self = *self - rhs;
    }
}

impl Mul<Offset> for Offset {
    type Output = Offset;

    fn mul(self, rhs: Offset) -> Self::Output {
        Offset {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<Offset> for Offset {
    fn mul_assign(&mut self, rhs: Offset) {
        *self = *self * rhs;
    }
}

impl Offset {
    #[must_use]
    pub fn manhattan_distance(&self, other: Offset) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}
