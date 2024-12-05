use crate::models::{
    player::Player,
    spaces::space::{PropertyState, Space},
};

use super::properties::Properties;

#[derive(Debug, PartialEq, Clone)]
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
                    let bought_property = Properties::Railroad(Railroads::Reading {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
                }
            }
            Railroads::Pennsylvania { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    let bought_property = Properties::Railroad(Railroads::Pennsylvania {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
                }
            }
            Railroads::Bo { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    let bought_property = Properties::Railroad(Railroads::Bo {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
                }
            }
            Railroads::ShortLine { state } => {
                if *state == PropertyState::ForSale {
                    player.money -= 200;
                    let bought_property = Properties::Railroad(Railroads::ShortLine {
                        state: PropertyState::Owned,
                    });
                    player.add_property(bought_property);
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
        Space::Property(Properties::Railroad(self))
    }
}
