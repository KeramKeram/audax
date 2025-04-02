use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct MousePosition(pub f32, pub f32);