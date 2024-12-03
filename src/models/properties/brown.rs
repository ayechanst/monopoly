use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};
pub enum BrownProperty {
    MediterraneanAve { houses: HouseCount },
    BalticAve { houses: HouseCount },
}

impl BrownProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            BrownProperty::MediterraneanAve { houses } => {
                let rent = 2;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => rent * 80,
                    Hotel => rent * 125,
                }
            }
            BrownProperty::BalticAve { houses } => {
                let rent = 4;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => rent * 80,
                    Hotel => 450,
                }
            }
        }
    }
    pub fn mediterranean_ave(self) -> Self {
        BrownProperty::MediterraneanAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Brown(self),
        ))
    }
}
