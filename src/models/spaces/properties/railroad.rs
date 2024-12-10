use crate::models::{
    board::{Board, PlayerRef},
    spaces::space::{PropertyState, Space},
};

use super::properties::Properties;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        match self {
            Railroads::Reading { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(reading) = property {
                            if reading == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::Pennsylvania { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(pennsylvania) = property {
                            if pennsylvania == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::Bo { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(bo) = property {
                            if bo == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
            Railroads::ShortLine { state } => {
                for player in players.iter() {
                    let properties = &player.borrow().properties;
                    for property in properties.iter() {
                        if let Properties::Railroad(short_line) = property {
                            if short_line == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
                None
            }
        }
    }

    pub fn buy_property(&mut self, player: PlayerRef) {
        match self {
            Railroads::Reading { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 200;
                    let bought_property = Properties::Railroad(Railroads::Reading {
                        state: PropertyState::Owned,
                    });
                    player.borrow_mut().add_property(bought_property);
                }
            }
            Railroads::Pennsylvania { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 200;
                    let bought_property = Properties::Railroad(Railroads::Pennsylvania {
                        state: PropertyState::Owned,
                    });
                    player.borrow_mut().add_property(bought_property);
                }
            }
            Railroads::Bo { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 200;
                    let bought_property = Properties::Railroad(Railroads::Bo {
                        state: PropertyState::Owned,
                    });
                    player.borrow_mut().add_property(bought_property);
                }
            }
            Railroads::ShortLine { state } => {
                if *state == PropertyState::ForSale {
                    player.borrow_mut().money -= 200;
                    let bought_property = Properties::Railroad(Railroads::ShortLine {
                        state: PropertyState::Owned,
                    });
                    player.borrow_mut().add_property(bought_property);
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
