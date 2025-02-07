use std::{cell::RefCell, rc::Rc};

use crate::{models::spaces::properties::properties::Properties, utils::prompts::prompt_player};
use rand::Rng;

use super::board::Board;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub player_number: u8,
    pub active_player: bool,
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
            active_player: false,
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
    pub fn mortgage(self_ref: Rc<RefCell<Self>>) {
        let mut properties = self_ref.borrow_mut().properties.clone();
        let balance_before_mortgage = self_ref.borrow().money;
        for property in properties.iter_mut() {
            let prompt = format!("Would you like to mortgage {:?}? (y/n)", property);
            let choice = prompt_player(&prompt);
            match choice.trim().to_lowercase().as_str() {
                "y" => {
                    property.mortgage(self_ref.clone());
                    let balance_after_mortgage = self_ref.borrow().money;
                    println!(
                        "Money before mortgage: {:?}, and money after: {:?}",
                        balance_before_mortgage, balance_after_mortgage
                    );
                }
                "n" => println!("{:?} will not be mortgaged.", property),
                _ => println!("Not valid input"),
            }
        }
    }
    pub fn buy_house(self_ref: Rc<RefCell<Self>>, board: &Board) {
        let mut properties = self_ref.borrow_mut().properties.clone();
        let balance_before_mortgage = self_ref.borrow().money;
        for property in properties.iter_mut() {
            let prompt = format!("Would you like to buy a house for {:?}? (y/n)", property);
            let choice = prompt_player(&prompt);
            match choice.trim().to_lowercase().as_str() {
                "y" => {
                    // property.mortgage(self_ref.clone());
                    property.buy_house(self_ref.clone(), board);
                    let balance_after_mortgage = self_ref.borrow().money;
                    println!(
                        "Money before house: {:?}, and money after: {:?}",
                        balance_before_mortgage, balance_after_mortgage
                    );
                }
                "n" => println!("No houses will be bought for {:?}.", property),
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

    // updates the player's true position
    pub fn roll_dice(&mut self) {
        let dice_roll = Self::dice_logic();
        // let new_position = Self::movement(dice_roll, self.position);
        // let new_position = self.movement(dice_roll, self.position);
        self.position = self.movement(dice_roll, self.position);
    }

    // pub fn movement(dice: (i32, i32), position: i32) -> i32 {
    pub fn movement(&mut self, dice: (i32, i32), position: i32) -> i32 {
        let dice_total = dice.0 + dice.1;
        let new_position = (position + dice_total) % 40;
        if new_position < position {
            self.money += 200;
        }
        new_position
        // if position + dice_total >= 39 {
        //     self.money += 200;
        //     (position - 40) + dice_total
        // } else {
        //     position + dice_total
        // }
    }
}
