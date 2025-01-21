use super::tile::{Tile, TileType};
use crate::{GRID_SIZE, SQUARE_SIZE};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines, screen_height, screen_width};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

pub struct Board {}

impl Board {
    pub fn new() -> Self {
        Self {}
    }

    pub fn display(&self) {
        clear_background(WHITE);

        // Pobranie rozmiaru ekranu
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Wyśrodkowanie siatki na ekranie
        let grid_width = GRID_SIZE as f32 * SQUARE_SIZE;
        let grid_height = GRID_SIZE as f32 * SQUARE_SIZE;
        let offset_x = (screen_width - grid_width) / 2.0;
        let offset_y = (screen_height - grid_height) / 2.0;

        // Rysowanie siatki
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = offset_x + col as f32 * SQUARE_SIZE;
                let y = offset_y + row as f32 * SQUARE_SIZE;

                draw_rectangle_lines(x, y, SQUARE_SIZE, SQUARE_SIZE, 2.0, BLACK);
            }
        }
    }
}
