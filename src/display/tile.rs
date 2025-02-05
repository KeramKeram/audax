#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Obstacle,
    SpawnPoint,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub occupied: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            occupied: false,
        }
    }
}
