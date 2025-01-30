use crate::{
    bevy_logic::{
        helpers::buttons::{PlayerCommand, PlayerCommandSender},
        player_components::{Offset, Position},
    },
    models::board::TurnOutcomeForFrontend,
    utils::backend_loop::{backend_loop, TurnOutcomeReceiver},
};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use std::sync::{
    mpsc::{self},
    Arc, Mutex,
};

#[derive(Resource)]
pub struct GridSize(pub f32);

#[derive(Resource)]
pub struct ScaleFactor(pub f32);

pub struct FrontEndPlugin;

impl Plugin for FrontEndPlugin {
    fn build(&self, app: &mut App) {
        let (command_transmitter, command_receiver) = mpsc::channel::<PlayerCommand>();
        // let (update_transmitter, update_receiver) = mpsc::channel::<Change>();
        let (update_transmitter, update_receiver) = mpsc::channel::<TurnOutcomeForFrontend>();

        std::thread::spawn(move || {
            backend_loop(command_receiver, update_transmitter);
            println!("Backend thread exiting");
        });
        app.insert_resource(PlayerCommandSender(command_transmitter))
            // .insert_resource(ChangeReceiver(Arc::new(Mutex::new(update_receiver))))
            // TODO: Figure out how to insert an enum resource
            // .insert_resource(TurnOutcomeForFrontend(update_receiver))
            .insert_resource(TurnOutcomeReceiver(Arc::new(Mutex::new(update_receiver))))
            .insert_resource(GridSize(600.0))
            .insert_resource(ScaleFactor(1.0))
            .add_systems(Update, frontend_receiver);
    }
}

pub fn frontend_receiver(
    // update_receiver: Res<ChangeReceiver>,
    update_receiver: Res<TurnOutcomeReceiver>,
    mut query: Query<(&mut Transform, &Position, &Offset)>,
    commands: Commands,
    mut contexts: EguiContexts,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
) {
    if let Ok(receiver) = update_receiver.0.try_lock() {
        if let Ok(turn_outcome) = receiver.try_recv() {
            // let Change {
            //     init_game,
            //     landed_on_property,
            //     new_position,
            //     balance_change,
            // } = change;

            match turn_outcome {
                TurnOutcomeForFrontend::BoardUpdated(board_snapshot) => {
                    println!(
                        "---------frontend message received board_snapshot: {:?}",
                        board_snapshot
                    );
                }
                TurnOutcomeForFrontend::InputRequiredForFrontend(required_inputs_for_frontend) => {
                    println!(
                        "-------------- INPUT REQUIRED: {:?}",
                        required_inputs_for_frontend
                    );
                }
            }
        } else {
            println!("-no message available in channel");
        }
    } else {
        println!("Failed to acquire lock on channel");
    }
}
