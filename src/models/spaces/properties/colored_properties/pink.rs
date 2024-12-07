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
pub enum PinkProperty {
    StCharlesPlace { state: PropertyState },
    StatesAve { state: PropertyState },
    VirginiaAve { state: PropertyState },
}

impl PinkProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            PinkProperty::StCharlesPlace { state } => {
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
            PinkProperty::StatesAve { state } => {
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
            PinkProperty::VirginiaAve { state } => {
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
            PinkProperty::StCharlesPlace { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(pink_property)) =
                            property
                        {
                            if pink_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            PinkProperty::StatesAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(pink_property)) =
                            property
                        {
                            if pink_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            PinkProperty::VirginiaAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(pink_property)) =
                            property
                        {
                            if pink_property == self {
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
            PinkProperty::StCharlesPlace { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::StatesAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::VirginiaAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            PinkProperty::StCharlesPlace { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 140;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Pink(
                        PinkProperty::StCharlesPlace {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            PinkProperty::StatesAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 140;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Pink(
                        PinkProperty::StatesAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            PinkProperty::VirginiaAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 160;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Pink(
                        PinkProperty::VirginiaAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
        }
    }
    pub fn st_charles_place() -> Self {
        PinkProperty::StCharlesPlace {
            state: PropertyState::ForSale,
        }
    }
    pub fn states_ave() -> Self {
        PinkProperty::StatesAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn virginia_ave() -> Self {
        PinkProperty::VirginiaAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Pink(self)))
    }
}
