use rand::Rng;

use crate::models::board::PlayerRef;

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
    pub fn execute_card(player_ref: PlayerRef) {
        let card = Self::random_card();
        match card {
            Chance::One => todo!(),
            Chance::Two => todo!(),
            Chance::Three => todo!(),
            Chance::Four => todo!(),
            Chance::Five => todo!(),
            Chance::Six => todo!(),
            Chance::Seven => todo!(),
            Chance::Eight => todo!(),
            Chance::Nine => todo!(),
            Chance::Ten => todo!(),
            Chance::Eleven => todo!(),
            Chance::Twelve => todo!(),
            Chance::Thirteen => todo!(),
            Chance::Fourteen => todo!(),
            Chance::Fifteen => todo!(),
            Chance::Sixteen => todo!(),
        }
    }
}

// Needs to take in a player struct, board struct
