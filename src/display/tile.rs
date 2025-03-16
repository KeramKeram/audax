use crate::common::display::texture::load_texture_sync;
use macroquad::prelude::Texture2D;

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
}

impl Tile {
    pub fn new(tile_type: TileType, texture_path: &str) -> Self {
        Self {
            tile_type: tile_type,
            texture: load_texture_sync(texture_path),
            back_light: false
        }
    }
}
