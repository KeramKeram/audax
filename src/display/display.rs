use std::alloc::System;
use super::tile::{Tile, TileType};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_rectangle_lines, screen_height, screen_width, load_texture, vec2, Texture2D, Image};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
use crate::common::display::WindowSize;
use crate::common::display::texture::load_texture_sync;

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
    window_size: WindowSize,
    pub tiles: Vec<Tile>,
    grid_size: usize,
    square_size: f32,
    battle_icons: BattleIcons,
}

impl Board {
    const GRID_SIZE: usize = 10;
    const SQUARE_SIZE: f32 = 50.0;
    pub fn new(width: f32, height: f32) -> Self {
        let grid_size = 12;
        let square_size = 50.0;
        let window_size = WindowSize {
            screen_width: width,
            screen_height: height,
        };
        let tiles = vec![Tile::new(TileType::Empty, "data/graphics/general/empty_tail.png"); grid_size * grid_size];
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
            window_size,
            tiles,
            grid_size,
            square_size,
            battle_icons
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
            .position(vec2(self.window_size.screen_width - 80.0, 0.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.negotiate.clone())
            .size(vec2(80., 80.))
            .position(vec2(self.window_size.screen_width - 80.0, 80.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(self.battle_icons.system.clone())
            .size(vec2(80., 80.))
            .position(vec2(self.window_size.screen_width - 80.0, 160.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
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

    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        self.window_size = WindowSize {
            screen_width: width,
            screen_height: height,
        };

        let target_grid_size = f32::min(width, height) * 0.8;
        self.square_size = target_grid_size / self.grid_size as f32;
    }
}
