use crate::{models::board::Board, Command};
use bevy::prelude::Resource;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Resource)]
pub struct UpdateMessageSender(pub Sender<UpdateMessage>);
// #[derive(Clone)]
pub enum UpdateMessage {
    PositionChange,
    BalanceChange,
    PropertyStateChange,
}

// pub fn backend_loop(board: &mut Board, command_rx: Receiver<Command>, update_tx: Sender<Board>) {
pub fn backend_loop(command_rx: Receiver<Command>, update_tx: Sender<UpdateMessage>) {
    let mut board = Board::new();
    while let Ok(command) = command_rx.try_recv() {
        match command {
            Command::SpawnBoard => println!("boom"),
            Command::RollDice => println!("boom"),
            Command::Mortgage => println!("boom"),
            Command::Trade => println!("boom"),
            Command::BuyHouse => println!("boom"),
            Command::SellHouse => println!("boom"),
        }
        // update_tx.send(board.clone()).unwrap()

        // send this within each command match for example:
        update_tx.send(UpdateMessage::BalanceChange).unwrap();
    }
}
