use crate::{
    bevy_logic::{player_components::FrontendPlayer, plugins::frontend_plugin::GridSize},
    models::{
        board::{RequiredInputsForFrontend, TurnOutcomeForFrontend},
        player::Player,
    },
    utils::space_to_coords::space_to_coords,
};
use bevy::prelude::*;

pub fn update_players(
    players: Vec<Player>,
    mut query: Query<(&mut FrontendPlayer, &mut Transform)>,
    grid_size: Res<GridSize>,
    required_input: RequiredInputsForFrontend,
) {
    for (mut frontend_player, mut transform) in query.iter_mut() {
        for player in players.iter() {
            if frontend_player.player_number == player.player_number {
                frontend_player.active_player = player.active_player;
                frontend_player.balance = player.money as u32;
                frontend_player.position = space_to_coords(player.position as usize);
                frontend_player.required_input = required_input.clone();
                // println!(
                //     "(update_player.rs) required_input: {:?}",
                //     required_input.clone()
                // );
                frontend_player.set_changed();

                let (x_coord, y_coord) = space_to_coords(player.position as usize);
                transform.translation = Vec3::new(
                    (x_coord + frontend_player.offset.0) * grid_size.0,
                    (y_coord + frontend_player.offset.1) * grid_size.0,
                    1.0,
                );
            }
        }
    }
}
