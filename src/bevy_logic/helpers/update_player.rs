use crate::{
    bevy_logic::{player_components::FrontendPlayer, plugins::frontend_plugin::GridSize},
    models::player::Player,
    utils::space_to_coords::space_to_coords,
};
use bevy::prelude::*;

pub fn update_player(
    players: Vec<Player>,
    mut query: Query<(&mut FrontendPlayer, &mut Transform)>,
    grid_size: Res<GridSize>,
) {
    for (mut frontend_player, mut transform) in query.iter_mut() {
        for player in players.iter() {
            if frontend_player.player_number == player.player_number {
                frontend_player.active_player = player.active_player;
                frontend_player.balance = player.money as u32;
                frontend_player.position = space_to_coords(player.position as usize);
                frontend_player.set_changed();

                let (x, y) = space_to_coords(player.position as usize);
                transform.translation = Vec3::new(x * grid_size.0, y * grid_size.0, 1.0);
            }
        }
    }
}
