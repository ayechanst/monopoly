use super::colored_properties::ColoredProperties;
use crate::models::{
    board::{Board, PlayerRef},
    spaces::{
        properties::properties::Properties,
        space::{HouseCount, PropertyState},
    },
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PinkProperty {
    OrientalAve { state: PropertyState },
    VermontAve { state: PropertyState },
    VirginiaAve { state: PropertyState },
}

impl PinkProperty {
    pub fn rent_price(&self, board: &Board) -> i32 {
        let rent = 10;
        match self {
            PinkProperty::OrientalAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            rent * 2
                        } else {
                            rent
                        }
                    }
                    HouseCount::One => 50,
                    HouseCount::Two => 150,
                    HouseCount::Three => 450,
                    HouseCount::Four => 625,
                    HouseCount::Hotel => 750,
                },
                _ => rent,
            },
            PinkProperty::VermontAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            rent * 2
                        } else {
                            rent
                        }
                    }
                    HouseCount::One => 50,
                    HouseCount::Two => 150,
                    HouseCount::Three => 450,
                    HouseCount::Four => 625,
                    HouseCount::Hotel => 750,
                },
                _ => rent,
            },
            PinkProperty::VirginiaAve { state } => match state {
                PropertyState::Houses(house_count) => match house_count {
                    HouseCount::Zero => {
                        if self.has_monopoly(board) {
                            12
                        } else {
                            24
                        }
                    }
                    HouseCount::One => 60,
                    HouseCount::Two => 180,
                    HouseCount::Three => 500,
                    HouseCount::Four => 700,
                    HouseCount::Hotel => 900,
                },
                _ => rent,
            },
        }
    }
    pub fn pay_rent(&self, renter_ref: PlayerRef, board: &Board) {
        let owner_ref = self.get_owner(board).unwrap();
        let mut owner = owner_ref.borrow_mut();
        let mut renter = renter_ref.borrow_mut();
        let rent_price = self.rent_price(board);
        owner.money += rent_price;
        renter.money -= rent_price;
    }
    pub fn has_monopoly(&self, board: &Board) -> bool {
        if let Some(owner_ref) = self.get_owner(board) {
            let owner = owner_ref.borrow();
            let monopoly = vec![
                Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::OrientalAve {
                    state: PropertyState::Owned,
                })),
                Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::VermontAve {
                    state: PropertyState::Owned,
                })),
                Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::VirginiaAve {
                    state: PropertyState::Owned,
                })),
            ];
            monopoly
                .iter()
                .all(|property| owner.properties.contains(property))
        } else {
            false
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        let players = &board.players;
        for player in players.iter() {
            let player_ref = player.borrow();
            let properties = player_ref.properties.clone();
            for property in properties.iter() {
                match self {
                    PinkProperty::OrientalAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    PinkProperty::VermontAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                    PinkProperty::VirginiaAve { state } => {
                        if let Properties::ColoredProperty(ColoredProperties::Pink(prop)) = property
                        {
                            if prop == self {
                                return Some(player.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn buy_property(&mut self, player_ref: PlayerRef) {
        let cost = match self {
            PinkProperty::VirginiaAve { .. } => 120,
            _ => 100,
        };
        let mut player = player_ref.borrow_mut();
        player.money -= cost;
        match self {
            PinkProperty::OrientalAve { state }
            | PinkProperty::VermontAve { state }
            | PinkProperty::VirginiaAve { state } => *state = PropertyState::Owned,
        }
        player.add_property(Properties::ColoredProperty(ColoredProperties::Pink(*self)));
    }
    pub fn for_sale(&self) -> bool {
        match self {
            PinkProperty::OrientalAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::VermontAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            PinkProperty::VirginiaAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn st_charles_place() -> Self {
        PinkProperty::OrientalAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn states_ave() -> Self {
        PinkProperty::VermontAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn virginia_ave() -> Self {
        PinkProperty::VirginiaAve {
            state: PropertyState::ForSale,
        }
    }
}
