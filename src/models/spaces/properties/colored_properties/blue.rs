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
use std::cell::RefMut;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BlueProperty {
    ParkPlace { state: PropertyState },
    Boardwalk { state: PropertyState },
}

impl BlueProperty {
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            BlueProperty::ParkPlace { .. } => 350,
            BlueProperty::Boardwalk { .. } => 400,
            // _ => 100,
        };
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        // match self {
        //     BlueProperty::ParkPlace { state }
        //     | BlueProperty::Boardwalk { state }
        //     // | BlueProperty::IllinoisAve { state } => *state = PropertyState::Owned,
        //      => *state = PropertyState::Owned,
        // }
        // self.update_property_state(
        //     RefMut::map(player_ref.borrow_mut(), |p| p),
        //     PropertyState::Owned,
        // );
        player.add_property(Properties::ColoredProperty(ColoredProperties::Blue(*self)));
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
                    BlueProperty::ParkPlace { state }
                    | BlueProperty::Boardwalk { state }
                    // | BlueProperty::IllinoisAve { state } => {
                     => {
                        *state = PropertyState::Owned;
                    }
                }
                winner.add_property(Properties::ColoredProperty(ColoredProperties::Blue(*self)));
                println!(
                    "Player {:?} has aquired {:?} for ${:?}",
                    winner.player_number,
                    ColoredProperties::Blue(*self),
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
            // BlueProperty::IllinoisAve { .. } => 60,
            BlueProperty::ParkPlace { .. } => 350 / 2,
            BlueProperty::Boardwalk { .. } => 200,
            // _ => 50,
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
            BlueProperty::ParkPlace { state }
            | BlueProperty::Boardwalk { state }
            // | BlueProperty::IllinoisAve { state } => match state {
            => match state {
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
            BlueProperty::ParkPlace { state }
            | BlueProperty::Boardwalk { state }
            // | BlueProperty::IllinoisAve { state } => {
            => {
                *state = new_state;
            }
        }
        if let Some(property) = player.properties.iter_mut().find(|p| match p {
            Properties::ColoredProperty(ColoredProperties::Blue(inner)) => inner == self,
            _ => false,
        }) {
            if let Properties::ColoredProperty(ColoredProperties::Blue(inner)) = property {
                match inner {
                    BlueProperty::ParkPlace { state }
                    | BlueProperty::Boardwalk { state }
                    // | BlueProperty::IllinoisAve { state } => {
                    => {
                        *state = new_state;
                    }
                }
            }
        }
    }
    pub fn legal_house_purchase(&self, board: &Board) -> bool {
        let (park_place, boardwalk) = self.house_count(board);
        if vec![park_place, boardwalk]
            .windows(2)
            .all(|window| (window[0] as i8 - window[1] as i8).abs() <= 1)
        {
            match self {
                BlueProperty::ParkPlace { state } => {
                    // park_place > boardwalk && park_place > illinois_ave
                    park_place > boardwalk
                }
                BlueProperty::Boardwalk { state } => {
                    // boardwalk > park_place && boardwalk > illinois_ave
                    boardwalk > park_place
                }
            }
        } else {
            false
        }
    }
    // pub fn house_count(&self, board: &Board) -> (u8, u8, u8) {
    pub fn house_count(&self, board: &Board) -> (u8, u8) {
        if self.has_monopoly(board) {
            let park_place = match self {
                BlueProperty::ParkPlace { state } => match state {
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
            let boardwalk = match self {
                BlueProperty::Boardwalk { state } => match state {
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
            // (park_place, boardwalk, illinois_ave)
            (park_place, boardwalk)
        } else {
            // (0, 0, 0)
            (0, 0)
        }
    }
    pub fn has_monopoly(&self, board: &Board) -> bool {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            let monopoly = vec![
                Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::ParkPlace {
                    state: PropertyState::Owned,
                })),
                Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::Boardwalk {
                    state: PropertyState::Owned,
                })),
            ];
            monopoly.iter().all(|property| {
                if owner.properties.contains(property) {
                    match property {
                        Properties::ColoredProperty(ColoredProperties::Blue(prop)) => match prop {
                            BlueProperty::ParkPlace { state }
                            | BlueProperty::Boardwalk { state }
                            // | BlueProperty::IllinoisAve { state } => {
                             => {
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
            BlueProperty::ParkPlace { state } => match state {
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
            BlueProperty::Boardwalk { state } => match state {
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
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        for player in players.iter() {
            let player_ref = player.borrow();
            let properties = player_ref.properties.clone();
            for property in properties.iter() {
                match self {
                    BlueProperty::ParkPlace { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Blue(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    BlueProperty::Boardwalk { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Blue(prop)) = property
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
            BlueProperty::ParkPlace { state } => {
                matches!(state, PropertyState::ForSale)
            }
            BlueProperty::Boardwalk { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn park_place() -> Self {
        BlueProperty::ParkPlace {
            state: PropertyState::ForSale,
        }
    }
    pub fn boardwalk() -> Self {
        BlueProperty::Boardwalk {
            state: PropertyState::ForSale,
        }
    }
}
