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

    pub fn get_owner(&self, board: &Board) -> Option<Player> {
        let players = &board.players;
        match self {
            LightBlueProperty::OrientalAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
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
                    let properties = &player.properties;
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
                    let properties = &player.properties;
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
                println!("for_sale: {:?}", state);
                matches!(state, PropertyState::ForSale)
            }
            LightBlueProperty::VermontAve { state } => {
                println!("for_sale: {:?}", state);
                matches!(state, PropertyState::ForSale)
            }
            LightBlueProperty::ConnecticutAve { state } => {
                println!("for_sale: {:?}", state);
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            // LightBlueProperty::OrientalAve { mut state } => {
            LightBlueProperty::OrientalAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 100;
                    let bought_property = Properties::ColoredProperty(
                        ColoredProperties::LightBlue(LightBlueProperty::OrientalAve {
                            state: PropertyState::Owned,
                        }),
                    );
                    player.add_property(bought_property);
                    *state = PropertyState::Owned;
                    println!("Property State: {:?}", state);
                }
            }
            // LightBlueProperty::VermontAve { state } => {
            //     if *state == PropertyState::ForSale {
            //         player.money -= 100;
            //         let bought_property = Properties::ColoredProperty(
            //             ColoredProperties::LightBlue(LightBlueProperty::VermontAve {
            //                 state: PropertyState::Owned,
            //             }),
            //         );
            //         player.add_property(bought_property);
            //         *state = PropertyState::Owned;
            //         println!("Property State: {:?}", state);
            //     }
            // }
            // LightBlueProperty::VermontAve { mut state } => {
            LightBlueProperty::VermontAve { state } => {
                // look more into this
                if *state == PropertyState::ForSale {
                    player.money -= 100;
                    let bought_property = Properties::ColoredProperty(
                        ColoredProperties::LightBlue(LightBlueProperty::VermontAve {
                            state: PropertyState::Owned,
                        }),
                    );
                    player.add_property(bought_property);
                    *state = PropertyState::Owned; // this is not actually updating
                    println!("Property State: {:?}", state);
                }
            }
            // LightBlueProperty::ConnecticutAve { mut state } => {
            LightBlueProperty::ConnecticutAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 120;
                    let bought_property = Properties::ColoredProperty(
                        ColoredProperties::LightBlue(LightBlueProperty::ConnecticutAve {
                            state: PropertyState::Owned,
                        }),
                    );
                    player.add_property(bought_property);
                    *state = PropertyState::Owned;
                    println!("Property State: {:?}", state);
                }
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
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::LightBlue(
            self,
        )))
    }
}
