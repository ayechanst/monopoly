use crate::models::{
    player::Player,
    spaces::{PropertyState, Space},
};

#[derive(Debug, PartialEq)]
pub enum Utilities {
    ElectricCompany { state: PropertyState },
    WaterWorks { state: PropertyState },
}

impl Utilities {
    // needs to take in dice roll to calculate rent_price
    pub fn rent_price(&self) -> i32 {
        420
    }
    pub fn for_sale(&self) -> bool {
        match self {
            Utilities::ElectricCompany { state } => {
                matches!(state, PropertyState::ForSale)
            }
            Utilities::WaterWorks { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            Utilities::ElectricCompany { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 150;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
            Utilities::WaterWorks { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 150;
                    *state = PropertyState::Owned(player.player_number);
                }
            }
        }
    }
    pub fn electric_company() -> Self {
        Utilities::ElectricCompany {
            state: PropertyState::ForSale,
        }
    }
    pub fn water_works() -> Self {
        Utilities::WaterWorks {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::Utility(self))
    }
}
