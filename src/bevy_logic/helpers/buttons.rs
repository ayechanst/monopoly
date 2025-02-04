use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

use crate::bevy_logic::player_components::{FrontendPlayer, PlayerNumber};

#[derive(Resource, Debug)]
pub struct PlayerCommand {
    pub player_number: usize,
    pub command: Command,
}

#[derive(Resource)]
pub struct PlayerCommandSender(pub Sender<PlayerCommand>);

#[derive(Resource, Debug)]
pub enum Command {
    SpawnBoard,
    RollDice,
    BuyProperty,
    AuctionProperty,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
    PassTurn,
}
// TODO: make a window for each player displaying their data

pub fn buttons(
    commands: Res<PlayerCommandSender>,
    // commands: Res<Command>,
    mut contexts: EguiContexts,
    mut spawned: Local<bool>,
) {
    egui::Window::new("Game Controls").show(contexts.ctx_mut(), |ui| {
        if !*spawned {
            if ui.button("Initialize Game").clicked() {
                *spawned = true;
                commands
                    .0
                    .send(PlayerCommand {
                        player_number: 1,
                        command: Command::SpawnBoard,
                    })
                    // .send(Command::SpawnBoard)
                    .unwrap();
                println!("(buttons)----------- Command::SpawnBoard success");
            }
        } else {
            ui.label("Game initialized");
        }
        if ui.button("Roll Dice").clicked() {
            // trying to get current player
            // let player_number = query;

            commands
                .0
                .send(PlayerCommand {
                    player_number: 1,
                    command: Command::RollDice,
                })
                .unwrap();
            println!("Roll Dice was clicked");
        }
        if ui.button("Pass Turn").clicked() {
            commands
                .0
                .send(PlayerCommand {
                    player_number: 1,
                    command: Command::PassTurn,
                })
                .unwrap();
        }
    });
}
