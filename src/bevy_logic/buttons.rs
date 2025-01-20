use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::mpsc::Sender;

#[derive(Resource)]
pub struct CommandSender(pub Sender<Command>);
pub enum Command {
    SpawnBoard,
    RollDice,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

pub fn buttons(commands: Res<CommandSender>, mut contexts: EguiContexts, mut spawned: Local<bool>) {
    egui::Window::new("Game Controls").show(contexts.ctx_mut(), |ui| {
        if ui.button("Spawn Board").clicked() && !*spawned {
            *spawned = true;
            commands.0.send(Command::SpawnBoard).unwrap();
        }
        if ui.button("Roll Dice").clicked() {
            commands.0.send(Command::RollDice).unwrap();
            println!("Roll Dice was clicked");
        }
    });
}
