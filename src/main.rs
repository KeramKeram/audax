mod common;
mod display;
mod game;

use crate::common::io::MousePosition;
use crate::game::{GameEvent, GuiEvent};
use bincode::{config};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use crate::display::{Board, GameState};
use crate::display::tile::Tile;

fn back_light_tiles(move_count: usize, tiles_size: usize, tile_index: usize, tiles: &mut std::sync::MutexGuard<'_, Vec<Tile>>) {
    for i in 0..move_count {
        if i < tiles_size {
            if (tile_index >= GameState::GRID_SIZE) {
                let minus_row = tile_index.checked_sub(GameState::GRID_SIZE);
                if let Some(minus_row) = minus_row {
                    if let Some(tile) = tiles.get_mut(minus_row) {
                        tile.back_light = true;
                    }
                }
            }

            if tile_index <= (GameState::GRID_SIZE * GameState::GRID_SIZE - 1) {
                let plus_row = tile_index.checked_add(GameState::GRID_SIZE);
                if let Some(plus_row) = plus_row {
                    if let Some(tile) = tiles.get_mut(plus_row) {
                        tile.back_light = true;
                    }
                }
            }

            if (tile_index % GameState::GRID_SIZE) != 0 {
                let minus_column = tile_index.checked_sub(1);
                if let Some(minus_column) = minus_column {
                    if let Some(tile) = tiles.get_mut(minus_column) {
                        tile.back_light = true;
                    }
                }
            }

            if tile_index >= GameState::GRID_SIZE && (tile_index % GameState::GRID_SIZE != 0) {
                let corner_column_left = tile_index.checked_sub(GameState::GRID_SIZE + 1);
                if let Some(corner_column_left) = corner_column_left {
                    if let Some(tile) = tiles.get_mut(corner_column_left) {
                        tile.back_light = true;
                    }
                }
            }

            if tile_index >= GameState::GRID_SIZE && ((tile_index + 1) % GameState::GRID_SIZE != 0) {
                let corner_column_rh = tile_index.checked_sub(GameState::GRID_SIZE - 1);
                if let Some(corner_column_rh) = corner_column_rh {
                    if let Some(tile) = tiles.get_mut(corner_column_rh) {
                        tile.back_light = true;
                    }
                }
            }

            if tile_index <= (GameState::GRID_SIZE * GameState::GRID_SIZE - 1) && (tile_index % GameState::GRID_SIZE) != 0 {
                let corner_column_left_down = tile_index.checked_add(GameState::GRID_SIZE - 1);
                if let Some(corner_column_left_down) = corner_column_left_down {
                    if let Some(tile) = tiles.get_mut(corner_column_left_down) {
                        tile.back_light = true;
                    }
                }
            }


            if ((tile_index + 1) % GameState::GRID_SIZE) != 0 {
                let plus_column = tile_index.checked_add(1);
                if let Some(plus_column) = plus_column {
                    if let Some(tile) = tiles.get_mut(plus_column) {
                        tile.back_light = true;
                    }
                }
            }

            if tile_index <= (GameState::GRID_SIZE * GameState::GRID_SIZE - 1) && ((tile_index + 1) % GameState::GRID_SIZE) != 0 {
                let corner_column_rh_down = tile_index.checked_add(GameState::GRID_SIZE + 1);
                if let Some(corner_column_rh_down) = corner_column_rh_down {
                    if let Some(tile) = tiles.get_mut(corner_column_rh_down) {
                        tile.back_light = true;
                    }
                }
            }
        }
    }
}
#[macroquad::main("Grid Example")]
async fn main() {
    let mut screen_height: f32 = 800.0;
    let mut screen_width: f32 = 600.0;

    let (board_obj, game_stat) = display::Board::new(screen_width, screen_height);
    let board = Arc::new(Mutex::new(board_obj));
    let game_state = Arc::new(game_stat);

    let (tx, rx) = mpsc::channel();
    let (tx_gui, rx_gui) = mpsc::channel();

    let handler_mouse_cliked = Arc::new(Mutex::new(game::MouseClickHandler::new(
        game_state,
        board.clone(),
        tx_gui.clone(),
    )));
    let handler_window_size = Arc::new(Mutex::new(crate::game::WindowResizeHandler {}));

    let loop_thread = std::thread::spawn(move || {
        let event_loop = crate::game::EventLoop::new(rx);
        event_loop.register_handler(GameEvent::MouseClicked, handler_mouse_cliked.clone());
        event_loop.register_handler(GameEvent::WindowResized, handler_window_size.clone());
        event_loop.start();
    });

    let board_renderer = display::BoardRenderer::new(board.clone());
    let config = config::standard();

    let unit = display::Unit { id: 0, move_range: 2 };
    board.lock().unwrap().add_unit(0, 0, unit);

    loop {
        board_renderer.display();
        board_renderer.display_battle_interface();
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let position = MousePosition(mouse_x, mouse_y);
            let encoded: Vec<u8> = bincode::encode_to_vec(&position, config).unwrap();
            tx.send((GameEvent::MouseClicked, encoded)).unwrap();
            print!("Mouse clicked at ({}, {})\n", mouse_x, mouse_y);
        }

        if screen_width != macroquad::window::screen_width()
            || screen_height != macroquad::window::screen_height()
        {
            screen_width = macroquad::window::screen_width();
            screen_height = macroquad::window::screen_height();
            board
                .lock()
                .unwrap()
                .update_screen_size(screen_width, screen_height);
        }

        if let Ok((event, payload)) = rx_gui.try_recv() {
            match event {
                GuiEvent::BackLightTile => {
                    let (tile_index, _): (usize, usize) =
                        bincode::decode_from_slice(&payload[..], config).unwrap();
                    println!("Backlighting tile at index: {}", tile_index);
                    {
                        let mut board_guard = board.lock().unwrap();
                        board_guard.reset_back_light_all_tiles();
                        let mut tiles = board_guard.game_state.tiles.lock().unwrap();
                        let tiles_size = tiles.len();
                        if let Some(tile) = tiles.get_mut(tile_index) {
                            tile.back_light = true;
                            if let Some(unit) = tile.get_unit() {
                                let move_count = unit.move_range + 1;
                                back_light_tiles(move_count, tiles_size, tile_index, &mut tiles);
                            }
                        }
                    }
                },
                GuiEvent::MoveUnit => {
                    let mut board_guard = board.lock().unwrap();
                    board_guard.reset_back_light_all_tiles();
                    let ((tile_index, unit_id), _): ((usize, usize), usize) =
                        bincode::decode_from_slice(&payload[..], config).unwrap();
                    println!("Move tile at index: {}", tile_index);
                    {
                        board_guard.move_unit(tile_index, unit_id);
                    }
                }
            }
        }

        next_frame().await
    }
    loop_thread.join().unwrap();
}