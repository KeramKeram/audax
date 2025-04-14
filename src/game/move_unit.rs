use crate::display::tile::TileType;
use crate::display::{GameState};
use crate::game::event_loop::{Payload};
use crate::game::{GuiEvent};
use std::sync::{mpsc, Arc};
use bincode::config::Configuration;

pub struct MoveUnit {
    game_state: Arc<GameState>,
    tx: mpsc::Sender<(GuiEvent, Payload)>,
}

impl MoveUnit {
    pub fn new(game_state: Arc<GameState>, tx: mpsc::Sender<(GuiEvent, Payload)>) -> Self {
        Self { game_state, tx }
    }

    pub fn move_unit(&self, config: Configuration, index: usize, last_selested_index: usize) {
        let mut tiles = self.game_state.tiles.lock().unwrap();
        let last_tile = tiles.get_mut(last_selested_index).unwrap();
        let encoded: Vec<u8> = bincode::encode_to_vec((index, last_tile.get_unit().unwrap().id), config).unwrap();
        let tile = tiles.get_mut(index).unwrap();
        tile.tile_type = TileType::MyUnit;
        self.tx.send((GuiEvent::MoveUnit, encoded)).unwrap();
    }
}