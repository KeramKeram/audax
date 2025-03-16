#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameEvent {
    TileClicked,
    WindowResized,
    MouseCliked
}

pub enum GuiEvent {
    BackLightTile
}

