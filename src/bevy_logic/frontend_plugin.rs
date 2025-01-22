use super::{
    buttons::{Command, CommandSender},
    // world::spawn_board,
};
use crate::{
    bevy_logic::board_and_player::{player_position, spawn_board},
    utils::backend_loop::{backend_loop, Change},
};
use bevy::prelude::*;
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

#[derive(Resource)]
pub struct ChangeReceiver(pub Arc<Mutex<Receiver<Change>>>);

pub struct FrontEndPlugin;

impl Plugin for FrontEndPlugin {
    fn build(&self, app: &mut App) {
        let (command_transmitter, command_receiver) = mpsc::channel::<Command>();
        let (update_transmitter, update_receiver) = mpsc::channel::<Change>();

        std::thread::spawn(move || {
            backend_loop(command_receiver, update_transmitter);
            println!("Backend thread exiting");
        });
        app.insert_resource(CommandSender(command_transmitter))
            .insert_resource(ChangeReceiver(Arc::new(Mutex::new(update_receiver))))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(
    update_receiver: Res<ChangeReceiver>,
    mut query: Query<&mut Transform>,
    // mut commands: Commands,
    commands: Commands,
) {
    if let Ok(receiver) = update_receiver.0.try_lock() {
        if let Ok(change) = receiver.try_recv() {
            println!("---------frontend message received (change): {:?}", change);
            match change {
                Change::InitGame => {
                    spawn_board(commands);
                    // player_position(commands);
                }
                Change::PositionChange => player_position(commands),
                Change::BalanceChange => println!("beem"),
                Change::PropertyStateChange => println!("beem"),
            }
        } else {
            println!("-no message available in channel");
        }
    } else {
        println!("Failed to acquire lock on channel");
    }
}
