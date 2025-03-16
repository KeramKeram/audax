use crate::common::display::WindowSize;
use crate::common::io::MousePosition;
use crate::display::tile::TileType;
use crate::display::{Board, GameState};
use crate::game::event_loop::{Handler, Payload};
use crate::game::{GameEvent, GuiEvent};
use bincode::config;
use std::sync::{mpsc, Arc, Mutex};


pub struct MouseClickHandler {
    pub(crate) game_state: Arc<GameState>,
    board: Arc<Mutex<Board>>,
    tx: mpsc::Sender<(GuiEvent, Vec<u8>)>,
}
impl MouseClickHandler {
    pub fn new(game_state: Arc<GameState>, board: Arc<Mutex<Board>>, tx: mpsc::Sender<(GuiEvent, Vec<u8>)>) -> Self {
        Self { game_state, board, tx }
    }
}
impl Handler for MouseClickHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        let config = config::standard();
        let (decoded, len): (MousePosition, usize) =
            bincode::decode_from_slice(&payload[..], config).unwrap();
        println!("Mouse clicked Event Loop! {}, {}", decoded.0, decoded.1);
        //let mut stats = self.game_state.tiles.lock().unwrap();
        if let Ok(board) = self.board.lock() {
            let (mouse_x, mouse_y) = (decoded.0, decoded.1);
            let click_in_area = board.check_if_is_in_boundries(mouse_x, mouse_y);
            if click_in_area {
                let tile_index = board.get_tile_index(mouse_x, mouse_y);
                let mut tiles = self.game_state.tiles.lock().unwrap();
                if let Some(index) = tile_index {
                    let tile = tiles.get_mut(index).unwrap();
                    match tile.tile_type {
                        TileType::MyUnit => {
                            let config = config::standard();
                            let encoded: Vec<u8> = bincode::encode_to_vec(&index, config).unwrap();
                            self.tx.send((GuiEvent::BackLightTile, encoded)).unwrap();
                        }
                        _ => {}
                    }
                }
                println!(
                    "Mouse clicked Event Loop! Cliked in area {}, {}",
                    decoded.0, decoded.1
                );
            }
        } // lock jest automatycznie zwalniany tutaj
    }
}

pub struct WindowResizeHandler {}

impl Handler for WindowResizeHandler {
    fn handle(&self, event: &GameEvent, payload: &Payload) {
        let (window_size, _): (WindowSize, usize) =
            bincode::decode_from_slice(payload, bincode::config::standard()).unwrap();
        println!("Window resized Event Loop! {:?}", window_size);
    }
}
