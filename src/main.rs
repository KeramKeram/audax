mod common;
mod display;
mod game;

use crate::common::io::MousePosition;
use crate::game::{GameEvent, GuiEvent};
use bincode::{config};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex, mpsc};

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
        event_loop.register_handler(GameEvent::MouseCliked, handler_mouse_cliked.clone());
        event_loop.register_handler(GameEvent::WindowResized, handler_window_size.clone());
        event_loop.start();
    });

    let board_renderer = display::BoardRenderer::new(board.clone());
    let config = config::standard();

    let unit = display::Unit { id: 0 };
    board.lock().unwrap().add_unit(0, 0, unit);

    loop {
        board_renderer.display();
        board_renderer.display_battle_interface();
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let position = MousePosition(mouse_x, mouse_y);
            let encoded: Vec<u8> = bincode::encode_to_vec(&position, config).unwrap();
            tx.send((GameEvent::MouseCliked, encoded)).unwrap();
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
                        if let Some(tile) = tiles.get_mut(tile_index) {
                            tile.back_light = true;
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