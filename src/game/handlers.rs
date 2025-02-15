use crate::common::display::WindowSize;
use crate::game::event_loop::{Handler, Payload};
use crate::game::GameEvent;
pub struct MouseClickHandler {}

impl Handler for MouseClickHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        println!("Mouse clicked Event Loop!");
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
