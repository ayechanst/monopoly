use crate::{
    bevy_logic::{
        helpers::{
            buttons::{PlayerCommand, PlayerCommandSender},
            spawn_board::spawn_board,
            update_player::update_players,
        },
        player_components::FrontendPlayer,
    },
    models::board::{RequiredInputsForFrontend, TurnOutcomeForFrontend},
    utils::backend_loop::{backend_loop, TurnOutcomeReceiver},
};
use bevy::prelude::*;
use std::sync::{
    mpsc::{self},
    Arc, Mutex,
};

#[derive(Resource)]
pub struct GridSize(pub f32);

#[derive(Resource)]
pub struct ScaleFactor(pub f32);

pub struct FrontEndPlugin;

impl Plugin for FrontEndPlugin {
    fn build(&self, app: &mut App) {
        let (command_transmitter, command_receiver) = mpsc::channel::<PlayerCommand>();
        let (update_transmitter, update_receiver) = mpsc::channel::<TurnOutcomeForFrontend>();

        std::thread::spawn(move || {
            backend_loop(command_receiver, update_transmitter);
            println!("Backend thread exiting");
        });

        app.insert_resource(PlayerCommandSender(command_transmitter))
            .insert_resource(TurnOutcomeReceiver(Arc::new(Mutex::new(update_receiver))))
            .insert_resource(GridSize(600.0))
            .insert_resource(ScaleFactor(1.0))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(
    update_receiver: Res<TurnOutcomeReceiver>,
    query: Query<(&mut FrontendPlayer, &mut Transform)>,
    commands: Commands,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
) {
    if let Ok(receiver) = update_receiver.0.try_lock() {
        if let Ok(turn_outcome) = receiver.try_recv() {
            let players = turn_outcome.board_snapshot.players;

            if players.iter().all(|player| player.position == 0) {
                spawn_board(commands, grid_size, scale_factor, players);
            } else {
                update_players(players, query, grid_size, turn_outcome.required_input);
            }

            // match turn_outcome {
            //     TurnOutcomeForFrontend::BoardUpdated(board_snapshot) => {
            //         let players = board_snapshot.players;
            //         if players.iter().all(|player| player.position == 0) {
            //             spawn_board(commands, grid_size, scale_factor, players);
            //         } else {
            //             update_players(players, query, grid_size);
            //         }
            //     }
            //     TurnOutcomeForFrontend::InputRequiredForFrontend(
            //         required_inputs_for_frontend,
            //         board_snapshot,
            //     ) => {
            //         let players = board_snapshot.players;
            //         update_players(players, query, grid_size);

            //         match required_inputs_for_frontend {
            //             RequiredInputsForFrontend::None => println!("No input required"),
            //             RequiredInputsForFrontend::RollDice => println!("(frontend.rs) roll dice"),
            //             RequiredInputsForFrontend::Buy => {
            //                 // needs to update something to display on buttons.rs the option
            //                 println!("(frontend.rs) buy?")
            //             }
            //             RequiredInputsForFrontend::Mortgage => println!("(frontend.rs) mortgage"),
            //             RequiredInputsForFrontend::Trade => println!("(frontend.rs) trade"),
            //             RequiredInputsForFrontend::BuyHouse => println!("(frontend.rs) house"),
            //             RequiredInputsForFrontend::SellHouse => {
            //                 println!("(frontend.rs) sell house")
            //             }
            //         }
            //     }
            // }
        } else {
            // println!("no message");
        }
    } else {
        println!("Failed to acquire lock on channel");
    }
}
