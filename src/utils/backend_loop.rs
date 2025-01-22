use crate::{models::board::Board, Command};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum Change {
    InitGame,
    PositionChange,
    BalanceChange,
    PropertyStateChange,
}

// 1. Takes in `Receiver<Command>` from frontend
// 2. Takes in a `Sender<Change>` to send back to frontend
// Receiver<T> & Sender<T> allows messages to be passed between threads
pub fn backend_loop(command_receiver: Receiver<Command>, update_transmitter: Sender<Change>) {
    let mut board = Board::new();
    for command in command_receiver {
        println!("(backend)-----------------Received command: {:?}", command);
        match command {
            Command::SpawnBoard => {
                update_transmitter.send(Change::InitGame).unwrap();
                update_transmitter.send(Change::PositionChange).unwrap();
            }
            Command::RollDice => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
    }
    println!("Backend loop exiting");
}
