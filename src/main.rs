mod display;
mod game;
mod common;

use crate::common::io::MousePosition;
use crate::game::{GameEvent, GuiEvent};
use macroquad::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use bincode::{config, Decode, Encode};

#[macroquad::main("Grid Example")]
async fn main() {
    let mut screen_height: f32 = 800.0;
    let mut screen_width: f32 =600.0;

    let (mut board_obj, mut game_stat) = display::Board::new(screen_width, screen_height);
    let mut board = Arc::new(Mutex::new(board_obj));
    let mut game_state = Arc::new(game_stat);

    let (tx, rx) = mpsc::channel();
    let (txGui, rxGui) = mpsc::channel();

    let handler_mouse_cliked = Arc::new(game::MouseClickHandler::new(game_state, board.clone(), txGui.clone()));
    let handler_window_size = Arc::new(crate::game::WindowResizeHandler {});

    let loop_thread = std::thread::spawn(move || {
        let event_loop = crate::game::EventLoop::new(rx, vec![]);
        event_loop.register_handler(GameEvent::MouseCliked, handler_mouse_cliked.clone());
        event_loop.register_handler(GameEvent::WindowResized, handler_window_size.clone());
        event_loop.start();
    });

    let mut board_renderer = display::BoardRenderer::new(board.clone());
    let config = config::standard();
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
            board.lock().unwrap().update_screen_size(screen_width, screen_height);
        }

        if let Ok((event, payload)) = rxGui.try_recv() {
            match event {
                GuiEvent::BackLightTile => {
                    let (tile_index, _): (usize, usize) =
                        bincode::decode_from_slice(&payload[..], config).unwrap();
                    println!("Backlighting tile at index: {}", tile_index);
                    // Handle tile backlighting here
                }
            }
        }

        next_frame().await
    }
    loop_thread.join().unwrap();
}

/*
1. wysyłamy do loop przyciśnięcie myszy
2. loop odbiera i przekazuje do handlera
3. handler odbiera ogarnia co to było
4. generuje i wysyła do głównego wątku update czyli typ i dane
5. główny wątek odbiera i ogarnia co to było
6. np. wyciągamy tail do osobnego wątku
7. w event loop handler sprawdza czy kliknięto tail, jak tak czy jest jednostka, jak tak to podświetla taile wokół
6. czyli do głównego wątku idzie info, podswietl_taile, numery taily.

architektura:
wyciągnij do gamestats m.in tile pewnie plus wielkości
w main niech konstruktor zwraca tą strukturę
dostanie się do niej drugi wątek, pola są w aRC
 */