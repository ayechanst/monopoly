mod bevy_logic;
mod models;
use bevy_egui::EguiPlugin;
use bevy_logic::world::WorldPlugin;
// use bevy_logic::{game::GamePlugin, world::WorldPlugin};
use models::board::Board;
use std::{
    io::{self, Write},
    sync::mpsc,
};
use utils::cmd_consumer::backend_loop;
mod utils;
use bevy::prelude::*;
pub enum Command {
    SpawnBoard,
    RollDice,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

fn main() {
    // creates a channel for front end to communicate with backend
    // commands received from front-end
    let (command_tx, command_rx) = mpsc::channel::<Command>();
    // creates a channel for back end to communicate with frontend
    // data being sent back to front-end
    let (update_tx, update_rx) = mpsc::channel::<Command>();

    std::thread::spawn(move || {
        let mut board = Board::new();
        backend_loop(&mut board, command_rx, update_tx);
    });

    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldPlugin)
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
