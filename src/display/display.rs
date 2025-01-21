use super::tile::{Tile, TileType};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines, screen_height, screen_width};

pub struct Board {
    screen_width: f32,
    screen_height: f32,
    pub tiles: Vec<Tile>,
}

impl Board {
    const GRID_SIZE: usize = 10;
    const SQUARE_SIZE: f32 = 50.0;
    pub fn new() -> Self {
        let tiles = vec![Tile::new(TileType::Empty); Self::GRID_SIZE * Self::GRID_SIZE];
        let screen_width = screen_width();
        let screen_height = screen_height();
        Self {
            screen_width,
            screen_height,
            tiles,
        }
    }

    pub fn display(&self) {
        clear_background(WHITE);

        let grid_width = Board::GRID_SIZE as f32 * Board::SQUARE_SIZE;
        let grid_height = Board::GRID_SIZE as f32 * Board::SQUARE_SIZE;
        let offset_x = (self.screen_width - grid_width) / 2.0;
        let offset_y = (self.screen_height - grid_height) / 2.0;

        for row in 0..Board::GRID_SIZE {
            for col in 0..Board::GRID_SIZE {
                let x = offset_x + col as f32 * Board::SQUARE_SIZE;
                let y = offset_y + row as f32 * Board::SQUARE_SIZE;

                draw_rectangle_lines(x, y, Board::SQUARE_SIZE, Board::SQUARE_SIZE, 2.0, BLACK);
            }
        }
    }

    pub fn get_tile(&self, x: f32, y: f32) -> Option<&Tile> {
        let grid_width = Board::GRID_SIZE as f32 * Board::SQUARE_SIZE;
        let grid_height = Board::GRID_SIZE as f32 * Board::SQUARE_SIZE;
        let offset_x = (self.screen_width - grid_width) / 2.0;
        let offset_y = (self.screen_height - grid_height) / 2.0;

        if x < offset_x || x > offset_x + grid_width || y < offset_y || y > offset_y + grid_height {
            return None;
        }

        let col = ((x - offset_x) / Board::SQUARE_SIZE) as usize;
        let row = ((y - offset_y) / Board::SQUARE_SIZE) as usize;

        self.tiles.get(row * Board::GRID_SIZE + col)
    }
}
