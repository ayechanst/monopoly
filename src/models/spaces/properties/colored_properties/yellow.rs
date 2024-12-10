use super::colored_properties::ColoredProperties;
use crate::models::{
    board::{Board, PlayerRef},
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState, Space},
    },
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum YellowProperty {
    AtlanticAve { state: PropertyState },
    VentnorAve { state: PropertyState },
    MarvinGardens { state: PropertyState },
}

impl YellowProperty {
    // TODO: rent price not correct
    pub fn rent_price(&self) -> i32 {
        match self {
            YellowProperty::AtlanticAve { state } => {
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
            YellowProperty::VentnorAve { state } => {
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
            YellowProperty::MarvinGardens { state } => {
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

    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            YellowProperty::AtlanticAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Yellow(
                            yellow_property,
                        )) = property
                        {
                            if yellow_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            YellowProperty::VentnorAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Yellow(
                            yellow_property,
                        )) = property
                        {
                            if yellow_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            YellowProperty::MarvinGardens { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Yellow(
                            yellow_property,
                        )) = property
                        {
                            if yellow_property == self {
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
            YellowProperty::AtlanticAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            YellowProperty::VentnorAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            YellowProperty::MarvinGardens { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: PlayerRef) {
        match self {
            YellowProperty::AtlanticAve { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 260;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Yellow(
                        YellowProperty::AtlanticAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.borrow_mut().add_property(bought_property);
                }
            }
            YellowProperty::VentnorAve { state } => {
                player.borrow_mut().money -= 260;
                if *state == PropertyState::ForSale {
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Yellow(
                        YellowProperty::VentnorAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.borrow_mut().add_property(bought_property);
                }
            }
            YellowProperty::MarvinGardens { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 280;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Yellow(
                        YellowProperty::MarvinGardens {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.borrow_mut().add_property(bought_property);
                }
            }
        }
    }
    pub fn atlantic_ave() -> Self {
        YellowProperty::AtlanticAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn ventnor_ave() -> Self {
        YellowProperty::VentnorAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn marvin_gardens() -> Self {
        YellowProperty::MarvinGardens {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Yellow(self)))
    }
}
