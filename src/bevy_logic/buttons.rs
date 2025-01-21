use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

// CommandSender needs to be a resource so bevy knows how to work with it.
#[derive(Resource)]
pub struct CommandSender(pub Sender<Command>);
// Sender<Command> allows systems to send commands, as seen in the match statements
pub enum Command {
    SpawnBoard,
    RollDice,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

// Res<T> is a type of `Resource`
// pub fn buttons(commands: Res<CommandSender>, mut contexts: EguiContexts, mut spawned: Local<bool>) {
pub fn buttons(commands: Res<CommandSender>, mut contexts: EguiContexts, mut spawned: Local<bool>) {
    egui::Window::new("Game Controls").show(contexts.ctx_mut(), |ui| {
        if ui.button("Initialize Game").clicked() && !*spawned {
            *spawned = true;
            // sending command through Command::SpawnBoard because the `Command` enum
            // is inside of `CommandSender`
            commands.0.send(Command::SpawnBoard).unwrap();
        }
        if ui.button("Roll Dice").clicked() {
            commands.0.send(Command::RollDice).unwrap();
            println!("Roll Dice was clicked");
        }
    });
}
