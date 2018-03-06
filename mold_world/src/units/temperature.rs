use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Temperature(pub u16);

impl Add for Temperature {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Temperature(self.0 + rhs.0)
    }
}

impl Sub for Temperature {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Temperature(self.0 - rhs.0)
    }
}
