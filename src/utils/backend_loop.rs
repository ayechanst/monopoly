use crate::{
    bevy_logic::helpers::buttons::{Command, PlayerCommand},
    models::board::Board,
};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum OldChange {
    InitGame,
    PositionChange,
    BalanceChange,
    PropertyStateChange,
}

#[derive(Debug)]
pub struct Change {
    pub init_game: bool,
    pub landed_on_property: bool, // prompts ui
    pub new_position: Option<(f32, f32)>,
    pub balance_change: i32,
}

// 1. Takes in `Receiver<Command>` from frontend
// 2. Takes in a `Sender<Change>` to send back to frontend
// Receiver<T> & Sender<T> allows messages to be passed between threads

// pub fn backend_loop(command_receiver: Receiver<Command>, update_transmitter: Sender<Change>) {
pub fn backend_loop(command_receiver: Receiver<PlayerCommand>, update_transmitter: Sender<Change>) {
    let mut board = Board::new();
    for player_command in command_receiver {
        let PlayerCommand {
            player_number,
            command,
        } = player_command;
        println!("(backend)-----------------Received command: {:?}", command);

        match command {
            Command::SpawnBoard => {
                // update_transmitter.send(Change::InitGame).unwrap();
                update_transmitter
                    .send(Change {
                        init_game: true,
                        landed_on_property: false,
                        new_position: None,
                        balance_change: 0,
                    })
                    .unwrap();
                // update_transmitter.send(Change::PositionChange).unwrap();
            }
            Command::RollDice => {
                // board.player_turn(player_number, update_transmitter.clone());

                board.player_turn(player_number);

                // return player_position

                let player_position = board.get_position(player_number);
                // return a button that says either "buy" or "auction"
                println!("+++++++++++++++player position: {:?}", player_position);
            }
            Command::BuyProperty => println!("boom"),
            Command::AuctionProperty => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
    }
    println!("Backend loop exiting");
}
