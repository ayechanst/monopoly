use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

#[derive(Resource, Debug)]
pub struct PlayerCommand {
    pub player_number: usize,
    pub command: Command,
}

#[derive(Resource)]
pub struct PlayerCommandSender(pub Sender<PlayerCommand>);

// CommandSender needs to be a resource so bevy knows how to work with it.

// #[derive(Resource)]
// pub struct CommandSender(pub Sender<Command>);

// Sender<Command> allows systems to send commands, as seen in the match statements
#[derive(Debug)]
pub enum Command {
    SpawnBoard,
    RollDice,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

// could send structs or tuples instead of enums

// pub fn buttons(commands: Res<CommandSender>, mut contexts: EguiContexts, mut spawned: Local<bool>) {
pub fn buttons(
    commands: Res<PlayerCommandSender>,
    mut contexts: EguiContexts,
    mut spawned: Local<bool>,
) {
    egui::Window::new("Game Controls").show(contexts.ctx_mut(), |ui| {
        if !*spawned {
            if ui.button("Initialize Game").clicked() {
                *spawned = true;
                commands
                    .0
                    // .send(Command::SpawnBoard)
                    .send(PlayerCommand {
                        player_number: 1,
                        command: Command::SpawnBoard,
                    })
                    // .unwrap_or_else(|_| println!("failed Command::SpawnBoard"));
                    .unwrap();
                println!("(buttons)----------- Command::SpawnBoard success");
            }
        } else {
            ui.label("Game initialized");
        }
        if ui.button("Roll Dice").clicked() {
            // commands.0.send(Command::RollDice).unwrap();
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
