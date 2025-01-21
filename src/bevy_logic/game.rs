use crate::utils::backend_loop::Change;
use bevy::prelude::*;
use std::sync::{mpsc::Receiver, Arc, Mutex};

use super::world::spawn_board;

// #[derive(Resource)]
// struct RoundCount(u32);

// Wrap Change in a Receiver (to recieve it)
// Arc<Mutex<T>> just makes the Receiver thread safe
#[derive(Resource)]
pub struct ChangeReceiver(pub Arc<Mutex<Receiver<Change>>>);

// pub struct GamePlugin;

// impl Plugin for GamePlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(RoundCount(0))
//             .add_systems(Update, frontend_receiver);
//     }
// }

pub fn frontend_receiver(
    update_receiver: Res<ChangeReceiver>,
    // update_receiver: Receiver<Change>,
    mut query: Query<&mut Transform>,
    mut commands: Commands,
) {
    if let Ok(change) = update_receiver.0.lock().unwrap().try_recv() {
        // if let Ok(change) = update_receiver.try_recv() {
        match change {
            Change::InitGame => spawn_board(commands),
            Change::PositionChange => todo!(),
            Change::BalanceChange => todo!(),
            Change::PropertyStateChange => todo!(),
        }
    }
}
