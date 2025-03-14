use crate::common::display::WindowSize;
use crate::common::io::MousePosition;
use crate::display::{Board, GameState};
use crate::game::GameEvent;
use crate::game::event_loop::{Handler, Payload};
use bincode::config;
use std::sync::{Arc, Mutex};

pub struct MouseClickHandler {
    pub(crate) game_state: Arc<GameState>,
    board: Arc<Mutex<Board>>,
}
impl MouseClickHandler {
    pub fn new(game_state: Arc<GameState>, board: Arc<Mutex<Board>>) -> Self {
        Self { game_state, board }
    }
}
impl Handler for MouseClickHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        let config = config::standard();
        let (decoded, len): (MousePosition, usize) =
            bincode::decode_from_slice(&payload[..], config).unwrap();
        println!("Mouse clicked Event Loop! {}, {}", decoded.0, decoded.1);
        let mut stats = self.game_state.tiles.lock().unwrap();
        let click_in_area = self.board.lock().unwrap().check_if_is_in_boundries(decoded.0, decoded.1);
        if click_in_area {
            println!("Mouse clicked Event Loop! Cliked in area {}, {}", decoded.0, decoded.1);
        } else {
            println!("Mouse clicked Event Loop! Cliked NOT in area {}, {}", decoded.0, decoded.1);
        }
    }
}

pub struct WindowResizeHandler {}

impl Handler for WindowResizeHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        let (window_size, _): (WindowSize, usize) =
            bincode::decode_from_slice(payload, bincode::config::standard()).unwrap();
        println!("Window resized Event Loop! {:?}", window_size);
    }
}
