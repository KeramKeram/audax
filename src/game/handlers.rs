use bincode::{config};
use crate::common::display::WindowSize;
use crate::common::io::MousePosition;
use crate::game::event_loop::{Handler, Payload};
use crate::game::GameEvent;
use crate::display::{GameState};
use std::sync::{Arc};

pub struct MouseClickHandler {
    pub(crate) game_state: Arc<GameState>,
}
impl MouseClickHandler {
    pub fn new(game_state: Arc<GameState>) -> Self {
        Self { game_state }
    }
}
impl Handler for MouseClickHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        let config = config::standard();
        let (decoded, len): (MousePosition, usize) = bincode::decode_from_slice(&payload[..], config).unwrap();
        println!("Mouse clicked Event Loop! {}, {}", decoded.0, decoded.1);
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
