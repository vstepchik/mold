// todo: implement Z-order curve

use units::*;
use world::*;

mod units;
mod world;

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
