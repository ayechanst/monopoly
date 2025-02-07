mod bevy_logic;
mod models;
use bevy_egui::EguiPlugin;
use bevy_logic::{
    helpers::buttons::buttons,
    plugins::{frontend_plugin::FrontEndPlugin, world::WorldPlugin},
};
mod utils;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(FrontEndPlugin)
        .add_systems(Update, buttons)
        .run();
}
