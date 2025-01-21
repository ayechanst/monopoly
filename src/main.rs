mod bevy_logic;
mod models;
use bevy_logic::{
    buttons::{buttons, Command, CommandSender},
    game::{frontend_receiver, ChangeReceiver},
    world::WorldPlugin,
};
use std::{
    io::{self, Write},
    sync::{mpsc, Arc, Mutex},
};
use utils::backend_loop::{backend_loop, Change};
mod utils;
use bevy::prelude::*;
// TERMINOLOGY
// Res<T> is a type of `Resource`

fn main() {
    let (command_transmitter, command_receiver) = mpsc::channel::<Command>();
    let (update_transmitter, update_receiver) = mpsc::channel::<Change>();

    std::thread::spawn(move || {
        println!("Backend thread initialized");
        backend_loop(command_receiver, update_transmitter);
        println!("Backend thread exiting");
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .insert_resource(CommandSender(command_transmitter))
        .insert_resource(ChangeReceiver(Arc::new(Mutex::new(update_receiver))))
        .add_systems(Update, frontend_receiver)
        .run();
}

// pub fn game_system() {
//     let mut board = Board::new();
//     let mut round_count = 0;
//     let num_of_players = 4;
//     loop {
//         let current_player = round_count % num_of_players;
//         board.first_main_phase(current_player);
//         round_count += 1;
//         // wait_for_player();
//     }
// }

fn wait_for_player() {
    print!("Press Enter to pass the turn... ");
    io::stdout().flush().unwrap(); // Ensures the message is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Waits for player input
}
