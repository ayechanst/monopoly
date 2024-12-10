mod models;
use std::{cell::RefCell, rc::Rc};

use models::board::Board;
fn main() {
    let mut board = Board::new();
    board.player_turn(0);
    board.player_turn(1);
    board.player_turn(2);
    board.player_turn(3);

    // let boi = String::from("Garcon");
    // let shared_string = Rc::new(RefCell::new(boi));
    // let string_ref = shared_string.borrow_mut();
}
