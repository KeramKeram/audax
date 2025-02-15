use super::tile::{Tile, TileType};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines};
use crate::common::display::WindowSize;

pub struct Board {
    window_size: WindowSize,
    pub tiles: Vec<Tile>,
    grid_size: usize,
    square_size: f32,
}

impl Board {
    pub fn new(width: f32, height: f32) -> Self {
        let grid_size = 10;
        let square_size = 50.0;
        let window_size = WindowSize {
            screen_width: width,
            screen_height: height,
        };
        let tiles = vec![Tile::new(TileType::Empty); grid_size * grid_size];
        Self {
            window_size,
            tiles,
            grid_size,
            square_size,
        }
    }

    pub fn display(&self) {
        clear_background(WHITE);

        let grid_width = self.grid_size as f32 * self.square_size;
        let grid_height = self.grid_size as f32 * self.square_size;
        let offset_x = (self.window_size.screen_width - grid_width) / 2.0;
        let offset_y = (self.window_size.screen_height - grid_height) / 2.0;

        for row in 0..self.grid_size {
            for col in 0..self.grid_size {
                let x = offset_x + col as f32 * self.square_size;
                let y = offset_y + row as f32 * self.square_size;

                draw_rectangle_lines(x, y, self.square_size, self.square_size, 2.0, BLACK);
            }
        }
    }

    pub fn get_tile(&self, x: f32, y: f32) -> Option<&Tile> {
        let grid_width = self.grid_size as f32 * self.square_size;
        let grid_height = self.grid_size as f32 * self.square_size;
        let offset_x = (self.window_size.screen_width - grid_width) / 2.0;
        let offset_y = (self.window_size.screen_height - grid_height) / 2.0;

        if x < offset_x || x > offset_x + grid_width || y < offset_y || y > offset_y + grid_height {
            return None;
        }

        let col = ((x - offset_x) / self.square_size) as usize;
        let row = ((y - offset_y) / self.square_size) as usize;

        self.tiles.get(row * self.grid_size + col)
    }
}
