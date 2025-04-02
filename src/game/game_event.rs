use bincode::{Decode, Encode};
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameEvent {
    TileClicked,
    WindowResized,
    MouseClicked,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Encode, Decode)]
pub enum GuiEvent {
    BackLightTile,
    MoveUnit
}

