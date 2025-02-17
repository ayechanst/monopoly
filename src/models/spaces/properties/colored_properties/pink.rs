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
// #[derive(Debug, PartialEq, Clone)]
pub enum PinkProperty {
    StCharlesPlace { state: PropertyState },
    VermontAve { state: PropertyState },
    ConnecticutAve { state: PropertyState },
}

impl PinkProperty {
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            PinkProperty::ConnecticutAve { .. } => 120,
            _ => 100,
        };
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        // match self {
        //     PinkProperty::StCharlesPlace { state }
        //     | PinkProperty::VermontAve { state }
        //     | PinkProperty::ConnecticutAve { state } => *state = PropertyState::Owned,
        // }
        // player.add_property(Properties::ColoredProperty(ColoredProperties::Pink(*self)));
        player.add_property(Properties::ColoredProperty(ColoredProperties::Pink(
            self.clone(),
        )));
        self.update_property_state(player, PropertyState::Owned);
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
                    PinkProperty::StCharlesPlace { state }
                    | PinkProperty::VermontAve { state }
                    | PinkProperty::ConnecticutAve { state } => {
                        *state = PropertyState::Owned;
                    }
                }
                winner.add_property(Properties::ColoredProperty(ColoredProperties::Pink(*self)));
                // winner.add_property(Properties::ColoredProperty(ColoredProperties::Pink(
                //     self.clone(),
                // )));
                println!(
                    "Player {:?} has aquired {:?} for ${:?}",
                    winner.player_number,
                    ColoredProperties::Pink(*self),
                    // ColoredProperties::Pink(self.clone()),
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
            PinkProperty::ConnecticutAve { .. } => 60,
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
            PinkProperty::StCharlesPlace { state }
            | PinkProperty::VermontAve { state }
            | PinkProperty::ConnecticutAve { state } => match state {
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
            PinkProperty::StCharlesPlace { state }
            | PinkProperty::VermontAve { state }
            | PinkProperty::ConnecticutAve { state } => {
                *state = new_state;
            }
        }
        if let Some(property) = player.properties.iter_mut().find(|p| match p {
            Properties::ColoredProperty(ColoredProperties::Pink(inner)) => inner == self,
            _ => false,
        }) {
            if let Properties::ColoredProperty(ColoredProperties::Pink(inner)) = property {
                match inner {
                    PinkProperty::StCharlesPlace { state }
                    | PinkProperty::VermontAve { state }
                    | PinkProperty::ConnecticutAve { state } => {
                        *state = new_state;
                    }
                }
            }
        }
    }
    pub fn legal_house_purchase(&self, board: &Board) -> bool {
        let (st_charles_place, states_ave, virginia_ave) = self.house_count(board);
        if vec![st_charles_place, states_ave, virginia_ave]
            .windows(2)
            .all(|window| (window[0] as i8 - window[1] as i8).abs() <= 1)
        {
            match self {
                PinkProperty::StCharlesPlace { state } => {
                    st_charles_place > states_ave && st_charles_place > virginia_ave
                }
                PinkProperty::VermontAve { state } => {
                    states_ave > st_charles_place && states_ave > virginia_ave
                }
                PinkProperty::ConnecticutAve { state } => {
                    virginia_ave > st_charles_place && virginia_ave > states_ave
                }
            }
        } else {
            false
        }
    }
    pub fn house_count(&self, board: &Board) -> (u8, u8, u8) {
        if self.has_monopoly(board) {
            let st_charles_place = match self {
                PinkProperty::StCharlesPlace { state } => match state {
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
            let states_ave = match self {
                PinkProperty::VermontAve { state } => match state {
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
            let virginia_ave = match self {
                PinkProperty::ConnecticutAve { state } => match state {
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
            (st_charles_place, states_ave, virginia_ave)
        } else {
            (0, 0, 0)
        }
    }
    pub fn has_monopoly(&self, board: &Board) -> bool {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            let monopoly = vec![
                Properties::ColoredProperty(ColoredProperties::Pink(
                    PinkProperty::StCharlesPlace {
                        state: PropertyState::Owned,
                    },
                )),
                Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::VermontAve {
                    state: PropertyState::Owned,
                })),
                Properties::ColoredProperty(ColoredProperties::Pink(
                    PinkProperty::ConnecticutAve {
                        state: PropertyState::Owned,
                    },
                )),
            ];
            monopoly.iter().all(|property| {
                if owner.properties.contains(property) {
                    match property {
                        Properties::ColoredProperty(ColoredProperties::Pink(prop)) => match prop {
                            PinkProperty::StCharlesPlace { state }
                            | PinkProperty::VermontAve { state }
                            | PinkProperty::ConnecticutAve { state } => {
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
            PinkProperty::StCharlesPlace { state } => match state {
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
            PinkProperty::VermontAve { state } => match state {
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
            PinkProperty::ConnecticutAve { state } => match state {
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
                    PinkProperty::StCharlesPlace { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    PinkProperty::VermontAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    PinkProperty::ConnecticutAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
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
            PinkProperty::StCharlesPlace { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::VermontAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::ConnecticutAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn st_charles_place() -> Self {
        PinkProperty::StCharlesPlace {
            state: PropertyState::ForSale,
        }
    }
    pub fn states_ave() -> Self {
        PinkProperty::VermontAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn virginia_ave() -> Self {
        PinkProperty::ConnecticutAve {
            state: PropertyState::ForSale,
        }
    }
}
