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
pub enum OrangeProperty {
    StJamesPlace { state: PropertyState },
    TennesseeAve { state: PropertyState },
    NewYorkAve { state: PropertyState },
}

impl OrangeProperty {
    // TODO: rent price not correct
    pub fn rent_price(&self) -> i32 {
        match self {
            OrangeProperty::StJamesPlace { state } => {
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
            OrangeProperty::TennesseeAve { state } => {
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
            OrangeProperty::NewYorkAve { state } => {
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
            OrangeProperty::StJamesPlace { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Orange(
                            orange_property,
                        )) = property
                        {
                            if orange_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            OrangeProperty::TennesseeAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Orange(
                            orange_property,
                        )) = property
                        {
                            if orange_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            OrangeProperty::NewYorkAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Orange(
                            orange_property,
                        )) = property
                        {
                            if orange_property == self {
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
            OrangeProperty::StJamesPlace { state } => {
                matches!(state, PropertyState::ForSale)
            }
            OrangeProperty::TennesseeAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            OrangeProperty::NewYorkAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            OrangeProperty::StJamesPlace { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 180;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Orange(
                        OrangeProperty::StJamesPlace {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            OrangeProperty::TennesseeAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 180;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Orange(
                        OrangeProperty::TennesseeAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            OrangeProperty::NewYorkAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Orange(
                        OrangeProperty::NewYorkAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
        }
    }
    pub fn st_james_place() -> Self {
        OrangeProperty::StJamesPlace {
            state: PropertyState::ForSale,
        }
    }
    pub fn tennessee_ave() -> Self {
        OrangeProperty::TennesseeAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn new_york_ave() -> Self {
        OrangeProperty::NewYorkAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Orange(self)))
    }
}
