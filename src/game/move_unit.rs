use bincode::config::Configuration;
use crate::display::GameState;
use crate::display::tile::TileType;
use crate::display::tile::Unit;
use crate::game::GuiEvent;
use crate::game::event_loop::Payload;
use std::sync::{Arc, mpsc};

pub struct MoveUnit {
    game_state: Arc<GameState>,
    tx: mpsc::Sender<(GuiEvent, Payload)>,
}

impl MoveUnit {
    pub fn new(game_state: Arc<GameState>, tx: mpsc::Sender<(GuiEvent, Payload)>) -> Self {
        Self { game_state, tx }
    }

    pub fn move_unit(&self, config: Configuration, index: usize, last_selected_index: usize) -> Result<(), String> {
        let mut tiles = self.game_state.tiles.lock().map_err(|_| "Can't lock tile for move of unit")?;
        let mut last_tile = tiles.get_mut(last_selected_index).ok_or_else(|| "Can't get last tile")?;
        let unit = last_tile.get_unit().ok_or_else(|| "Can't get unit")?;
        let encoded: Vec<u8> = bincode::encode_to_vec((index, unit.id), config).map_err(|_| "Serialization error")?;
        let tile = tiles.get_mut(index).ok_or_else(|| "Can't get tile")?;
        tile.tile_type = TileType::MyUnit;
        self.tx.send((GuiEvent::MoveUnit, encoded)).map_err(|_| "Failed to send move unit")?;
        Ok(())
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
    macro_rules! setup_game_state {
    () => {{
        let (tx, rx) = mpsc::channel();
        let game_state = GameState {
            tiles: Arc::new(Mutex::new(vec![
                Tile::new_for_test(TileType::Empty);
                GameState::GRID_SIZE * GameState::GRID_SIZE
            ])),
        };
        let config = config::standard();
        (tx, rx, game_state, config)
    }};
}
    #[test]
    fn test_no_selected_unit() {
        let (tx, rx, game_state, config) = setup_game_state!();
        let sut = MoveUnit::new(Arc::new(game_state), tx.clone());

        sut.move_unit(config, 0, 1);
        let response = rx.try_recv();

        assert_eq!(response.is_err(), true);
        assert_eq!(response.err().unwrap(), TryRecvError::Empty);
    }

    #[test]
    fn test_move_to_index_two() {
        let (tx, rx, game_state, config) = setup_game_state!();
        let last_selected_index: usize = 0;
        let index: usize = 1;
        let mut tiles = game_state.tiles.lock().unwrap();
        if let Some(last_tile) = tiles.get_mut(last_selected_index) {
            last_tile.unit = Some(Unit { id: 0 });
        }
        drop(tiles);
        let sut = MoveUnit::new(Arc::new(game_state), tx.clone());


        sut.move_unit(config, 1, 0);
        let response = rx.try_recv();

        assert_eq!(response.is_ok(), true);
        // assert_eq!(rx.try_recv().err().unwrap(), TryRecvError::Empty);
    }
}
