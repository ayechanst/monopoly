use crate::{
    bevy_logic::helpers::buttons::{Command, PlayerCommand},
    models::board::{Board, TurnOutcomeForFrontend},
};
use bevy::prelude::Resource;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

#[derive(Resource)]
pub struct TurnOutcomeReceiver(pub Arc<Mutex<Receiver<TurnOutcomeForFrontend>>>);

pub fn backend_loop(
    command_receiver: Receiver<PlayerCommand>,
    update_transmitter: Sender<TurnOutcomeForFrontend>,
) {
    let mut board = Board::new();
    for player_command in command_receiver {
        let PlayerCommand { command } = player_command;
        match command {
            Command::SpawnBoard => {
                let snapshot = board.snapshot();
                update_transmitter
                    .send(TurnOutcomeForFrontend::BoardUpdated(snapshot))
                    .unwrap();
            }
            Command::RollDice => {
                let outcome = board.player_turn();
                update_transmitter.send(outcome).unwrap();
                // send another sender along with outcome
            }
            Command::BuyProperty => println!("boom"),
            Command::AuctionProperty => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
            Command::PassTurn => {
                board.pass_turn();
                let snapshot = board.snapshot();
                println!("updating transmitter with snapshot");
                update_transmitter
                    .send(TurnOutcomeForFrontend::BoardUpdated(snapshot))
                    .unwrap();
            }
        }
    }
    println!("Backend loop exiting");
}
