use bincode::{Decode, Encode};
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct WindowSize {
    pub screen_width: f32,
    pub screen_height: f32,
}

impl WindowSize {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self { screen_width, screen_height }
    }
}