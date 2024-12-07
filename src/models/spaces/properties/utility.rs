use super::properties::Properties;
use crate::models::{
    board::Board,
    player::Player,
    spaces::space::{PropertyState, Space},
};

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn get_owner(&self, board: &Board) -> Option<Player> {
        let players = &board.players;
        match self {
            Utilities::ElectricCompany { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::Utility(electric_company) = property {
                            if electric_company == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Utilities::WaterWorks { state } => {
                for player in players.iter() {
                    let properties = &player.properties;
                    for property in properties.iter() {
                        if let Properties::Utility(water_works) = property {
                            if water_works == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            Utilities::ElectricCompany { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 150;
                    let bought_property = Properties::Utility(Utilities::ElectricCompany {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
                }
            }
            Utilities::WaterWorks { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 150;
                    let bought_property = Properties::Utility(Utilities::WaterWorks {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
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
        Space::Property(Properties::Utility(self))
    }
}
