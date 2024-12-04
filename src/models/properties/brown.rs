use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        PropertyState, Space,
    },
};
// #[derive(Debug)]
// pub enum BrownProperty {
//     MediterraneanAve { houses: HouseCount },
//     BalticAve { houses: HouseCount },
// }
#[derive(Debug)]
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
                        Zero => rent,
                        One => rent * 5,
                        Two => rent * 15,
                        Three => rent * 45,
                        Four => rent * 80,
                        Hotel => rent * 125,
                    },
                    _ => rent,
                }
            }
            BrownProperty::BalticAve { state } => {
                let rent = 4;
                match state {
                    PropertyState::Houses(house_count) => match house_count {
                        Zero => rent,
                        One => rent * 5,
                        Two => rent * 15,
                        Three => rent * 45,
                        Four => rent * 80,
                        Hotel => 450,
                    },
                    _ => rent,
                }
            }
        }
    }
    pub fn for_sale(&self) -> bool {
        // put this on utils and trains also
        match self {
            BrownProperty::MediterraneanAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
            BrownProperty::BalticAve { state } => {
                matches!(state, PropertyState::ForSale)
            }
        }
    }
    pub fn baltic_ave() -> Self {
        BrownProperty::MediterraneanAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn mediterranean_ave() -> Self {
        // class method: creates an instance of itself
        BrownProperty::MediterraneanAve {
            state: PropertyState::ForSale,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Brown(self),
        ))
    }
}
