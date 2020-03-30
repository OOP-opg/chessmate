use chess_macros::chess_board;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug)]
pub enum Side {
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Piece {
    kind: Kind,
    side: Side,
}

#[derive(Debug)]
pub enum Square {
    Piece(Piece),
    Empty,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Board {
    squares: Vec<Square>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    current_board: Board,
    current_side: Side,
}

#[wasm_bindgen]
impl Board {
    pub fn default() -> Board {
        chess_board! {
            r k b q u b k r
            p p p p p p p p
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            P P P P P P P P
            R K B Q U B K R
        }
    }
    pub fn to_string(&self) -> String {
        format!("Zaebis: {:?}", &self)
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            current_board: Board::default(),
            current_side: Side::White,
        }
    }
}
