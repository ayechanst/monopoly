use crate::{
    bevy_logic::player_components::FrontendPlayer, models::player::Player,
    utils::space_to_coords::space_to_coords,
};
use bevy::prelude::*;

pub fn update_player(players: Vec<Player>, mut query: Query<&mut FrontendPlayer>) {
    // pub fn update_player(players: Vec<Player>, query: &mut Query<(Entity, &mut FrontendPlayer)>) {
    // println!("Frontend query: {:?}", query);
    for mut frontend_player in query.iter_mut() {
        // println!("looping through query");
        for player in players.iter() {
            // println!("looping through players (back end players");
            if frontend_player.player_number == player.player_number {
                println!("matched frontend player to backend player");
                frontend_player.active_player = player.active_player;
                frontend_player.balance = player.money as u32;
                frontend_player.position = space_to_coords(player.position as usize);
                frontend_player.set_changed();
                // println!("**** frontend_player updated to: {:?}", frontend_player);
            }
        }
    }
}
