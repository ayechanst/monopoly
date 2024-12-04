mod models;
use models::board::Board;
fn main() {
    let board = Board::new();
    println!("Hello, world!");
    println!("Board: {:?}", board);
}
