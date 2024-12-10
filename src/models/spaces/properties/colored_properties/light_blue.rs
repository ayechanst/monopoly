use super::colored_properties::ColoredProperties;
use crate::models::{
    board::{Board, PlayerRef},
    player::Player,
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState, Space},
    },
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LightBlueProperty {
    OrientalAve { state: PropertyState },
    VermontAve { state: PropertyState },
    ConnecticutAve { state: PropertyState },
}

impl LightBlueProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            LightBlueProperty::OrientalAve { state } => {
                let rent = 6;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 90,
                        HouseCount::Three => 270,
                        HouseCount::Four => 400,
                        HouseCount::Hotel => 550,
                    },
                    _ => rent,
                }
            }
            LightBlueProperty::VermontAve { state } => {
                let rent = 6;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 90,
                        HouseCount::Three => 270,
                        HouseCount::Four => 400,
                        HouseCount::Hotel => 550,
                    },
                    _ => rent,
                }
            }
            LightBlueProperty::ConnecticutAve { state } => {
                let rent = 8;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        HouseCount::Zero => rent,
                        HouseCount::One => rent * 5,
                        HouseCount::Two => 100,
                        HouseCount::Three => 300,
                        HouseCount::Four => 450,
                        HouseCount::Hotel => 600,
                    },
                    _ => rent,
                }
            }
        }
    }

    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            LightBlueProperty::OrientalAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(
                            light_blue_property,
                        )) = property
                        {
                            if light_blue_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            LightBlueProperty::VermontAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(
                            light_blue_property,
                        )) = property
                        {
                            if light_blue_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            LightBlueProperty::ConnecticutAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::LightBlue(
                            light_blue_property,
                        )) = property
                        {
                            if light_blue_property == self {
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
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            LightBlueProperty::ConnecticutAve { .. } => 120,
            _ => 100,
        };

        // Mutably borrow the player and update their state.
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        player.add_property(Properties::ColoredProperty(ColoredProperties::LightBlue(
            *self,
        )));

        // Update the property state after releasing the mutable borrow on the player.
        match self {
            LightBlueProperty::OrientalAve { state }
            | LightBlueProperty::VermontAve { state }
            | LightBlueProperty::ConnecticutAve { state } => *state = PropertyState::Owned,
        }

        println!("Property State: {:?}", PropertyState::Owned);
        // let mut player = player_ref.borrow_mut();
        // match self {
        //     LightBlueProperty::OrientalAve { mut state } => {
        //         if state == PropertyState::ForSale {
        //             player.money -= 100;
        //             player.add_property(Properties::ColoredProperty(ColoredProperties::LightBlue(
        //                 LightBlueProperty::OrientalAve {
        //                     state: PropertyState::Owned,
        //                 },
        //             )));
        //             state = PropertyState::Owned;
        //             println!("Property State: {:?}", state);
        //         }
        //     }
        //     LightBlueProperty::VermontAve { mut state } => {
        //         if state == PropertyState::ForSale {
        //             player.money -= 100;
        //             player.add_property(Properties::ColoredProperty(ColoredProperties::LightBlue(
        //                 LightBlueProperty::VermontAve {
        //                     state: PropertyState::Owned,
        //                 },
        //             )));
        //             state = PropertyState::Owned;
        //             println!("Property State: {:?}", state);
        //         }
        //     }
        //     LightBlueProperty::ConnecticutAve { mut state } => {
        //         if state == PropertyState::ForSale {
        //             player.money -= 120;
        //             player.add_property(Properties::ColoredProperty(ColoredProperties::LightBlue(
        //                 LightBlueProperty::ConnecticutAve {
        //                     state: PropertyState::Owned,
        //                 },
        //             )));
        //             state = PropertyState::Owned;
        //             println!("Property State: {:?}", state);
        //         }
        //     }
        // }
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
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::LightBlue(
            self,
        )))
    }
}
