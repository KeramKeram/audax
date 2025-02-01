use crate::game::event_loop::{Handler, Payload};
use crate::game::GameEvent;

pub struct MouseClickHandler {
}

impl Handler for MouseClickHandler {
    fn handle(&self, _event: &GameEvent, _payload: &Payload) {
        println!("Mouse clicked Event Loop!");
    }
}