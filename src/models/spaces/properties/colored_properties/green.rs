use std::cell::RefMut;

use super::colored_properties::ColoredProperties;
use crate::{
    models::{
        board::{Board, PlayerRef},
        player::Player,
        spaces::{
            properties::properties::Properties,
            space::{HouseCount, PropertyState},
        },
    },
    utils::prompts::bid,
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GreenProperty {
    PacificAve { state: PropertyState },
    NorthCarolinaAve { state: PropertyState },
    PennsylvaniaAve { state: PropertyState },
}

impl GreenProperty {
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            GreenProperty::PennsylvaniaAve { .. } => 120,
            _ => 100,
        };
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        match self {
            GreenProperty::PacificAve { state }
            | GreenProperty::NorthCarolinaAve { state }
            | GreenProperty::PennsylvaniaAve { state } => *state = PropertyState::Owned,
        }
        player.add_property(Properties::ColoredProperty(ColoredProperties::Green(*self)));
    }
    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price(board);
        owner.money += rent_price;
        renter.money -= rent_price;
    }
    pub fn auction(&mut self, board: &Board) {
        let player_refs = &board.players;
        let mut bid_price = 10; // Starting bid
        let mut round_bids = vec![false; player_refs.len()];
        let mut current_bidder_index = 0;
        let mut highest_bidder_index = current_bidder_index;
        let mut highest_bid = 0;
        let mut round_count = 0;
        loop {
            let player_ref = player_refs[current_bidder_index].clone();
            let choice = bid(player_ref.borrow(), bid_price);
            if choice.trim().to_lowercase() == "y" {
                if bid_price > highest_bid {
                    highest_bid = bid_price;
                    highest_bidder_index = current_bidder_index;
                }
            } else {
                round_bids[current_bidder_index] = true;
            }
            if round_bids.iter().all(|&b| b) {
                let mut winner = player_refs[highest_bidder_index].borrow_mut();
                winner.money -= highest_bid - 10;
                match self {
                    GreenProperty::PacificAve { state }
                    | GreenProperty::NorthCarolinaAve { state }
                    | GreenProperty::PennsylvaniaAve { state } => {
                        *state = PropertyState::Owned;
                    }
                }
                winner.add_property(Properties::ColoredProperty(ColoredProperties::Green(*self)));
                println!(
                    "Player {:?} has aquired {:?} for ${:?}",
                    winner.player_number,
                    ColoredProperties::Green(*self),
                    bid_price - 10
                );
                break;
            }
            current_bidder_index = (current_bidder_index + 1) % player_refs.len(); // Fix the bid_price increasing at the end
            round_count += 1;
            if round_count % 4 == 0 {
                bid_price += 10;
            }
        }
    }
    pub fn mortgage(&mut self, player_ref: PlayerRef) {
        // TODO: cant mortgage if a property has houses on it
        let mortgage_value = match self {
            GreenProperty::PennsylvaniaAve { .. } => 60,
            _ => 50,
        };
        let mut player = player_ref.borrow_mut();
        player.money += mortgage_value;
        self.update_property_state(player, PropertyState::Mortgaged);
    }
    pub fn buy_house(&mut self, player_ref: PlayerRef, board: &Board) {
        let house_cost = 50;
        let mut player = player_ref.borrow_mut();
        player.money -= house_cost;
        if !self.legal_house_purchase(board) {
            println!("You can't buy a house here yet");
        } else {
            return;
        }
        let new_state = match self {
            GreenProperty::PacificAve { state }
            | GreenProperty::NorthCarolinaAve { state }
            | GreenProperty::PennsylvaniaAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => PropertyState::Houses(HouseCount::One),
                    HouseCount::One => PropertyState::Houses(HouseCount::Two),
                    HouseCount::Two => PropertyState::Houses(HouseCount::Three),
                    HouseCount::Three => PropertyState::Houses(HouseCount::Four),
                    HouseCount::Four => PropertyState::Houses(HouseCount::Hotel),
                    HouseCount::Hotel => {
                        println!("Your property is already developed to the max!");
                        return;
                    }
                },
                _ => {
                    println!("You need to spread your houses out better to buy!");
                    return;
                }
            },
        };
        self.update_property_state(player, new_state);
    }
    pub fn update_property_state(
        &mut self,
        mut player: RefMut<'_, Player>,
        new_state: PropertyState,
    ) {
        match self {
            GreenProperty::PacificAve { state }
            | GreenProperty::NorthCarolinaAve { state }
            | GreenProperty::PennsylvaniaAve { state } => {
                *state = new_state;
            }
        }
        if let Some(property) = player.properties.iter_mut().find(|p| match p {
            Properties::ColoredProperty(ColoredProperties::Green(inner)) => inner == self,
            _ => false,
        }) {
            if let Properties::ColoredProperty(ColoredProperties::Green(inner)) = property {
                match inner {
                    GreenProperty::PacificAve { state }
                    | GreenProperty::NorthCarolinaAve { state }
                    | GreenProperty::PennsylvaniaAve { state } => {
                        *state = new_state;
                    }
                }
            }
        }
    }
    pub fn legal_house_purchase(&self, board: &Board) -> bool {
        let (pacific_ave, north_carolina_ave, pennsylvania_ave) = self.house_count(board);
        if vec![pacific_ave, north_carolina_ave, pennsylvania_ave]
            .windows(2)
            .all(|window| (window[0] as i8 - window[1] as i8).abs() <= 1)
        {
            match self {
                GreenProperty::PacificAve { state } => {
                    pacific_ave > north_carolina_ave && pacific_ave > pennsylvania_ave
                }
                GreenProperty::NorthCarolinaAve { state } => {
                    north_carolina_ave > pacific_ave && north_carolina_ave > pennsylvania_ave
                }
                GreenProperty::PennsylvaniaAve { state } => {
                    pennsylvania_ave > pacific_ave && pennsylvania_ave > north_carolina_ave
                }
            }
        } else {
            false
        }
    }
    pub fn house_count(&self, board: &Board) -> (u8, u8, u8) {
        if self.has_monopoly(board) {
            let pacific_ave = match self {
                GreenProperty::PacificAve { state } => match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => 0,
                        HouseCount::One => 1,
                        HouseCount::Two => 2,
                        HouseCount::Three => 3,
                        HouseCount::Four => 4,
                        HouseCount::Hotel => 5,
                    },
                    _ => 0,
                },
                _ => 0,
            };
            let north_carolina_ave = match self {
                GreenProperty::NorthCarolinaAve { state } => match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => 0,
                        HouseCount::One => 1,
                        HouseCount::Two => 2,
                        HouseCount::Three => 3,
                        HouseCount::Four => 4,
                        HouseCount::Hotel => 5,
                    },
                    _ => 0,
                },
                _ => 0,
            };
            let pennsylvania_ave = match self {
                GreenProperty::PennsylvaniaAve { state } => match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => 0,
                        HouseCount::One => 1,
                        HouseCount::Two => 2,
                        HouseCount::Three => 3,
                        HouseCount::Four => 4,
                        HouseCount::Hotel => 5,
                    },
                    _ => 0,
                },
                _ => 0,
            };
            (pacific_ave, north_carolina_ave, pennsylvania_ave)
        } else {
            (0, 0, 0)
        }
    }
    pub fn has_monopoly(&self, board: &Board) -> bool {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            let monopoly = vec![
                Properties::ColoredProperty(ColoredProperties::Green(GreenProperty::PacificAve {
                    state: PropertyState::Owned,
                })),
                Properties::ColoredProperty(ColoredProperties::Green(
                    GreenProperty::NorthCarolinaAve {
                        state: PropertyState::Owned,
                    },
                )),
                Properties::ColoredProperty(ColoredProperties::Green(
                    GreenProperty::PennsylvaniaAve {
                        state: PropertyState::Owned,
                    },
                )),
            ];
            monopoly.iter().all(|property| {
                if owner.properties.contains(property) {
                    match property {
                        Properties::ColoredProperty(ColoredProperties::Green(prop)) => match prop {
                            GreenProperty::PacificAve { state }
                            | GreenProperty::NorthCarolinaAve { state }
                            | GreenProperty::PennsylvaniaAve { state } => {
                                *state != PropertyState::ForSale
                            }
                        },
                        _ => true,
                    }
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
    pub fn rent_price(&self, board: &Board) -> i32 {
        let rent = 6;
        match self {
            GreenProperty::PacificAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            rent * 2
                        } else {
                            rent
                        }
                    }
                    HouseCount::One => 30,
                    HouseCount::Two => 90,
                    HouseCount::Three => 270,
                    HouseCount::Four => 400,
                    HouseCount::Hotel => 550,
                },
                _ => rent,
            },
            GreenProperty::NorthCarolinaAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            rent * 2
                        } else {
                            rent
                        }
                    }
                    HouseCount::One => 30,
                    HouseCount::Two => 90,
                    HouseCount::Three => 270,
                    HouseCount::Four => 400,
                    HouseCount::Hotel => 550,
                },
                _ => rent,
            },
            GreenProperty::PennsylvaniaAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            rent * 2
                        } else {
                            rent
                        }
                    }
                    HouseCount::One => 40,
                    HouseCount::Two => 100,
                    HouseCount::Three => 300,
                    HouseCount::Four => 450,
                    HouseCount::Hotel => 600,
                },
                _ => rent,
            },
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        for player in players.iter() {
            let player_ref = player.borrow();
            let properties = player_ref.properties.clone();
            for property in properties.iter() {
                match self {
                    GreenProperty::PacificAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Green(prop)) =
                            property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    GreenProperty::NorthCarolinaAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Green(prop)) =
                            property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    GreenProperty::PennsylvaniaAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Green(prop)) =
                            property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn for_sale(&self) -> bool {
        match self {
            GreenProperty::PacificAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            GreenProperty::NorthCarolinaAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            GreenProperty::PennsylvaniaAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn pacific_ave() -> Self {
        GreenProperty::PacificAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn north_carolina_ave() -> Self {
        GreenProperty::NorthCarolinaAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn pennsylvania_ave() -> Self {
        GreenProperty::PennsylvaniaAve {
            state: PropertyState::ForSale,
        }
    }
}
