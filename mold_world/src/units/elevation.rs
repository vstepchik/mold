use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Elevation(pub u8);

impl Add for Elevation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Elevation(self.0 + rhs.0)
    }
}

impl Sub for Elevation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Elevation(self.0 - rhs.0)
    }
}
