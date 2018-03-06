
#[derive(Copy, Clone)]
pub struct TileCoordinate(u64);

impl TileCoordinate {
    pub fn new(x: u32, y: u32) -> Self {
        TileCoordinate(Self::encode(x, y))
    }

    fn encode(x: u32, y: u32) -> u64 {
        x as u64 | ((y as u64) << 32)
    }
}
