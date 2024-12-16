use super::properties::Properties;
use crate::{
    models::{
        board::{Board, PlayerRef},
        player::Player,
        spaces::space::{PropertyState, Space},
    },
    utils::prompts::bid,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Utilities {
    ElectricCompany { state: PropertyState },
    WaterWorks { state: PropertyState },
}

impl Utilities {
    // needs to take in dice roll to calculate rent_price
    pub fn rent_price(&self) -> i32 {
        420
    }
    pub fn for_sale(&self) -> bool {
        match self {
            Utilities::ElectricCompany { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Utilities::WaterWorks { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }

    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price();
        owner.money += rent_price;
        renter.money -= rent_price;
    }

    // TODO: get_owner uses the old logic, update later
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            Utilities::ElectricCompany { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Utility(electric_company) = property {
                            if electric_company == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Utilities::WaterWorks { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Utility(water_works) = property {
                            if water_works == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
        }
    }
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = 200;
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        match self {
            Utilities::ElectricCompany { state } | Utilities::WaterWorks { state } => {
                *state = PropertyState::Owned
            }
        }
        player.add_property(Properties::Utility(*self));
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
                    Utilities::ElectricCompany { state } | Utilities::WaterWorks { state } => {
                        *state = PropertyState::Owned;
                    }
                }
                winner.add_property(Properties::Utility(*self));
                println!(
                    "Player {:?} has aquired {:?} for ${:?}",
                    winner.player_number,
                    Properties::Utility(*self),
                    bid_price - 10
                );
                break;
            }
            current_bidder_index = (current_bidder_index + 1) % player_refs.len();
            round_count += 1;
            if round_count % 4 == 0 {
                bid_price += 10;
            }
        }
    }
    pub fn mortgage(&mut self, player_ref: PlayerRef) {
        let mortgage_value = 75;
        let mut player = player_ref.borrow_mut();
        player.money += mortgage_value;
        match self {
            Utilities::ElectricCompany { state } | Utilities::WaterWorks { state } => {
                *state = PropertyState::Mortgaged
            }
        }
        if let Some(property) = player.properties.iter_mut().find(|p| match p {
            Properties::Utility(inner) => inner == self,
            _ => false,
        }) {
            if let Properties::Utility(inner) = property {
                match inner {
                    Utilities::ElectricCompany { state } | Utilities::WaterWorks { state } => {
                        *state = PropertyState::Mortgaged
                    }
                }
            }
        }
    }
    pub fn electric_company() -> Self {
        Utilities::ElectricCompany {
            state: PropertyState::ForSale,
        }
    }
    pub fn water_works() -> Self {
        Utilities::WaterWorks {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::Utility(self))
    }
}
