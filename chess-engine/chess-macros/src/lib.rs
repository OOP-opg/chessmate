use proc_macro::{TokenStream, TokenTree};

fn match_symbol(symbol: char) -> (&'static str, &'static str) {
    let (kind, side) = match symbol {
        'P' => ("Pawn", "White"),
        'p' => ("Pawn", "Black"),
        'B' => ("Bishop", "White"),
        'b' => ("Bishop", "Black"),
        'K' => ("Knight", "White"),
        'k' => ("Knight", "Black"),
        'R' => ("Rook", "White"),
        'r' => ("Rook", "Black"),
        'Q' => ("Queen", "White"),
        'q' => ("Queen", "Black"),
        'U' => ("King", "White"),
        'u' => ("King", "Black"),
        '_' => ("Empty", "Empty"),
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
    };
    (kind, side)
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
    let (kind, side) = match_symbol(symbol);
    format!(
        "
        Phigure {{
            piece: Piece {{
                kind: Kind::{kind}, 
                side: Side::{side}, 
            }},
            position: {position},
        }}
        ",
        kind = kind,
        side = side,
        position = number
    )
}

#[proc_macro]
pub fn chess_board(items: TokenStream) -> TokenStream {
    let content = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| build_square(i, item))
        .collect::<Vec<String>>()
        .join(",");

    format!("Board {{ board: vec![{}] }}", content)
        .parse::<TokenStream>()
        .unwrap()
}
