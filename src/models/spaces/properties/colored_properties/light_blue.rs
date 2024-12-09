use super::colored_properties::ColoredProperties;
use crate::models::{
    board::{Board, PlayerRef},
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState},
    },
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LightBlueProperty {
    OrientalAve { state: PropertyState },
    VermontAve { state: PropertyState },
    ConnecticutAve { state: PropertyState },
}

impl LightBlueProperty {
    pub fn rent_price(&self, board: &Board) -> i32 {
        let rent = 6;
        match self {
            LightBlueProperty::OrientalAve { state } => match state {
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
            LightBlueProperty::VermontAve { state } => match state {
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
            LightBlueProperty::ConnecticutAve { state } => match state {
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
    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price(board);
        owner.money += rent_price;
        renter.money -= rent_price;
    }
    pub fn has_monopoly(&self, board: &Board) -> bool {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            let monopoly = vec![
                Properties::ColoredProperty(ColoredProperties::LightBlue(
                    LightBlueProperty::OrientalAve {
                        state: PropertyState::Owned,
                    },
                )),
                Properties::ColoredProperty(ColoredProperties::LightBlue(
                    LightBlueProperty::VermontAve {
                        state: PropertyState::Owned,
                    },
                )),
                Properties::ColoredProperty(ColoredProperties::LightBlue(
                    LightBlueProperty::ConnecticutAve {
                        state: PropertyState::Owned,
                    },
                )),
            ];
            monopoly
                .iter()
                .all(|property| owner.properties.contains(property))
        } else {
            false
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        for player in players.iter() {
            let player_ref = player.borrow();
            let properties = player_ref.properties.clone();
            for property in properties.iter() {
                match self {
                    LightBlueProperty::OrientalAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(prop)) =
                            property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    LightBlueProperty::VermontAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(prop)) =
                            property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    LightBlueProperty::ConnecticutAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(prop)) =
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
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            LightBlueProperty::ConnecticutAve { .. } => 120,
            _ => 100,
        };
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        match self {
            LightBlueProperty::OrientalAve { state }
            | LightBlueProperty::VermontAve { state }
            | LightBlueProperty::ConnecticutAve { state } => *state = PropertyState::Owned,
        }
        player.add_property(Properties::ColoredProperty(ColoredProperties::LightBlue(
            *self,
        )));
    }
    pub fn for_sale(&self) -> bool {
        match self {
            LightBlueProperty::OrientalAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            LightBlueProperty::VermontAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            LightBlueProperty::ConnecticutAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn oriental_ave() -> Self {
        LightBlueProperty::OrientalAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn vermont_ave() -> Self {
        LightBlueProperty::VermontAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn connecticut_ave() -> Self {
        LightBlueProperty::ConnecticutAve {
            state: PropertyState::ForSale,
        }
    }
}
