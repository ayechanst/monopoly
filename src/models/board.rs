use super::{
    player::Player,
    spaces::{
        properties::{
            colored_properties::{
                colored_properties::ColoredProperties, BlueProperty, BrownProperty, GreenProperty,
                LightBlueProperty, OrangeProperty, PinkProperty, RedProperty, YellowProperty,
            },
            properties::Properties,
            railroad::Railroads,
            utility::Utilities,
        },
        space::Space,
    },
};
use crate::{
    models::cards::chance::Chance,
    utils::{
        debug_helpers::{debug_buy_property, debug_property, debug_rent},
        prompts::prompt_player,
    },
};
use std::cell::RefCell;
use std::rc::Rc;

pub type PlayerRef = Rc<RefCell<Player>>;
pub type SpaceRef = Rc<RefCell<Space>>;

#[derive(Debug)]
pub struct Board {
    pub spaces: Vec<SpaceRef>,
    pub players: Vec<PlayerRef>,
}

impl Board {
    pub fn roll_player_dice(&self, index: usize) {
        let mut player = self.players[index].borrow_mut();
        player.roll_dice();
    }
    pub fn get_position(&self, index: usize) -> usize {
        (self.players[index].borrow().position) as usize
    }
    pub fn first_main_phase(&mut self, index: usize) {
        let choice =
            prompt_player("(trade/mortgage/buy-houses/sell-houses/roll-dice? (t/m/bh/sh/rd)");
        match choice.trim().to_lowercase().as_str() {
            "t" => todo!(),
            "m" => {
                let player_ref = self.players[index].clone();
                Player::mortgage(player_ref);
            }
            "bh" => {
                let player_ref = self.players[index].clone();
                Player::buy_house(player_ref, self);
            }
            "sh" => todo!(),
            "rd" => self.player_turn(index),
            _ => println!("Invalid Choice buddy"),
        }
    }
    pub fn player_turn(&mut self, index: usize) {
        self.roll_player_dice(index);
        let position = self.get_position(index);
        let player_ref = self.players[index].clone();
        let mut space_landed_on = self.spaces[position].borrow_mut();

        match &mut *space_landed_on {
            Space::Property(properties) => {
                if properties.is_for_sale() {
                    debug_property(player_ref.borrow(), *properties);
                    let choice = prompt_player("Buy or auction this property? (buy/auction)");
                    match choice.trim().to_lowercase().as_str() {
                        "buy" => {
                            properties.buy_property(player_ref.clone());
                            debug_buy_property(player_ref.borrow(), *properties);
                        }
                        "auction" => {
                            println!("{:?} will be put up for auction.", properties);
                            properties.auction(self);
                        }
                        _ => println!("Invalid choice buddy"),
                    }
                } else {
                    if let Some(owner) = properties.get_owner(self) {
                        properties.pay_rent(player_ref.clone(), self); // PAYING RENT HERE
                        debug_rent(owner.borrow(), player_ref.borrow());
                    } else {
                        println!("owner not found");
                    }
                }
            }
            Space::Chance => {
                let player_og_balance = player_ref.borrow().money;
                let chance_card = Chance::random_card();
                chance_card.execute_card(player_ref.clone(), self);
                let player_new_balance = player_ref.borrow().money;
                println!(
                    "Player {:?} has landed on Chance variant:{:?}.",
                    player_ref.borrow().player_number,
                    chance_card
                );
                println!("OG balance: {:?}", player_og_balance);
                println!("New balance: {:?}", player_new_balance);
            }
            Space::CommunityChest => {
                println!(
                    "Player {:?} has landed on Community Chest!",
                    player_ref.borrow().player_number
                )
            }
            Space::IncomeTax => {
                println!(
                    "Player {:?} landed on Income Tax!",
                    player_ref.borrow().player_number
                );
                let player_og_balance = player_ref.borrow().money;
                player_ref.borrow_mut().money -= 200;
                let player_new_balance = player_ref.borrow().money;
                println!("OG balance: {:?}", player_og_balance);
                println!("New balance: {:?}", player_new_balance);
            }
            Space::LuxuryTax => {
                println!(
                    "Player {:?} landed on Luxury Tax!",
                    player_ref.borrow().player_number
                );
                player_ref.borrow_mut().money -= 100;
            }
            Space::Go => {
                println!("Player {:?} has  pooped", player_ref.borrow().player_number)
            }
            Space::GoToJail => {
                println!(
                    "Player {:?} has landed on Go To Jail Bitch!",
                    player_ref.borrow().player_number
                )
            }
            Space::Jail => {
                println!(
                    "Player {:?} has landed on Jail (just passing)",
                    player_ref.borrow().player_number
                )
            }
            Space::FreeParking => {
                println!(
                    "Player {:?} has landed on Free Parking",
                    player_ref.borrow().player_number
                )
            }
        }
    }
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        // Bottom 0-9
        spaces.push(Rc::new(RefCell::new(Space::Go))); // 0
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Brown(
                BrownProperty::mediterranean_ave(),
            )),
        )))); // 1
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest))); // 2
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Brown(BrownProperty::baltic_ave())),
        )))); // 3
        spaces.push(Rc::new(RefCell::new(Space::IncomeTax))); // 4
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::reading()),
        )))); // 5
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::oriental_ave(),
            )),
        )))); // 6
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 7
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::vermont_ave(),
            )),
        )))); // 8
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::connecticut_ave(),
            )),
        )))); // 9
              // Left 10-19
        spaces.push(Rc::new(RefCell::new(Space::Jail))); // 10
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::st_charles_place())),
        )))); // 11
        spaces.push(Rc::new(RefCell::new(Space::Property(Properties::Utility(
            Utilities::electric_company(),
        ))))); // 12
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::states_ave())),
        )))); // 13
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::virginia_ave())),
        )))); // 14
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::pennsylvania()),
        )))); // 15
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Orange(OrangeProperty::st_james_place()),
            ),
        )))); // 16
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest))); // 17
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Orange(OrangeProperty::tennessee_ave())),
        )))); // 18
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Orange(OrangeProperty::new_york_ave())),
        )))); // 19
              // Top 20-29
        spaces.push(Rc::new(RefCell::new(Space::FreeParking))); // 20
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::kentucky_ave())),
        )))); // 21
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 22
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::indiana_ave())),
        )))); // 23
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::illinois_ave())),
        )))); // 24
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::bo()),
        )))); // 25
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Yellow(YellowProperty::atlantic_ave())),
        )))); // 26
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Yellow(YellowProperty::ventnor_ave())),
        )))); // 27
        spaces.push(Rc::new(RefCell::new(Space::Property(Properties::Utility(
            Utilities::water_works(),
        ))))); // 28
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Yellow(YellowProperty::marvin_gardens()),
            ),
        )))); // 29
              // Right 30-39
        spaces.push(Rc::new(RefCell::new(Space::GoToJail))); // 30
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Green(GreenProperty::pacific_ave())),
        )))); // 31
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Green(
                GreenProperty::north_carolina_ave(),
            )),
        )))); // 32
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest))); // 33
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Green(GreenProperty::pennsylvania_ave()),
            ),
        )))); // 34
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::shortline()),
        )))); // 35
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 36
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::park_place())),
        )))); // 37
        spaces.push(Rc::new(RefCell::new(Space::LuxuryTax))); // 38
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::boardwalk())),
        )))); // 39

        let players = vec![
            Rc::new(RefCell::new(Player::new(1))),
            Rc::new(RefCell::new(Player::new(2))),
            Rc::new(RefCell::new(Player::new(3))),
            Rc::new(RefCell::new(Player::new(4))),
        ];

        Board { spaces, players }
    }
}

// 22 properties
// 4 trains
// 2 utilities

// 3 chance
// 3 community chest
// 2 taxes (Income tax, Luxury tax)
// 1 free parking
// 1 go to jail
// 1 jail
// 1 Go
