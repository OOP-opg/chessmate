use chess_engine::Game;

pub fn run_cli() {
    println!("Command line use");
    let game = Game::new();
    println!("{}", game.current_board);
}
