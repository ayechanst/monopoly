mod models;
use models::board::Board;
fn main() {
    let mut board = Board::new();
    // let mut players = &mut board.players;
    board.player_turn(0);
    board.player_turn(1);
    board.player_turn(2);
    board.player_turn(3);
}
