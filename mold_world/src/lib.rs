// todo: implement Z-order curve

const TILE_SIDE: u32 = 32;

#[derive(Copy, Clone)]
struct Temperature(u16);
#[derive(Copy, Clone)]
struct Elevation(u8);
#[derive(Copy, Clone)]
struct TileCoordinate(u64);
#[derive(Copy, Clone)]
enum GroundType {
    Dirt,
    Grass,
    Sand,
    Water,
}

#[derive(Copy, Clone)]
struct Cell {
    temperature: Temperature,
    elevation: Elevation,
    ground: GroundType,
}

struct Tile {
    coord: TileCoordinate,
    cells: [Cell; (TILE_SIDE * TILE_SIDE) as usize],
}

impl Tile {
    pub fn new() -> Self {
        Tile { coord: TileCoordinate(0), cells: [Cell {temperature: Temperature(0), elevation: Elevation(0), ground: GroundType::Dirt}; (TILE_SIDE * TILE_SIDE) as usize] }
    }
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

pub fn report_struct_size() {
    let tile = Tile::new();
    let temp: Temperature = Temperature(335);
    let elev: Elevation = Elevation(12);

    use std::mem;
    println!("\
    Sizes in bytes:
    Temperature: {}
    Elevation: {}
    GroundType: {}
    Cell: {}
    Tile: {}
    ",
             mem::size_of::<Temperature>(),
             mem::size_of::<Elevation>(),
             mem::size_of::<GroundType>(),
             mem::size_of::<Cell>(),
             mem::size_of::<Tile>(),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
