mod display;
mod game;

use crate::game::GameEvent;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{mpsc, Arc};

#[macroquad::main("Grid Example")]
async fn main() {
    let mut screen_height: f32 = macroquad::window::screen_height();
    let mut screen_width: f32 = macroquad::window::screen_width();

    let board = display::Board::new(screen_width, screen_height);
    let (tx, rx) = mpsc::channel();
    let handler = Arc::new(crate::game::MouseClickHandler {});
    let loop_thread = std::thread::spawn(move || {
        let event_loop = crate::game::EventLoop::new(rx, vec![]);
        event_loop.register_handler(GameEvent::TileClicked, handler.clone());
        event_loop.start();
    });

    loop {
        board.display();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let my_vec: Vec<u8> = vec![10, 20];
            //event_loop.add_event(game_event::GameEvent::TileClicked {
            //}, my_vec);
            tx.send((GameEvent::TileClicked, my_vec)).unwrap();
            print!("Mouse clicked at ({}, {})\n", mouse_x, mouse_y);
        }

        if (screen_width != macroquad::window::screen_width()
            || screen_height != macroquad::window::screen_height())
        {
            screen_width = macroquad::window::screen_width();
            screen_height = macroquad::window::screen_height();
            //tx.send((GameEvent::TileClicked, my_vec)).unwrap();
        }

        next_frame().await
    }
    loop_thread.join().unwrap();
}
