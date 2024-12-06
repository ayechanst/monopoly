mod models;
use models::board::Board;
fn main() {
    let mut board = Board::new();
    {
        let mut player_one = &board.players[0];

        println!("sigma: {}", player_one.position);
        println!("balls: {}", player_one.money);
        board.player_turn(player_one.clone());

        let player = &board.players[0];
        println!("poop: {}", player.position);
        println!("pee: {}", player.money);
        println!("vomit: {:?}", player.properties)
    }
}
