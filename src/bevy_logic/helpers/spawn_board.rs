use crate::{
    bevy_logic::{
        player_components::{Balance, Offset, Player, PlayerNumber, Position, Properties},
        plugins::frontend_plugin::{GridSize, ScaleFactor},
        sprite_builder::{make_player::make_player, make_space::make_space},
    },
    models::board::Board,
};
use bevy::prelude::*;

pub fn spawn_board(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
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
    for (i, &offset) in player_offset.iter().enumerate() {
        let player_entity = commands
            .spawn((
                Player,
                PlayerNumber(i as u32 + 1),
                Balance(1500),
                Position((5.0, -5.0)),
                Offset(offset),
                Properties(Vec::new()),
                make_player(i + 1, grid_size.0, scale_factor.0),
            ))
            .id();
        let (x_offset, y_offset) = offset;
        commands.entity(player_entity).insert(Transform::from_xyz(
            (5.0 + x_offset) * grid_size.0,
            (-5.0 + y_offset) * grid_size.0,
            1.0,
        ));
        println!("done spawning player: {:?} ", i + 1);
        // TODO: Init the game (front end)
    }
    println!("done spawning players");

    // for each player, give them their game options:
}
