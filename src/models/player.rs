use crate::models::spaces::properties::properties::Properties;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub player_number: u8,
    pub money: i64,
    pub properties: Vec<Properties>,
    pub get_out_of_jail_cards: u8,
    pub player_state: PlayerState,
    pub position: i32,
    pub player_roll: i32,
}

impl Player {
    pub fn new(player_number: u8) -> Self {
        Player {
            player_number,
            money: 1500,
            properties: Vec::new(),
            get_out_of_jail_cards: 0,
            player_state: PlayerState::NotTheirTurn,
            position: 0,
            player_roll: 0,
        }
    }

    pub fn add_property(&mut self, bought_property: Properties) {
        // used on properties
        self.properties.push(bought_property);
    }

    pub fn dice_logic() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(1..=6), rng.gen_range(1..=6))
    }

    pub fn check_doubles(dice: (i32, i32)) -> bool {
        dice.0 == dice.1
    }

    pub fn movement(dice: (i32, i32), position: i32) -> i32 {
        let dice_total = dice.0 + dice.1;
        if position + dice_total >= 39 {
            (position - 40) + dice_total
        } else {
            position + dice_total
        }
    }

    // updates the player's true position
    pub fn roll_dice(&mut self) {
        let dice_roll = Self::dice_logic();
        let doubles = Self::check_doubles(dice_roll);
        let new_position = Self::movement(dice_roll, self.position);
        self.position = new_position;
    }

    // 1. Player rolls die
    // 2. Player moves
    // 3. Something checks what they landed on
    // 4. That brings up a set of options (buy, pay rent, or nothing)
    // 5. The player can trade
}

// payment logic, probably will be a trait shared with the bank
#[derive(Debug, PartialEq, Clone)]
pub enum PlayerState {
    NotTheirTurn,
    TheirTurn,
}
