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
pub enum GreenProperty {
    PacificAve { state: PropertyState },
    NorthCarolinaAve { state: PropertyState },
    PennsylvaniaAve { state: PropertyState },
}

impl GreenProperty {
    // TODO: rent price not correct
    pub fn rent_price(&self) -> i32 {
        match self {
            GreenProperty::PacificAve { state } => {
                let rent = 10;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 150,
                        HouseCount::Three => 450,
                        HouseCount::Four => 625,
                        HouseCount::Hotel => 750,
                    },
                    _ => rent,
                }
            }
            GreenProperty::NorthCarolinaAve { state } => {
                let rent = 10;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 150,
                        HouseCount::Three => 450,
                        HouseCount::Four => 625,
                        HouseCount::Hotel => 750,
                    },
                    _ => rent,
                }
            }
            GreenProperty::PennsylvaniaAve { state } => {
                let rent = 12;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 180,
                        HouseCount::Three => 500,
                        HouseCount::Four => 700,
                        HouseCount::Hotel => 900,
                    },
                    _ => rent,
                }
            }
        }
    }

    pub fn get_owner(&self, board: &Board) -> Option<Player> {
        let players = &board.players;
        match self {
            GreenProperty::PacificAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Green(
                            green_property,
                        )) = property
                        {
                            if green_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            GreenProperty::NorthCarolinaAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Green(
                            green_property,
                        )) = property
                        {
                            if green_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            GreenProperty::PennsylvaniaAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Green(
                            green_property,
                        )) = property
                        {
                            if green_property == self {
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
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            GreenProperty::PacificAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 300;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Green(
                        GreenProperty::PacificAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            GreenProperty::NorthCarolinaAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 300;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Green(
                        GreenProperty::NorthCarolinaAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            GreenProperty::PennsylvaniaAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 300;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Green(
                        GreenProperty::PennsylvaniaAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
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
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Green(self)))
    }
}
