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
        debug_helpers::{debug_property, debug_rent},
        prompts::prompt_player,
    },
};
use bevy::prelude::Resource;
use std::cell::RefCell;
use std::rc::Rc;

pub type PlayerRef = Rc<RefCell<Player>>;
pub type SpaceRef = Rc<RefCell<Space>>;

#[derive(Debug, Clone)]
pub enum RequiredInputsForFrontend {
    None,
    RollDice,
    Buy,
    Mortgage,
    Trade,
    BuyHouse,
    SellHouse,
}

// #[derive(Resource)]
// pub enum TurnOutcomeForFrontend {
//     BoardUpdated(BoardSnapshot),
//     InputRequiredForFrontend(RequiredInputsForFrontend, BoardSnapshot),
// }

#[derive(Resource)]
pub struct TurnOutcomeForFrontend {
    pub board_snapshot: BoardSnapshot,
    pub required_input: RequiredInputsForFrontend,
}

#[derive(Debug, Clone)]
pub struct BoardSnapshot {
    pub spaces: Vec<Space>,
    pub players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub spaces: Vec<SpaceRef>,
    pub players: Vec<PlayerRef>,
}

impl Board {
    pub fn player_turn(&mut self) -> TurnOutcomeForFrontend {
        let index = self.get_active_player_number() - 1;
        self.roll_player_dice(index);
        let player_ref = self.players[index].clone();
        let mut space_landed_on = self.spaces[self.get_position(index)].borrow_mut().clone();

        match &mut space_landed_on {
            Space::Property(properties) => {
                if properties.is_for_sale() {
                    debug_property(player_ref.borrow(), *properties);
                    // return TurnOutcomeForFrontend::InputRequiredForFrontend(
                    //     RequiredInputsForFrontend::Buy,
                    //     self.snapshot(),
                    // );

                    return TurnOutcomeForFrontend {
                        board_snapshot: self.snapshot(),
                        required_input: RequiredInputsForFrontend::Buy,
                    };

                    //     "buy" =>  properties.buy_property(player_ref.clone());
                    //         debug_buy_property(player_ref.borrow(), *properties);
                    //     "auction" => properties.auction(self);
                    //      println!("{:?} will be put up for auction.", properties);
                } else {
                    if let Some(owner) = properties.get_owner(self) {
                        properties.pay_rent(player_ref.clone(), self); // PAYING RENT HERE
                        debug_rent(owner.borrow(), player_ref.borrow());
                        // return board state for this and all matches below
                        println!("you paid rent");
                        // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                        return TurnOutcomeForFrontend {
                            board_snapshot: self.snapshot(),
                            required_input: RequiredInputsForFrontend::None,
                        };
                    } else {
                        println!("owner not found");
                        // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                        return TurnOutcomeForFrontend {
                            board_snapshot: self.snapshot(),
                            required_input: RequiredInputsForFrontend::None,
                        };
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
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::CommunityChest => {
                println!(
                    "Player {:?} has landed on Community Chest!",
                    player_ref.borrow().player_number
                );
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
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
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::LuxuryTax => {
                println!(
                    "Player {:?} landed on Luxury Tax!",
                    player_ref.borrow().player_number
                );
                player_ref.borrow_mut().money -= 100;
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::Go => {
                println!(
                    "Player {:?} has landed on GO!",
                    player_ref.borrow().player_number
                );
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::GoToJail => {
                println!(
                    "Player {:?} has landed on Go To Jail Bitch!",
                    player_ref.borrow().player_number
                );
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::Jail => {
                println!(
                    "Player {:?} has landed on Jail (just passing)",
                    player_ref.borrow().player_number
                );
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
            Space::FreeParking => {
                println!(
                    "Player {:?} has landed on Free Parking",
                    player_ref.borrow().player_number
                );
                // return TurnOutcomeForFrontend::BoardUpdated(self.snapshot());
                return TurnOutcomeForFrontend {
                    board_snapshot: self.snapshot(),
                    required_input: RequiredInputsForFrontend::None,
                };
            }
        }
        // player_ref.borrow_mut().active_player = false;
    }

    pub fn snapshot(&self) -> BoardSnapshot {
        BoardSnapshot {
            spaces: self
                .spaces
                .iter()
                .map(|space| space.borrow().clone())
                .collect(),
            players: self
                .players
                .iter()
                .map(|player| player.borrow().clone())
                .collect(),
        }
    }

    pub fn roll_player_dice(&self, index: usize) {
        let mut player = self.players[index].borrow_mut();
        player.roll_dice();
    }
    pub fn get_position(&self, index: usize) -> usize {
        let position = (self.players[index].borrow().position) as usize;
        println!("(Board.rs) get_position: {}", position);
        position
    }

    pub fn get_active_player_number(&self) -> usize {
        let active_player_number = self
            .players
            .iter()
            .find(|player| player.borrow().active_player)
            .map(|player| player.borrow().player_number as usize)
            .unwrap_or(0);
        println!("(Board.rs) active player: {}", active_player_number);
        active_player_number
    }

    pub fn pass_turn(&mut self) {
        let active_index = self.get_active_player_number() - 1;
        self.players[active_index].borrow_mut().active_player = false;
        if active_index + 1 >= self.players.len() {
            self.players[0].borrow_mut().active_player = true;
        } else {
            self.players[active_index + 1].borrow_mut().active_player = true;
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

        // set first player to active_player
        players[0].borrow_mut().active_player = true;

        Board { spaces, players }
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
            // "rd" => self.player_turn(index),
            _ => println!("Invalid Choice buddy"),
        }
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
