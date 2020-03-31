use std::fmt;
use chess_macros::chess_board;
use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug)]
pub struct Piece {
    kind: Kind,
    side: Side,
}

#[derive(Debug)]
pub struct Board {
    squares: Vec<Option<Piece>>,
}

#[cfg(wasm)]
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    current_board: Board,
    current_side: Side,
}

#[derive(Debug)]
pub struct Game {
    pub current_board: Board,
    pub current_side: Side,
}

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
}


impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for line in self.squares.chunks(8) {
            for square in line {
                let symbol = match square {
                    Some(piece) => format!("{}", piece),
                    None => String::from("_"),
                };
                write!(formatter, "{} ", symbol)?;
            }
            write!(formatter, "\n")?;
        }
        Ok(())
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_board: Board::default(),
            current_side: Side::White,
        }
    }
}

#[wasm_bindgen]
impl Game {
    pub fn make_board() -> String {
        format!("{}", Game::new().current_board)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Piece {kind, side: Side::White } => format!("{}", kind),
            Piece {kind, side: Side::Black } => format!("{}", kind).to_ascii_lowercase(),
        };
        write!(formatter, "{}", symbol)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Kind::Pawn => "P",
            Kind::Bishop => "B",
            Kind::Rook => "R",
            Kind::Knight => "K",
            Kind::Queen => "Q",
            Kind::King => "U",
        };
        write!(formatter, "{}", symbol)
    }
}
