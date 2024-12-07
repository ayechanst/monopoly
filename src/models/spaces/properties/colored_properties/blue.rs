use super::colored_properties::ColoredProperties;
use crate::models::{
    board::Board,
    player::Player,
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState, Space},
    },
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BlueProperty {
    ParkPlace { state: PropertyState },
    Boardwalk { state: PropertyState },
}

impl BlueProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            BlueProperty::ParkPlace { state } => {
                let rent = 2;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => rent * 15,
                        HouseCount::Three => rent * 45,
                        HouseCount::Four => rent * 80,
                        HouseCount::Hotel => rent * 125,
                    },
                    _ => rent,
                }
            }
            BlueProperty::Boardwalk { state } => {
                let rent = 4;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => rent * 15,
                        HouseCount::Three => rent * 45,
                        HouseCount::Four => rent * 80,
                        HouseCount::Hotel => 450,
                    },
                    _ => rent,
                }
            }
        }
    }

    pub fn get_owner(&self, board: &Board) -> Option<Player> {
        let players = &board.players;
        match self {
            BlueProperty::ParkPlace { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Blue(blue_property)) =
                            property
                        {
                            if blue_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            BlueProperty::Boardwalk { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Blue(blue_property)) =
                            property
                        {
                            if blue_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
        }
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
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            BlueProperty::ParkPlace { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 350;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Blue(
                        BlueProperty::ParkPlace {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            BlueProperty::Boardwalk { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 400;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Blue(
                        BlueProperty::Boardwalk {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
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
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Blue(self)))
    }
}
