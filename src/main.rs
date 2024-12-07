mod models;
use models::board::Board;
fn main() {
    let mut board = Board::new();
    let player_one = board.players[0].clone();
    let player_two = board.players[1].clone();
    let player_three = board.players[2].clone();
    let player_four = board.players[3].clone();

    // let player_one = board.players[0];
    // let player_two = board.players[1];
    // let player_three = board.players[2].clone();
    // let player_four = board.players[3].clone();

    // println!("balls: {}", player_one.money);
    board.player_turn(player_one);
    board.player_turn(player_two);
    board.player_turn(player_three);
    board.player_turn(player_four);
    // board.player_turn(player_one.clone());
}
