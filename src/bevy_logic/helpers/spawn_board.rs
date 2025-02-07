use crate::models::board::RequiredInputsForFrontend;
use crate::{
    bevy_logic::{
        player_components::FrontendPlayer,
        plugins::frontend_plugin::{GridSize, ScaleFactor},
        sprite_builder::{make_player::make_player, make_space::make_space},
    },
    models::{board::Board, player::Player},
    utils::space_to_coords::space_to_coords,
};
use bevy::prelude::*;

pub fn spawn_board(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
    players: Vec<Player>,
) {
    let board = Board::new();
    for (index, space_ref) in board.spaces.iter().cloned().enumerate() {
        let i = index as f32;
        let (x, y) = if i < 10.0 {
            (5.0 - i, -5.0)
        } else if i < 20.0 {
            (-5.0, i - 15.0)
        } else if i < 30.0 {
            (i - 25.0, 5.0)
        } else {
            (5.0, -(i - 35.0))
        };
        let space_entity = make_space(
            space_ref,
            &mut commands,
            grid_size.0,
            scale_factor.0,
            (x, y),
        );
        println!("Entity: {:?} number: {:?}", space_entity, i);
        commands.entity(space_entity).insert(Transform::from_xyz(
            x * grid_size.0,
            y * grid_size.0,
            0.0,
        ));
    }
    let player_offset = [(0.25, 0.25), (-0.25, 0.25), (-0.25, -0.25), (0.25, -0.25)];
    for (i, player) in players.iter().enumerate() {
        let offset = player_offset[i % player_offset.len()];
        let frontend_player = FrontendPlayer {
            player_number: player.player_number,
            active_player: player.active_player,
            balance: player.money as u32,
            position: space_to_coords(player.position as usize),
            offset,
            properties: player.properties.iter().map(|p| p.to_string()).collect(),
            required_input: RequiredInputsForFrontend::None,
        };
        let player_entity = commands
            .spawn((
                frontend_player,
                make_player(player.player_number as usize, grid_size.0, scale_factor.0),
            ))
            .id();
        let (x_offset, y_offset) = offset;
        let (x_coord, y_coord) = space_to_coords(player.position as usize);
        println!(
            "player: {:?} X: {:?} Y: {:?} ",
            player.player_number, x_coord, y_coord
        );
        commands.entity(player_entity).insert(Transform::from_xyz(
            (x_coord + x_offset) * grid_size.0,
            (y_coord + y_offset) * grid_size.0,
            1.0,
        ));
        println!("done spawning player: {:?} ", player.player_number);
    }
}
