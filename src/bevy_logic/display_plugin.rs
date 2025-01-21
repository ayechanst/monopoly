use super::board_and_player::Player;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// #[derive(Default, Resource)]
// pub struct Player {
//     pub balances: HashMap<String, u32>,
// }

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_info));
    }
}

pub fn player_info(mut contexts: EguiContexts, balances: Res<Player>) {
    egui::Window::new("Balances").show(contexts.ctx_mut(), |ui| {
        // for (player, balances) in  {
        //     ui.label(format!("{}: ${}", player, balances));
        // }
    });
}
