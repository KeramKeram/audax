use crate::common::display::texture::load_texture_sync;
use macroquad::prelude::Texture2D;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Obstacle,
    SpawnPoint,
    MyUnit,
    EnemyUnit,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    texture: Texture2D,
    pub back_light: bool,
    pub unit: Option<Unit>
}

impl Tile {
    pub fn new(tile_type: TileType, texture_path: &str) -> Self {
        Self {
            tile_type: tile_type,
            texture: load_texture_sync(texture_path),
            back_light: false,
            unit: None,
        }
    }

    pub fn set_unit(&mut self, unit: Unit, tile_type: TileType) {
        self.unit = Some(unit);
        self.tile_type = tile_type;
    }

    pub fn get_unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }
}


