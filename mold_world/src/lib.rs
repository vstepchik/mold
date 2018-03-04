struct Temperature(u16);
struct Elevation(u8);

enum GroundType {
    Dirt,
    Grass,
    Water,
}

struct Tile {
    temperature: Temperature,
    elevation: Elevation,
    ground: GroundType,
}

use std::ops::{Add, Sub};
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

pub fn report_struct_size() {
    let temp: Temperature = Temperature(335);
    let elev: Elevation = Elevation(12);
//    let sum = temp + elev;
//    println!("sum = {}", sum);

    use std::mem;
    println!("\
    Sizes in bytes:
    Temperature: {}
    Elevation: {}
    GroundType: {}
    Tile: {}
    ",
             mem::size_of::<Temperature>(),
             mem::size_of::<Elevation>(),
             mem::size_of::<GroundType>(),
             mem::size_of::<Tile>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
