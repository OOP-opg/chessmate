use proc_macro::{TokenStream, TokenTree};

fn match_symbol(symbol: char) -> Option<(&'static str, &'static str)> {
    match symbol {
        'P' => Some(("Pawn", "White")),
        'p' => Some(("Pawn", "Black")),
        'B' => Some(("Bishop", "White")),
        'b' => Some(("Bishop", "Black")),
        'K' => Some(("Knight", "White")),
        'k' => Some(("Knight", "Black")),
        'R' => Some(("Rook", "White")),
        'r' => Some(("Rook", "Black")),
        'Q' => Some(("Queen", "White")),
        'q' => Some(("Queen", "Black")),
        'U' => Some(("King", "White")),
        'u' => Some(("King", "Black")),
        '_' => None,
        err @ _ => panic!(format!(
            "
            Unexpected token: {err}
            This macro expect folloving symbols: 
                P for Pawn
                B for Bishop
                K for Knight
                R for Rook
                Q for Queen
                U for King
                _ for Empty
            ",
            err = err
        )),
    }
}

fn build_square(number: usize, piece: TokenTree) -> String {
    if number > 63 {
        panic!("Did you just put 65-th square on the board?")
    }
    let symbol = if let Some(s) = piece.to_string().chars().next() {
        s
    } else {
        panic!("Unable to get first character of identifier")
    };
    let piece = match_symbol(symbol);
    match piece {
        Some((kind, side)) => format!(
            "
            Some(
                Piece {{
                    kind: Kind::{kind}, 
                    side: Side::{side}, 
                }}
            )
            ",
            kind = kind,
            side = side,
        ),
        None => String::from("None"),
    }
}

#[proc_macro]
pub fn chess_board(items: TokenStream) -> TokenStream {
    let content = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| build_square(i, item))
        .collect::<Vec<String>>()
        .join(",");

    let board = format!("Board {{ squares: vec![{}] }}", content);

    board.parse::<TokenStream>().unwrap()
}
