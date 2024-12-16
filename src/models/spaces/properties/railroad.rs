use crate::{
    models::{
        board::{Board, PlayerRef},
        spaces::space::{PropertyState, Space},
    },
    utils::prompts::bid,
};

use super::properties::Properties;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Railroads {
    Reading { state: PropertyState },
    Pennsylvania { state: PropertyState },
    Bo { state: PropertyState },
    ShortLine { state: PropertyState },
}

impl Railroads {
    pub fn rent_price(&self, board: &Board) -> i32 {
        let railroads_owned = self.count_railroads(board);
        if railroads_owned == 1 {
            25
        } else if railroads_owned == 2 {
            50
        } else if railroads_owned == 3 {
            150
        } else if railroads_owned == 4 {
            200
        } else {
            0
        }
    }
    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price(board);
        owner.money += rent_price;
        renter.money -= rent_price;
    }
    pub fn count_railroads(&self, board: &Board) -> u8 {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            owner
                .properties
                .iter()
                .filter(|property| matches!(property, Properties::Railroad(_)))
                .count() as u8
        } else {
            0
        }
    }
    // old logic for get_owner()
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            Railroads::Reading { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(reading) = property {
                            if reading == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::Pennsylvania { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(pennsylvania) = property {
                            if pennsylvania == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::Bo { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(bo) = property {
                            if bo == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::ShortLine { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(short_line) = property {
                            if short_line == self {
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
            Railroads::Reading { state }
            | Railroads::Pennsylvania { state }
            | Railroads::Bo { state }
            | Railroads::ShortLine { state } => *state = PropertyState::Owned,
        }
        player.add_property(Properties::Railroad(*self));
    }
    pub fn mortgage(&mut self, player_ref: PlayerRef) {
        let mortgage_value = 75;
        let mut player = player_ref.borrow_mut();
        player.money += mortgage_value;
        match self {
            Railroads::Reading { state }
            | Railroads::Pennsylvania { state }
            | Railroads::Bo { state }
            | Railroads::ShortLine { state } => *state = PropertyState::Mortgaged,
        }
        if let Some(property) = player.properties.iter_mut().find(|p| match p {
            Properties::Railroad(inner) => inner == self,
            _ => false,
        }) {
            if let Properties::Railroad(inner) = property {
                match inner {
                    Railroads::Reading { state }
                    | Railroads::Pennsylvania { state }
                    | Railroads::Bo { state }
                    | Railroads::ShortLine { state } => *state = PropertyState::Mortgaged,
                }
            }
        }
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
                    Railroads::Reading { state }
                    | Railroads::Pennsylvania { state }
                    | Railroads::Bo { state }
                    | Railroads::ShortLine { state } => *state = PropertyState::Owned,
                }
                winner.add_property(Properties::Railroad(*self));
                println!(
                    "Player {:?} has aquired {:?} for ${:?}",
                    winner.player_number,
                    Properties::Railroad(*self),
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

    pub fn for_sale(&self) -> bool {
        match self {
            Railroads::Reading { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::Pennsylvania { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::Bo { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::ShortLine { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn reading() -> Self {
        Railroads::Reading {
            state: PropertyState::ForSale,
        }
    }

    pub fn pennsylvania() -> Self {
        Railroads::Pennsylvania {
            state: PropertyState::ForSale,
        }
    }

    pub fn bo() -> Self {
        Railroads::Bo {
            state: PropertyState::ForSale,
        }
    }

    pub fn shortline() -> Self {
        Railroads::ShortLine {
            state: PropertyState::ForSale,
        }
    }

    pub fn as_space(self) -> Space {
        Space::Property(Properties::Railroad(self))
    }
}
