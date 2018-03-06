use units::{Elevation, Temperature};
use world::GroundType;

#[derive(Copy, Clone)]
pub struct Cell {
    temperature: Temperature,
    elevation: Elevation,
    ground: GroundType,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            temperature: Temperature(0),
            elevation: Elevation(0),
            ground: GroundType::Dirt,
        }
    }
}
