use super::colored_properties::ColoredProperties;
use crate::models::{
    board::Board,
    player::Player,
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState, Space},
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum BrownProperty {
    MediterraneanAve { state: PropertyState },
    BalticAve { state: PropertyState },
}

impl BrownProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            BrownProperty::MediterraneanAve { state } => {
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
            BrownProperty::BalticAve { state } => {
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

    // pub fn pay_rent(&self, renter: &mut Player) {
    //     match self {
    //         BrownProperty::MediterraneanAve { state } => {
    //             match state {
    //                 PropertyState::Owned(owner) => match owner {},
    //                 _ => 0,
    //             };
    //         }
    //         BrownProperty::BalticAve { state } => todo!(),
    //     }
    // }

    pub fn get_owner(&self, board: Board) -> Option<Player> {
        let players = board.players;
        match self {
            BrownProperty::MediterraneanAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Brown(
                            brown_property,
                        )) = property
                        {
                            if brown_property == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            BrownProperty::BalticAve { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::ColoredProperty(ColoredProperties::Brown(
                            brown_property,
                        )) = property
                        {
                            if brown_property == self {
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
            BrownProperty::MediterraneanAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            BrownProperty::BalticAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            BrownProperty::MediterraneanAve { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 60;
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Brown(
                        BrownProperty::MediterraneanAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                }
            }
            BrownProperty::BalticAve { state } => {
                if *state == PropertyState::ForSale {
                    let bought_property = Properties::ColoredProperty(ColoredProperties::Brown(
                        BrownProperty::BalticAve {
                            state: PropertyState::Owned,
                        },
                    ));
                    player.add_property(bought_property);
                    player.money -= 60;
                }
            }
        }
    }
    pub fn mediterranean_ave() -> Self {
        BrownProperty::MediterraneanAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn baltic_ave() -> Self {
        // class method: creates an instance of itself
        BrownProperty::BalticAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(Properties::ColoredProperty(ColoredProperties::Brown(self)))
    }
}
