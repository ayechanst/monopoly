use crate::{
    bevy_logic::player_components::FrontendPlayer, models::player::Player,
    utils::space_to_coords::space_to_coords,
};
use bevy::prelude::*;

// pub fn update_player(players: Vec<Player>, player_component: FrontendPlayer) {}
pub fn update_player(players: Vec<Player>, mut query: Query<&mut FrontendPlayer>) {
    for mut frontend_player in query.iter_mut() {
        for player in players.iter() {
            if frontend_player.player_number == player.player_number {
                frontend_player.active_player = player.active_player;
                frontend_player.balance = player.money as u32;
                frontend_player.position = space_to_coords(player.position as usize);
            }
        }
    }
}
