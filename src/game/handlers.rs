use crate::game::event_loop::{Handler, Payload};
use crate::game::GameEvent;

pub struct MouseClickHandler {
}

impl Handler for MouseClickHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        println!("Mouse clicked Event Loop!");
    }
}