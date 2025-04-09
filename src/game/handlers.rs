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
    last_selected_index: Option<usize>,
}
impl MouseClickHandler {
    pub fn new(game_state: Arc<GameState>, board: Arc<Mutex<Board>>, tx: mpsc::Sender<(GuiEvent, Vec<u8>)>) -> Self {
        Self { game_state, board, tx, last_selected_index: None }
    }

    fn handle_click_in_area(&mut self, mouse_x: f32, mouse_y: f32) {
        let board = self.board.lock().unwrap();
        let config = config::standard();
        let tile_index = board.get_tile_index(mouse_x, mouse_y);
        let mut tiles = self.game_state.tiles.lock().unwrap();
        if let Some(index) = tile_index {
            let tile = tiles.get_mut(index).unwrap();
            match tile.tile_type {
                TileType::MyUnit => {
                    let encoded: Vec<u8> = bincode::encode_to_vec(index, config).unwrap();
                    self.tx.send((GuiEvent::BackLightTile, encoded)).unwrap();
                    self.last_selected_index = Some(index);
                },
                TileType::Empty => {
                    // First check if there is a selected unit
                    // then if it is my unit
                    // then try too move
                    if let Some(last_selested_index) = self.last_selected_index {
                        let last_tile = tiles.get_mut(last_selested_index).unwrap();
                        let encoded: Vec<u8> = bincode::encode_to_vec((index, last_tile.get_unit().unwrap().id), config).unwrap();
                        let tile = tiles.get_mut(index).unwrap();
                        tile.tile_type = TileType::MyUnit;
                        self.tx.send((GuiEvent::MoveUnit, encoded)).unwrap();
                    }
                    self.last_selected_index = None;
                }
                _ => {}
            }
        }
        println!(
            "Mouse clicked Event Loop! Cliked in area {}, {}",
            mouse_x, mouse_y
        );
    }
}
impl Handler for MouseClickHandler {
    fn handle(&mut self, event: &GameEvent, payload: &Payload) {
        let config = config::standard();
        let (decoded, _): (MousePosition, usize) =
            bincode::decode_from_slice(&payload[..], config).unwrap();

        let (mouse_x, mouse_y) = (decoded.0, decoded.1);
        println!("Mouse clicked Event Loop! {}, {}", mouse_x, mouse_y);

        let check_boundries = {
            let board = self.board.lock().unwrap();
            board.check_if_is_in_boundries(mouse_x, mouse_y)
        };

        if check_boundries {
            self.handle_click_in_area(mouse_x, mouse_y);
        } else {
            println!("Mouse clicked Event Loop! Out of boundries");
        }
    }
}

pub struct WindowResizeHandler {}

impl Handler for WindowResizeHandler {
    fn handle(&mut self, event: &GameEvent, payload: &Payload) {
        let (window_size, _): (WindowSize, usize) =
            bincode::decode_from_slice(payload, bincode::config::standard()).unwrap();
        println!("Window resized Event Loop! {:?}", window_size);
    }
}
