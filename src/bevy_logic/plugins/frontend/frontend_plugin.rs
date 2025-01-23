use crate::{
    bevy_logic::{
        // board_and_player::player_position,
        buttons::{Command, CommandSender},
        player_components::Position,
        plugins::frontend::{player_position::player_position, spawn_board::spawn_board},
    },
    utils::backend_loop::{backend_loop, Change},
};
use bevy::prelude::*;
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
        let (command_transmitter, command_receiver) = mpsc::channel::<Command>();
        let (update_transmitter, update_receiver) = mpsc::channel::<Change>();

        std::thread::spawn(move || {
            backend_loop(command_receiver, update_transmitter);
            println!("Backend thread exiting");
        });
        app.insert_resource(CommandSender(command_transmitter))
            .insert_resource(ChangeReceiver(Arc::new(Mutex::new(update_receiver))))
            .insert_resource(GridSize(600.0))
            .insert_resource(ScaleFactor(1.0))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(
    update_receiver: Res<ChangeReceiver>,
    mut query: Query<(&mut Transform, &Position)>,
    commands: Commands,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
) {
    if let Ok(receiver) = update_receiver.0.try_lock() {
        if let Ok(change) = receiver.try_recv() {
            println!("---------frontend message received (change): {:?}", change);
            match change {
                Change::InitGame => spawn_board(commands, grid_size, scale_factor),
                Change::PositionChange => player_position(query, grid_size),
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