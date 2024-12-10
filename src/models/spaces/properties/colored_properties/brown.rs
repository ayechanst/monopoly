use super::colored_properties::ColoredProperties;
use crate::models::{
    board::{Board, PlayerRef},
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState, Space},
    },
};

#[derive(Debug, PartialEq, Clone, Copy)]
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

    // pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
    pub fn get_owner(&self, board: Board) -> Option<PlayerRef> {
        let players = board.players;
        match self {
            BrownProperty::MediterraneanAve { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
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
                    let properties = &player.borrow().properties;
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
    pub fn buy_property(&mut self, player: PlayerRef) {
        match self {
            BrownProperty::MediterraneanAve { mut state } => {
                player.borrow_mut().money -= 60;
                let bought_property = Properties::ColoredProperty(ColoredProperties::Brown(
                    BrownProperty::MediterraneanAve {
                        state: PropertyState::Owned,
                    },
                ));
                player.borrow_mut().add_property(bought_property);
                state = PropertyState::Owned;
                println!("Property State: {:?}", state);
            }
            BrownProperty::BalticAve { mut state } => {
                player.borrow_mut().money -= 60;
                let bought_property = Properties::ColoredProperty(ColoredProperties::Brown(
                    BrownProperty::BalticAve {
                        state: PropertyState::Owned,
                    },
                ));
                player.borrow_mut().add_property(bought_property);
                state = PropertyState::Owned;
                println!("Property State: {:?}", state);
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
