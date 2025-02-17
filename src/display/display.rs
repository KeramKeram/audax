use std::alloc::System;
use super::tile::{Tile, TileType};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines, screen_height, screen_width, load_texture, vec2, Texture2D, Image};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
use std::fs::File;
use std::io::Read;

struct BattleIcons {
    attack: Texture2D,
    defend: Texture2D,
    magic: Texture2D,
    wait: Texture2D,
    run: Texture2D,
    negotiate: Texture2D,
    system: Texture2D,
}

pub struct Board {
    screen_width: f32,
    screen_height: f32,
    pub tiles: Vec<Tile>,
    battle_icons: BattleIcons,
}

fn load_texture_sync(path: &str) -> Texture2D {
    let mut file = File::open(path).expect("Can't open file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Read error.");

    let image = Image::from_file_with_format(&buffer, None).unwrap();
    Texture2D::from_image(&image)
}

impl Board {
    const GRID_SIZE: usize = 10;
    const SQUARE_SIZE: f32 = 50.0;
    pub fn new() -> Self {
        let tiles = vec![Tile::new(TileType::Empty); Self::GRID_SIZE * Self::GRID_SIZE];
        let screen_width = screen_width();
        let screen_height = screen_height();
        let battle_icons = BattleIcons {
            attack: load_texture_sync("data/graphics/ui/battle/attack.png"),
            defend: load_texture_sync("data/graphics/ui/battle/defence.png"),
            magic: load_texture_sync("data/graphics/ui/battle/magic.png"),
            wait: load_texture_sync("data/graphics/ui/battle/wait.png"),
            run: load_texture_sync("data/graphics/general/hands.png"),
            negotiate: load_texture_sync("data/graphics/general/hand.png"),
            system: load_texture_sync("data/graphics/general/unit_defence.png"),
        };
        Self {
            screen_width,
            screen_height,
            tiles,
            battle_icons
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

    pub fn display_battle_interface(&self) {
        if widgets::Button::new(self.battle_icons.magic.clone())
            .size(vec2(80., 80.))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.attack.clone())
            .size(vec2(80., 80.))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.defend.clone())
            .size(vec2(80., 80.))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.wait.clone())
            .size(vec2(80., 80.))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.run.clone())
            .size(vec2(80., 80.))
            .position(vec2(self.screen_width - 80.0, 0.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.negotiate.clone())
            .size(vec2(80., 80.))
            .position(vec2(self.screen_width - 80.0, 80.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.system.clone())
            .size(vec2(80., 80.))
            .position(vec2(self.screen_width - 80.0, 160.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
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
