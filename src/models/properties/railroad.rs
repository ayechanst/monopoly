use crate::models::{
    player::Player,
    spaces::{PropertyState, Space},
};

// #[derive(Debug)]
#[derive(Debug, PartialEq)]
pub enum Railroads {
    Reading { state: PropertyState },
    Pennsylvania { state: PropertyState },
    Bo { state: PropertyState },
    ShortLine { state: PropertyState },
}

impl Railroads {
    // NewRailRoad.rent_price(count of railroads)
    pub fn rent_price(&self) -> i32 {
        6699
    }
    pub fn for_sale(&self) -> bool {
        match self {
            Railroads::Reading { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::Pennsylvania { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::Bo { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Railroads::ShortLine { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            Railroads::Reading { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
            Railroads::Pennsylvania { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
            Railroads::Bo { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
            Railroads::ShortLine { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
        }
    }

    pub fn reading() -> Self {
        Railroads::Reading {
            state: PropertyState::ForSale,
        }
    }

    pub fn pennsylvania() -> Self {
        Railroads::Pennsylvania {
            state: PropertyState::ForSale,
        }
    }

    pub fn bo() -> Self {
        Railroads::Bo {
            state: PropertyState::ForSale,
        }
    }

    pub fn shortline() -> Self {
        Railroads::ShortLine {
            state: PropertyState::ForSale,
        }
    }

    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::Railroad(self))
    }
}
