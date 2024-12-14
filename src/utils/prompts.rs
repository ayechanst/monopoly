use crate::models::player::Player;
use std::cell::Ref;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn prompt_player(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // makes the prompt display immidiately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn bid(player: Ref<'_, Player>, bid: i32) -> String {
    let prompt = format!("Player {:?}, buy for ${:?}?", player.player_number, bid);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let choice = prompt_player(&prompt);
        let _ = tx.send(choice);
    });
    match rx.recv_timeout(Duration::from_secs(3)) {
        Ok(choice) => match choice.trim().to_lowercase().as_str() {
            "y" => "y".to_string(),
            "n" => "n".to_string(),
            _ => "n".to_string(),
        },
        Err(_) => "n".to_string(),
    }
}
