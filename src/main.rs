mod bevy_logic;
mod models;
use bevy_egui::EguiPlugin;
use bevy_logic::{
    helpers::buttons::buttons,
    plugins::{frontend_plugin::FrontEndPlugin, world::WorldPlugin},
};
use std::io::{self, Write};
mod utils;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldPlugin)
        .add_systems(Update, buttons)
        .add_plugins(FrontEndPlugin)
        // .add_plugins(PlayerPlugin)
        .run();
}

fn wait_for_player() {
    print!("Press Enter to pass the turn... ");
    io::stdout().flush().unwrap(); // Ensures the message is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Waits for player input
}
