use super::tile::{Tile, TileType};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines, screen_height, screen_width};

pub struct Board {
    screen_width: f32,
    screen_height: f32,
    pub tiles: Vec<Tile>,
    grid_size: usize,
    square_size: f32,
}

impl Board {

    pub fn new() -> Self {
        let grid_size = 10;
        let square_size = 50.0;
        let tiles = vec![Tile::new(TileType::Empty); grid_size * grid_size];
        let screen_width = screen_width();
        let screen_height = screen_height();
        Self {
            screen_width,
            screen_height,
            tiles,
            grid_size,
            square_size
        }
    }

    pub fn display(&self) {
        clear_background(WHITE);

        let grid_width = self.grid_size as f32 * self.square_size;
        let grid_height = self.grid_size as f32 * self.square_size;
        let offset_x = (self.screen_width - grid_width) / 2.0;
        let offset_y = (self.screen_height - grid_height) / 2.0;

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
        let offset_x = (self.screen_width - grid_width) / 2.0;
        let offset_y = (self.screen_height - grid_height) / 2.0;

        if x < offset_x || x > offset_x + grid_width || y < offset_y || y > offset_y + grid_height {
            return None;
        }

        let col = ((x - offset_x) / self.square_size) as usize;
        let row = ((y - offset_y) / self.square_size) as usize;

        self.tiles.get(row * self.grid_size + col)
    }

/*    pub fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::TileClicked { x, y } => {
                if let Some(tile) = self.get_tile(x as f32, y as f32) {
                   print!("Handle event: {:?}", tile);
                }
            }
        }
    }*/
}
