use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

use crate::bevy_logic::player_components::{Player, PlayerNumber};

#[derive(Resource, Debug)]
pub struct PlayerCommand {
    pub player_number: usize,
    pub command: Command,
}

#[derive(Resource)]
pub struct PlayerCommandSender(pub Sender<PlayerCommand>);

#[derive(Debug)]
pub enum Command {
    SpawnBoard,
    RollDice,
    BuyProperty,
    AuctionProperty,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

pub fn game_loop(
    mut contexts: EguiContexts,
    commands: Res<PlayerCommandSender>,
    selected_players: &mut u32,
) {
    egui::Window::new("Main Menu").show(contexts.ctx_mut(), |ui| {
        ui.label("Select number of players: ");
        for num in 2..=4 {
            if ui
                .radio_value(selected_players, num, format!("{num} players "))
                .clicked()
            {
                println!("{num} players have been selected");
            }
        }
        if ui.button("Initialize Game").clicked() {
            commands
                .0
                .send(PlayerCommand {
                    player_number: 1,
                    command: Command::SpawnBoard,
                })
                .unwrap();
        }
    });
}

pub fn buttons(
    commands: Res<PlayerCommandSender>,
    mut contexts: EguiContexts,
    mut spawned: Local<bool>,
    query: Query<(Entity, &Player, &PlayerNumber)>,
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
    });
}
