use crate::{bevy_logic::buttons::Command, models::board::Board};
use bevy::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

// pub fn process_command(mut board: ResMut<Board>, command_rx: NonSend<Receiver<Command>>) {
pub fn backend_loop(
    board: &mut Board,
    command_rx: NonSend<Receiver<Command>>,
    update_tx: Sender<Board>,
) {
    while let Ok(command) = command_rx.try_recv() {
        match command {
            Command::SpawnBoard => todo!(),
            Command::RollDice => todo!(),
            Command::Mortgage => todo!(),
            Command::Trade => todo!(),
            Command::BuyHouse => todo!(),
            Command::SellHouse => todo!(),
        }
        update_tx.send(board.clone()).unwrap()
    }
}
