use crate::bevy_logic::player_components::FrontendPlayer;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

#[derive(Resource, Debug)]
pub struct PlayerCommand {
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

pub fn buttons(
    commands: Res<PlayerCommandSender>,
    mut contexts: EguiContexts,
    mut spawned: Local<bool>,
    query: Query<&FrontendPlayer>,
) {
    egui::Window::new("Game Controls").show(contexts.ctx_mut(), |ui| {
        if !*spawned {
            if ui.button("Initialize Game").clicked() {
                *spawned = true;
                commands
                    .0
                    .send(PlayerCommand {
                        command: Command::SpawnBoard,
                    })
                    .unwrap();
                println!("(buttons)----------- Command::SpawnBoard success");
            }
        } else {
            ui.label("Game initialized");
        }
        for player in query.iter() {
            if player.active_player {
                if ui
                    .button(format!("Player {:?} Roll Dice", player.player_number))
                    .clicked()
                {
                    commands
                        .0
                        .send(PlayerCommand {
                            command: Command::RollDice,
                        })
                        .unwrap();
                    println!("Roll Dice was clicked");
                }
                if ui
                    .button(format!("Player {:?} Pass Turn", player.player_number))
                    .clicked()
                {
                    commands
                        .0
                        .send(PlayerCommand {
                            command: Command::PassTurn,
                        })
                        .unwrap();
                }
                // if player.input_required == true
            }
        }
    });
}
