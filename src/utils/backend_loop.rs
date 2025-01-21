use crate::{models::board::Board, Command};
use bevy::prelude::Resource;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Resource)]
// Wrap Change in a Sender (to send it)
pub struct ChangeSender(pub Sender<Change>);
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
    while let Ok(command) = command_receiver.try_recv() {
        match command {
            // Command::SpawnBoard => println!("boom"),
            Command::SpawnBoard => {
                update_transmitter.send(Change::InitGame).unwrap();
            }
            Command::RollDice => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
    }
}
