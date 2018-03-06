use world::{Cell, TileCoordinate};

const TILE_SIDE: u32 = 32;
const TILE_CELL_COUNT: usize = (TILE_SIDE * TILE_SIDE) as usize;

pub struct Tile {
    coord: TileCoordinate,
    cells: [Cell; TILE_CELL_COUNT],
}

impl Tile {
    pub fn new() -> Self {
        Tile { coord: TileCoordinate::new(1, 2), cells: [Cell::new(); TILE_CELL_COUNT] }
    }
}
