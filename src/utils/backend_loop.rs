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

    let mut active_player_number = 1;

    for player_command in command_receiver {
        let PlayerCommand {
            player_number,
            command,
        } = player_command;
        println!("(backend)-----------------Received command: {:?}", command);

        match command {
            Command::SpawnBoard => {
                let snapshot = board.snapshot();

                println!("updating transmitter with snapshot");
                update_transmitter
                    .send(TurnOutcomeForFrontend::BoardUpdated(snapshot))
                    .unwrap();
                // get active player here and update the variable to RollDice can have the correct player
            }
            Command::RollDice => {
                // let outcome = board.player_turn(player_number);
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
            }
        }
    }
    println!("Backend loop exiting");
}
