mod models;
use models::board::Board;
use std::io::{self, Write};
mod utils;

fn main() {
    let mut board = Board::new();
    for i in 0..4 {
        println!("It's player {}'s turn!", i);
        board.player_turn(i);
        wait_for_player();
    }
}

fn wait_for_player() {
    print!("Press Enter to pass the turn... ");
    io::stdout().flush().unwrap(); // Ensures the message is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Waits for player input
}
