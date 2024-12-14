use crate::{models::spaces::properties::properties::Properties, utils::prompts::prompt_player};
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub player_number: u8,
    pub money: i32,
    pub properties: Vec<Properties>,
    pub get_out_of_jail_cards: u8,
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
            position: 0,
            player_roll: 0,
        }
    }

    pub fn add_property(&mut self, bought_property: Properties) {
        self.properties.push(bought_property);
    }
    pub fn mortgage(&mut self) {
        // property cannot have houses if they want to mortgage
        let properties = self.properties.clone();
        for property in properties.iter() {
            let prompt = format!("Would you like to mortgage {:?}? (y/n)", property);
            let choice = prompt_player(&prompt);
            match choice.trim().to_lowercase().as_str() {
                "y" => todo!(), // property.mortgage()
                "n" => println!("{:?} will not be mortgaged.", property),
                _ => println!("Not valid input"),
            }
        }
    }

    pub fn dice_logic() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(1..=6), rng.gen_range(1..=6))
    }

    pub fn check_doubles(dice: (i32, i32)) -> bool {
        dice.0 == dice.1
    }

    // pub fn movement(dice: (i32, i32), position: i32) -> i32 {
    pub fn movement(&mut self, dice: (i32, i32), position: i32) -> i32 {
        let dice_total = dice.0 + dice.1;
        if position + dice_total >= 39 {
            self.money += 200;
            (position - 40) + dice_total
        } else {
            position + dice_total
        }
    }

    // updates the player's true position
    pub fn roll_dice(&mut self) {
        let dice_roll = Self::dice_logic();
        // let new_position = Self::movement(dice_roll, self.position);
        let new_position = self.movement(dice_roll, self.position);
        self.position = new_position;
    }
}
