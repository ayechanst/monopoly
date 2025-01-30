use crate::{
    bevy_logic::helpers::buttons::{Command, PlayerCommand},
    models::{
        board::{Board, RequiredInputsForFrontend, TurnOutcomeForFrontend},
        board_msg::BoardMsg,
    },
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

// CALLS BACKEND LOGIC
// pub fn backend_loop(command_receiver: Receiver<PlayerCommand>, update_transmitter: Sender<Change>) {
pub fn backend_loop(
    command_receiver: Receiver<PlayerCommand>,
    update_transmitter: Sender<TurnOutcomeForFrontend>,
) {
    let mut board = Board::new();

    for player_command in command_receiver {
        let PlayerCommand {
            player_number,
            command,
        } = player_command;
        println!("(backend)-----------------Received command: {:?}", command);

        // focus on sending strings back and forth to make sure the channel works correctly
        match command {
            Command::SpawnBoard => {
                let snapshot = board.snapshot();
                update_transmitter.send(TurnOutcomeForFrontend::BoardUpdated(snapshot));
            }
            Command::RollDice => {
                let outcome = board.player_turn(player_number);
                // will either send BoardSnapshot or requiredInput enum
                update_transmitter.send(outcome);
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
