use super::tile::{Tile, TileType};
use crate::common::display::texture::load_texture_sync;
use crate::common::display::WindowSize;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{
    clear_background, draw_rectangle_lines, vec2
    , Texture2D,
};
use macroquad::ui::{
    root_ui,
    widgets::{self},
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct BattleIcons {
    attack: Texture2D,
    defend: Texture2D,
    magic: Texture2D,
    wait: Texture2D,
    run: Texture2D,
    negotiate: Texture2D,
    system: Texture2D,
}

#[derive(Clone)]
pub struct GameState {
    pub tiles: Arc<Mutex<Vec<Tile>>>,
}

impl GameState {
    const GRID_SIZE: usize = 12;
}

#[derive(Clone)]
pub struct Board {
    window_size: WindowSize,
    pub game_state: GameState,
    square_size: f32,
    battle_icons: BattleIcons,
}

impl Board {
    const SQUARE_SIZE: f32 = 50.0;
    pub fn new(width: f32, height: f32) -> (Self, GameState) {
        let square_size = Self::SQUARE_SIZE;
        let window_size = WindowSize {
            screen_width: width,
            screen_height: height,
        };
        let game_state = GameState {
            tiles: Arc::new(Mutex::new(vec![
                Tile::new(
                    TileType::Empty,
                    "data/graphics/general/empty_tail.png"
                );
                GameState::GRID_SIZE * GameState::GRID_SIZE
            ])),
        };
        let battle_icons = BattleIcons {
            attack: load_texture_sync("data/graphics/ui/battle/attack.png"),
            defend: load_texture_sync("data/graphics/ui/battle/defence.png"),
            magic: load_texture_sync("data/graphics/ui/battle/magic.png"),
            wait: load_texture_sync("data/graphics/ui/battle/wait.png"),
            run: load_texture_sync("data/graphics/general/hands.png"),
            negotiate: load_texture_sync("data/graphics/general/hand.png"),
            system: load_texture_sync("data/graphics/general/unit_defence.png"),
        };
        (Self {
            window_size,
            game_state: game_state.clone(),
            square_size,
            battle_icons,
        }, game_state)
    }

    pub fn check_if_is_in_boundries(&self, x: f32, y: f32) -> bool {
        let grid_width = GameState::GRID_SIZE as f32 * self.square_size;
        let grid_height = GameState::GRID_SIZE as f32 * self.square_size;
        let offset_x = (self.window_size.screen_width - grid_width) / 2.0;
        let offset_y = (self.window_size.screen_height - grid_height) / 2.0;

        if x < offset_x || x > offset_x + grid_width || y < offset_y || y > offset_y + grid_height {
            return false;
        }
        return true;
    }
    pub fn get_tile_index(&self, x: f32, y: f32) -> Option<usize> {
        if (!self.check_if_is_in_boundries(x, y)) {
            return None;
        }

        let grid_width = GameState::GRID_SIZE as f32 * self.square_size;
        let grid_height = GameState::GRID_SIZE as f32 * self.square_size;
        let offset_x = (self.window_size.screen_width - grid_width) / 2.0;
        let offset_y = (self.window_size.screen_height - grid_height) / 2.0;
        let col = ((x - offset_x) / self.square_size) as usize;
        let row = ((y - offset_y) / self.square_size) as usize;
        Some(row * GameState::GRID_SIZE + col)
    }

    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        self.window_size = WindowSize {
            screen_width: width,
            screen_height: height,
        };

        let target_grid_size = f32::min(width, height) * 0.8;
        self.square_size = target_grid_size / GameState::GRID_SIZE as f32;
    }
}


pub struct BoardRenderer {
    board: Arc<Mutex<Board>>,
}

impl BoardRenderer {
    pub fn new(board: Arc<Mutex<Board>>) -> Self {
        Self { board }
    }
    pub fn display(&self) {
        clear_background(WHITE);

        let grid_width = GameState::GRID_SIZE as f32 * self.board.lock().unwrap().square_size;
        let grid_height = GameState::GRID_SIZE as f32 * self.board.lock().unwrap().square_size;
        let offset_x = (self.board.lock().unwrap().window_size.screen_width - grid_width) / 2.0;
        let offset_y = (self.board.lock().unwrap().window_size.screen_height - grid_height) / 2.0;

        for row in 0..GameState::GRID_SIZE {
            for col in 0..GameState::GRID_SIZE {
                let x = offset_x + col as f32 * self.board.lock().unwrap().square_size;
                let y = offset_y + row as f32 * self.board.lock().unwrap().square_size;
                let square_size = self.board.lock().unwrap().square_size;
                draw_rectangle_lines(x, y, square_size, square_size, 2.0, BLACK);
            }
        }
    }

    pub fn display_battle_interface(&self) {
        let x = 80.0;
        let y = 80.0;

        let board = self.board.lock().unwrap();
        let screen_width = board.window_size.screen_width;
        let icons = &board.battle_icons;

        if widgets::Button::new(icons.magic.clone())
            .size(vec2(x, y))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.attack.clone())
            .size(vec2(x, y))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.defend.clone())
            .size(vec2(x, y))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.wait.clone())
            .size(vec2(x, y))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.run.clone())
            .size(vec2(x, y))
            .position(vec2(screen_width - 80.0, 0.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.negotiate.clone())
            .size(vec2(x, y))
            .position(vec2(screen_width - 80.0, 80.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }

        if widgets::Button::new(icons.system.clone())
            .size(vec2(x, y))
            .position(vec2(screen_width - 80.0, 160.0))
            .ui(&mut *root_ui())
        {
            println!("Textured button clicked!");
        }
    }
}