use chess_engine::Board;

pub fn run_cli() {
    println!("Command line use");
    let board = Board::default().to_string();
    println!("{}", board);
}
