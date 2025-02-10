mod display;
mod game;

use crate::game::{game_event, GameEvent, Dispatcher};
use macroquad::prelude::*;
use std::sync::{Arc};

#[macroquad::main("Grid Example")]
async fn main() {
    let board = display::Board::new();
    let mut dispatcher = Dispatcher::new();
    let handler = Arc::new(crate::game::MouseClickHandler {});
    let loop_thread = std::thread::spawn(move || {
        let event_loop = Arc::new(crate::game::EventLoop::new(vec![]));
        dispatcher.tx = event_loop.get_sender();
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
            dispatcher.tx.send((GameEvent::TileClicked, my_vec)).unwrap();
            print!("Mouse clicked at ({}, {})\n", mouse_x, mouse_y);
        }

        next_frame().await
    }
    loop_thread.join().unwrap();
}
