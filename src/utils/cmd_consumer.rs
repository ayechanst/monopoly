// This will recieve an enum, and call the correct method on the board struct

use crate::models::board::Board;

pub fn process_command(mut board: ResMut<Board>, command_rx: NonSend<Receiver<Command>>) {
    while let Ok(command) = command_rx.try_recv {
        match command {}
    }
}
