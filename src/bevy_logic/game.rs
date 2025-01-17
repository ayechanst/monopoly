use crate::{models::board::Board, wait_for_player};
use bevy::prelude::*;

use super::world::{spawn_board, spawn_camera, spawn_light};

#[derive(Resource)]
struct RoundCount(u32);
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoundCount(0))
            .add_systems(Startup, (spawn_board, spawn_camera, spawn_light))
            .add_systems(Update, game_system);
    }
}

pub fn game_system(mut round_count: ResMut<RoundCount>) {
    let mut board = Board::new();
    let total_rounds = 5;
    // let mut round_count = 0;
    let num_of_players = 4;
    if round_count.0 < total_rounds {
        let current_player = round_count.0 % num_of_players;
        board.first_main_phase(current_player as usize);
        round_count.0 += 1;
    } else {
        println!("game over");
    }
    // loop {
    //     let current_player = round_count % num_of_players;
    //     board.first_main_phase(current_player);
    //     round_count += 1;
    //     // wait_for_player();
    // }
}
