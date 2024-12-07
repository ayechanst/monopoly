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
pub enum RedProperty {
    KentuckyAve { state: PropertyState },
    IndianaAve { state: PropertyState },
    IllinoisAve { state: PropertyState },
}

impl RedProperty {
    // TODO: rent price not correct
    pub fn rent_price(&self) -> i32 {
        match self {
            RedProperty::KentuckyAve { state } => {
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
            RedProperty::IndianaAve { state } => {
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
            RedProperty::IllinoisAve { state } => {
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
            RedProperty::KentuckyAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Red(red_property)) =
                            property
                        {
                            if red_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            RedProperty::IndianaAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Red(red_property)) =
                            property
                        {
                            if red_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            RedProperty::IllinoisAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Red(red_property)) =
                            property
                        {
                            if red_property == self {
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
            RedProperty::KentuckyAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            RedProperty::IndianaAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            RedProperty::IllinoisAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            RedProperty::KentuckyAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 220;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Red(
                        RedProperty::KentuckyAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            RedProperty::IndianaAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 220;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Red(
                        RedProperty::IndianaAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            RedProperty::IllinoisAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 240;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Red(
                        RedProperty::IllinoisAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
        }
    }
    pub fn kentucky_ave() -> Self {
        RedProperty::KentuckyAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn indiana_ave() -> Self {
        RedProperty::IndianaAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn illinois_ave() -> Self {
        RedProperty::IllinoisAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Red(self)))
    }
}
