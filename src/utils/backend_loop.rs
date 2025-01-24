use crate::{
    bevy_logic::helpers::buttons::{Command, PlayerCommand},
    models::board::Board,
};
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

// pub fn backend_loop(command_receiver: Receiver<Command>, update_transmitter: Sender<Change>) {
pub fn backend_loop(command_receiver: Receiver<PlayerCommand>, update_transmitter: Sender<Change>) {
    let mut board = Board::new();
    // let player_number
    for player_command in command_receiver {
        let PlayerCommand {
            player_number,
            command,
        } = player_command;
        println!("(backend)-----------------Received command: {:?}", command);
        match command {
            Command::SpawnBoard => {
                update_transmitter.send(Change::InitGame).unwrap();
                // update_transmitter.send(Change::PositionChange).unwrap();
            }
            Command::RollDice => {
                board.first_main_phase(player_number as usize);
            }
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
    }
    println!("Backend loop exiting");
}
