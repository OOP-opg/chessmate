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
    Empty,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Side {
    Empty,
    White,
    Black,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Piece {
    kind: Kind,
    side: Side,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Phigure {
    piece: Piece,
    position: u8,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Board {
    board: Vec<Phigure>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    current_board: Board,
    current_side: Side,
}

//const fn get_index(letter: char, num: u8) -> u8 {
//    (letter as u8 - 'a' as u8) * 8 + num
//}

//macro_rules! square {
//    ([$a:expr, $i:expr] => $p:ident) => (Square {
//        piece: Piece::$p, index: get_index($a, $i)
//    });
//}

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
