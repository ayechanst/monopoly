use crate::{
    bevy_logic::{
        // buttons::{Command, CommandSender},
        helpers::{
            // buttons::{Command, CommandSender, PlayerCommand},
            // buttons::{CommandSender, PlayerCommand},
            buttons::{PlayerCommand, PlayerCommandSender},
            spawn_board::spawn_board,
        },
        player_components::{Offset, Position},
        systems::player_position::player_position,
    },
    utils::backend_loop::{backend_loop, Change},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

#[derive(Resource)]
pub struct ChangeReceiver(pub Arc<Mutex<Receiver<Change>>>);

#[derive(Resource)]
pub struct GridSize(pub f32);

#[derive(Resource)]
pub struct ScaleFactor(pub f32);

pub struct FrontEndPlugin;

impl Plugin for FrontEndPlugin {
    fn build(&self, app: &mut App) {
        let (command_transmitter, command_receiver) = mpsc::channel::<PlayerCommand>();
        let (update_transmitter, update_receiver) = mpsc::channel::<Change>();

        std::thread::spawn(move || {
            backend_loop(command_receiver, update_transmitter);
            println!("Backend thread exiting");
        });
        app.insert_resource(PlayerCommandSender(command_transmitter))
            .insert_resource(ChangeReceiver(Arc::new(Mutex::new(update_receiver))))
            .insert_resource(GridSize(600.0))
            .insert_resource(ScaleFactor(1.0))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(
    update_receiver: Res<ChangeReceiver>,
    mut query: Query<(&mut Transform, &Position, &Offset)>,
    commands: Commands,
    mut contexts: EguiContexts,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
) {
    if let Ok(receiver) = update_receiver.0.try_lock() {
        if let Ok(change) = receiver.try_recv() {
            let Change {
                init_game,
                landed_on_property,
                new_position,
                balance_change,
            } = change;

            println!("---------frontend message received (change): {:?}", change);

            if change.init_game == true {
                spawn_board(commands, grid_size, scale_factor);
            } else if change.landed_on_property == true {
                println!("++++++++++frontend_receiver got the message");
                // egui::Window::new("Buy or Auction?").show(contexts.ctx_mut(), |ui| {
                //     if ui.button("Buy").clicked() {
                //         println!("player chose to buy");
                //     } else if ui.button("Auction").clicked() {
                //         println!("player chose to auction");
                //     }
                // });
            }
        } else {
            println!("-no message available in channel");
        }
    } else {
        println!("Failed to acquire lock on channel");
    }
}
