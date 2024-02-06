#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    White,
    Black,
}

use Player::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::coord::Coord;

impl Player {
    pub fn toggle(&self) -> Player {
        match self {
            White => Black,
            Black => White,
        }
    }

    pub fn pawn_dir(&self) -> Coord {
        match self {
            White => (-1, 0).into(),
            Black => (1, 0).into(),
        }
    }
}
