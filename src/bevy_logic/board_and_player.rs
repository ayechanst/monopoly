use crate::{bevy_logic::sprite_builder::make_space::make_space, models::board::Board};
use bevy::prelude::*;

use super::sprite_builder::make_player::make_player;
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
    // let scale_factor = 1.0 / 4.0;
    let scale_factor = 1.0;
    let mut players = Vec::new();
    for i in 1..=4 {
        // let mut offset: (f32, f32) = (0.0, 0.0);
        let mut offset;
        if i == 1 {
            offset = (0.5, 0.5);
        } else if i == 2 {
            offset = (-0.5, 0.5)
        } else if i == 3 {
            offset = (-0.5, -0.5)
        } else {
            offset = (0.5, -0.5)
        }
        players.push(Player {
            number: i,
            balance: 1500,
            position: (5.0, -5.0),
            offset: offset,
            properties: Vec::new(),
        })
    }
    for player in players {
        let player_entity = make_player(&player, &mut commands, grid_size, scale_factor);
        let (x_offset, y_offset) = player.offset;
        commands.entity(player_entity).insert(Transform::from_xyz(
            (5.0 + x_offset) * grid_size,
            (-5.0 + y_offset) * grid_size,
            0.5,
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
