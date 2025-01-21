use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::utils::backend_loop::{Change, ChangeSender};
use bevy::prelude::*;

#[derive(Resource)]
struct RoundCount(u32);

// Wrap Change in a Receiver (to recieve it)
// Arc<Mutex<T>> just makes the Receiver thread safe
#[derive(Resource)]
pub struct ChangeReceiver(pub Arc<Mutex<Receiver<Change>>>);
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RoundCount(0))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(update_receiver: Res<ChangeReceiver>, mut query: Query<&mut Transform>) {
    if let Ok(change) = update_receiver.0.lock().unwrap().try_recv() {
        match change {
            Change::InitGame => todo!(),
            Change::PositionChange => todo!(),
            Change::BalanceChange => todo!(),
            Change::PropertyStateChange => todo!(),
        }
    }
}
