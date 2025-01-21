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
    mut processed: Local<bool>,
) {
    println!("frontend_reciever is about to trigger");
    if let Ok(change) = update_receiver.0.lock().unwrap().try_recv() {
        if *processed {
            return;
        }
        // if let Ok(change) = update_receiver.0.lock().unwrap().recv() {
        match change {
            Change::InitGame => {
                println!("frontend reciever success");
                spawn_board(commands);
                println!("board should have spawned");
            }
            Change::PositionChange => todo!(),
            Change::BalanceChange => todo!(),
            Change::PropertyStateChange => todo!(),
        }
    }
}
