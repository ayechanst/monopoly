use rand::Rng;

use crate::models::board::{Board, PlayerRef};

pub enum Chance {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
}

impl Chance {
    pub fn random_card() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=16) {
            1 => Chance::One,
            2 => Chance::Two,
            3 => Chance::Three,
            4 => Chance::Four,
            5 => Chance::Five,
            6 => Chance::Six,
            7 => Chance::Seven,
            8 => Chance::Eight,
            9 => Chance::Nine,
            10 => Chance::Ten,
            11 => Chance::Eleven,
            12 => Chance::Twelve,
            13 => Chance::Thirteen,
            14 => Chance::Fourteen,
            15 => Chance::Fifteen,
            16 => Chance::Sixteen,
            _ => unreachable!(),
        }
    }
    pub fn execute_card(&self, player_ref: PlayerRef, board: &Board) {
        let card = Self::random_card();
        let mut player = player_ref.borrow_mut();
        match card {
            Chance::One => {
                // Advance to Boardwalk
                player.position = 39;
            }
            Chance::Two => {
                // Advance to Go and collect $200
                player.position = 0;
            }
            Chance::Three => {
                // Advance to Illinois Ave
                player.position = 24;
            }
            Chance::Four => {
                // Advance to St. Charles Place
                player.position = 11;
            }
            Chance::Five => {
                // Nearest railroad
                let player_position = player.position;
                if player_position == 7 {
                    player.position = 15;
                } else if player.position == 22 {
                    player.position = 25;
                } else if player.position == 36 {
                    player.position = 5;
                } else {
                    println!("Failed to move to nearest Railroad :(");
                }
            }
            Chance::Six => {
                // Nearest railroad
                let player_position = player.position;
                if player_position == 7 {
                    player.position = 15;
                } else if player.position == 22 {
                    player.position = 25;
                } else if player.position == 36 {
                    player.position = 5;
                } else {
                    println!("Failed to move to nearest Railroad :(");
                }
            }
            Chance::Seven => {
                // Nearest Utility
                let player_position = player.position;
                if player_position == 7 || player.position == 36 {
                    player.position = 12;
                } else if player.position == 28 {
                    player.position = 25;
                } else {
                    println!("Failed to move to nearest Railroad :(");
                }
            }
            Chance::Eight => {
                // Bank pays you $50
                player.money += 50;
            }
            Chance::Nine => {
                println!("Get out of jail for free card!");
            }
            Chance::Ten => {
                // Move back 3 spaces
                player.position -= 3;
            }
            Chance::Eleven => {
                // Go to Jail, no $200
                player.position = 10;
            }
            Chance::Twelve => {
                // For each house pay $25, for each hotel pay $100
                println!("For each house pay $25, for each hotel pay $100");
            }
            Chance::Thirteen => {
                // Speeding fine
                player.money -= 15;
            }
            Chance::Fourteen => {
                // Reading Railroad
                player.position = 5;
            }
            Chance::Fifteen => {
                // Pay each player $50
                let players = &board.players;
                for other_player_ref in players.iter() {
                    let mut other_player = other_player_ref.borrow_mut();
                    other_player.money += 50;
                }
                player.money -= 200;
            }
            Chance::Sixteen => {
                // Collect $150
                player.money += 150;
            }
        }
    }
}
