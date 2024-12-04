use super::spaces::Properties;
// use rand::prelude::*;
use rand::Rng;

#[derive(Debug)]
pub struct Player {
    player_number: u8,
    money: i64,
    properties: Vec<Properties>,
    get_out_of_jail_cards: u8,
    player_state: PlayerState,
    player_position: u8,
    player_roll: i32,
}

impl Player {
    pub fn new(player_number: u8) -> Self {
        Player {
            player_number,
            money: 1500,
            properties: Vec::new(),
            get_out_of_jail_cards: 0,
            player_state: PlayerState::NotTheirTurn,
            player_position: 0,
            player_roll: 0,
        }
    }
    pub fn roll_dice(&mut self) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let die1 = rng.gen_range(1..=6);
        let die2 = rng.gen_range(1..=6);
        self.player_roll = die1 + die2;
        (die1, die2)
    }
}

#[derive(Debug)]
pub enum PlayerState {
    NotTheirTurn,
    TheirTurn,
}
