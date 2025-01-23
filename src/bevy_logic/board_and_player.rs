use crate::{bevy_logic::sprite_builder::make_space::make_space, models::board::Board};
use bevy::prelude::*;

use super::{
    player_components::{Balance, Offset, PlayerNumber, Position, Properties},
    sprite_builder::make_player::make_player,
};
// put spawn_board and spawn_player in another file
// a player Resource (that can be updated) struct will be the data structure that
// moves/spawns the player and what the ui will read from

#[derive(Default, Resource)]
pub struct Player {
    pub number: u32,
    pub balance: u32,
    pub position: (f32, f32),
    pub offset: (f32, f32),
    pub properties: Vec<String>,
}

pub fn player_position(mut commands: Commands) {
    let grid_size = 600.0;
    let scale_factor = 1.0;
    let player_offset = [(0.25, 0.25), (-0.25, 0.25), (-0.25, -0.25), (0.25, -0.25)];
    for (i, &offset) in player_offset.iter().enumerate() {
        let player = Player {
            number: i as u32 + 1,
            balance: 1500,
            position: (5.0, -5.0),
            offset,
            properties: Vec::new(),
        };
        let player_entity = make_player(&player, &mut commands, grid_size, scale_factor);
        commands.entity(player_entity).insert((
            PlayerNumber(i as u32 + 1),
            Balance(1500),
            Position((5.0, -5.0)),
            Offset(offset),
            Properties(Vec::new()),
            Transform::from_xyz(
                (offset.0 + 5.0) * grid_size,
                (offset.1 - 5.0) * grid_size,
                0.5,
            ),
            GlobalTransform::default(),
        ));
    }
}

pub fn spawn_board(mut commands: Commands) {
    let board = Board::new();
    let grid_size = 600.0;
    let scale_factor = 1.0;
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
        let space_entity = make_space(space_ref, &mut commands, grid_size, scale_factor, (x, y));
        println!("Entity: {:?} number: {:?}", space_entity, i);
        commands.entity(space_entity).insert(Transform::from_xyz(
            x * grid_size,
            y * grid_size,
            0.0,
        ));
    }
}
