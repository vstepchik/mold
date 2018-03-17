use world::Cell;
use world::cgmath::Point2;

const TILE_SIDE: u32 = 32;
const TILE_CELL_COUNT: usize = (TILE_SIDE * TILE_SIDE) as usize;

pub struct Tile {
    coord: Point2<u16>,
    cells: [Cell; TILE_CELL_COUNT],
}

impl Tile {
    #[inline]
    pub fn new() -> Self {
        Tile { coord: Point2::new(0, 0), cells: [Cell::new(); TILE_CELL_COUNT] }
    }
}
