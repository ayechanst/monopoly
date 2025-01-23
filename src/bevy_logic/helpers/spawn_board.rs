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
    println!("about to spawn player");
    // player spawn
    let player_offset = [(0.25, 0.25), (-0.25, 0.25), (-0.25, -0.25), (0.25, -0.25)];
    for (i, &offset) in player_offset.iter().enumerate() {
        commands.spawn((
            Player,
            PlayerNumber(i as u32 + 1),
            Balance(1500),
            Position((5.0, -5.0)),
            Offset(offset),
            Properties(Vec::new()),
            make_player(i, grid_size.0, scale_factor.0),
        ));
        println!("done spawning player: {:?} ", i);
    }
    println!("done spawning players");
}
