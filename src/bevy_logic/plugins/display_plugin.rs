use crate::bevy_logic::board_and_player::Player;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_info);
    }
}

// this will be displayed from inside player_position?
pub fn player_info(mut contexts: EguiContexts, balances: Res<Player>) {
    egui::Window::new("Balances").show(contexts.ctx_mut(), |ui| {
        // for (player, balances) in  {
        //     ui.label(format!("{}: ${}", player, balances));
        // }
    });
}
