use super::properties::Properties;
use crate::models::{
    board::{Board, PlayerRef},
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

    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price();
        owner.money += rent_price;
        renter.money -= rent_price;
    }

    // TODO: get_owner uses the old logic, update later
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            Utilities::ElectricCompany { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
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
                    let properties = &player.borrow().properties;
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
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = 200;
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        match self {
            Utilities::ElectricCompany { state } | Utilities::WaterWorks { state } => {
                *state = PropertyState::Owned
            }
        }
        player.add_property(Properties::Utility(*self));
        // match self {
        //     Utilities::ElectricCompany { state } => {
        //         if *state == PropertyState::ForSale {
        //             player.borrow_mut().money -= 150;
        //             let bought_property = Properties::Utility(Utilities::ElectricCompany {
        //                 state: PropertyState::Owned,
        //             });
        //             player.borrow_mut().add_property(bought_property);
        //         }
        //     }
        //     Utilities::WaterWorks { state } => {
        //         if *state == PropertyState::ForSale {
        //             player.borrow_mut().money -= 150;
        //             let bought_property = Properties::Utility(Utilities::WaterWorks {
        //                 state: PropertyState::Owned,
        //             });
        //             player.borrow_mut().add_property(bought_property);
        //         }
        //     }
        // }
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
