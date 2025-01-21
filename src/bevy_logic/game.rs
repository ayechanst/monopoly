use super::world::spawn_board;
use crate::utils::backend_loop::Change;
use bevy::prelude::*;
use std::sync::{mpsc::Receiver, Arc, Mutex};

#[derive(Resource)]
pub struct ChangeReceiver(pub Arc<Mutex<Receiver<Change>>>);

pub fn frontend_receiver(
    update_receiver: Res<ChangeReceiver>,
    // update_receiver: Receiver<Change>,
    mut query: Query<&mut Transform>,
    mut commands: Commands,
    // mut processed: Local<bool>,
) {
    println!("-frontend_receiver trying to receive message...");
    if let Ok(receiver) = update_receiver.0.try_lock() {
        // println!("-frontend (receiver): {:?}", receiver);
        if let Ok(change) = receiver.try_recv() {
            println!("---------frontend message received (change): {:?}", change);
            match change {
                Change::InitGame => {
                    println!("-about to spawn board");
                    spawn_board(commands);
                    println!("-board should have spawned");
                }
                _ => {}
            }
        } else {
            println!("-no message available in channel");
        }
    } else {
        println!("Failed to acquire lock on channel");
    }
    // if let Ok(change) = update_receiver.0.lock().unwrap().try_recv() {
    //     println!("before the match statement");
    //     match change {
    //         Change::InitGame => {
    //             println!("frontend reciever success");
    //             spawn_board(commands);
    //             println!("board should have spawned");
    //             // *processed = true;
    //         }
    //         Change::PositionChange => println!("beem"),
    //         Change::BalanceChange => println!("beem"),
    //         Change::PropertyStateChange => println!("beem"),
    //     }
    // }
}
