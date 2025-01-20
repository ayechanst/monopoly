use crate::{models::board::Board, Command};
use std::sync::mpsc::{Receiver, Sender};

pub fn backend_loop(board: &mut Board, command_rx: Receiver<Command>, update_tx: Sender<Board>) {
    while let Ok(command) = command_rx.try_recv() {
        match command {
            Command::SpawnBoard => println!("boom"),
            Command::RollDice => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
        update_tx.send(board.clone()).unwrap()
    }
}
