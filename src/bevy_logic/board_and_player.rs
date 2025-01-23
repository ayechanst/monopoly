use bevy::prelude::*;

use super::{
    player_components::{Balance, Offset, Player, PlayerNumber, Position, Properties},
    sprite_builder::make_player::make_player,
};
// put spawn_board and spawn_player in another file
// a player Resource (that can be updated) struct will be the data structure that
// moves/spawns the player and what the ui will read from

// pub fn player_position(mut commands: Commands) {
//     let grid_size = 600.0;
//     let scale_factor = 1.0;
//     let player_offset = [(0.25, 0.25), (-0.25, 0.25), (-0.25, -0.25), (0.25, -0.25)];
//     for (i, &offset) in player_offset.iter().enumerate() {
//         let player = Player {
//             number: i as u32 + 1,
//             balance: 1500,
//             position: (5.0, -5.0),
//             offset,
//             properties: Vec::new(),
//         };
//         let player_entity = make_player(&player, &mut commands, grid_size, scale_factor);
//         commands.entity(player_entity).insert((
//             PlayerNumber(i as u32 + 1),
//             Balance(1500),
//             Position((5.0, -5.0)),
//             Offset(offset),
//             Properties(Vec::new()),
//             Transform::from_xyz(
//                 (offset.0 + 5.0) * grid_size,
//                 (offset.1 - 5.0) * grid_size,
//                 0.5,
//             ),
//             GlobalTransform::default(),
//         ));
//     }
// }
