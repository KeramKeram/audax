use bincode::{Decode, Encode};
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameEvent {
    TileClicked,
    WindowResized,
    MouseCliked,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Encode, Decode)]
pub enum GuiEvent {
    BackLightTile
}

