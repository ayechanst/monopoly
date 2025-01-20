mod bevy_logic;
mod models;
use bevy_logic::world::WorldPlugin;
use models::board::Board;
use std::{
    io::{self, Write},
    sync::mpsc,
};
use utils::backend_loop::backend_loop;
mod utils;
use bevy::prelude::*;
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

fn main() {
    // channel: frontend => backend (sends commands)
    let (command_tx, command_rx) = mpsc::channel::<Command>();
    // channel: backend => frontend (sends board)
    let (update_tx, update_rx) = mpsc::channel::<Board>();

    std::thread::spawn(move || {
        let mut board = Board::new();
        backend_loop(&mut board, command_rx, update_tx);
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
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
