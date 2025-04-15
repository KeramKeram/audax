use crate::display::GameState;
use crate::display::tile::TileType;
use crate::game::GuiEvent;
use crate::game::event_loop::Payload;
use bincode::config::Configuration;
use std::sync::{Arc, mpsc};

pub struct MoveUnit {
    game_state: Arc<GameState>,
    tx: mpsc::Sender<(GuiEvent, Payload)>,
}

impl MoveUnit {
    pub fn new(game_state: Arc<GameState>, tx: mpsc::Sender<(GuiEvent, Payload)>) -> Self {
        Self { game_state, tx }
    }

    pub fn move_unit(&self, config: Configuration, index: usize, last_selected_index: usize) {
        let mut tiles = self.game_state.tiles.lock().unwrap();
        if let Some(last_tile) = tiles.get_mut(last_selected_index) {
            if let Some(unit) = last_tile.get_unit() {
                let encoded: Vec<u8> = bincode::encode_to_vec((index, unit.id), config).unwrap();
                let tile = tiles.get_mut(index).unwrap();
                tile.tile_type = TileType::MyUnit;
                self.tx.send((GuiEvent::MoveUnit, encoded)).unwrap();
            }
        } else {
            return;
        }
        let last_tile = tiles.get_mut(last_selected_index).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::GameState;
    use crate::display::tile::Tile;
    use bincode::config;
    use std::sync::mpsc::TryRecvError; // Zmiana z mpmc na mpsc
    use std::sync::{Mutex, mpsc};

    #[test]
    fn test_no_selected_unit() {
        let (tx, rx) = mpsc::channel();
        let game_state = GameState {
            tiles: Arc::new(Mutex::new(vec![
                Tile::new_for_test(TileType::Empty);
                GameState::GRID_SIZE * GameState::GRID_SIZE
            ])),
        };
        let config = config::standard();

        let sut = MoveUnit::new(Arc::new(game_state), tx.clone());
        sut.move_unit(config, 0, 1);
        let response = rx.try_recv();
        assert_eq!(rx.try_recv().is_err(), true);
        assert_eq!(rx.try_recv().err().unwrap(), TryRecvError::Empty);
    }
}
