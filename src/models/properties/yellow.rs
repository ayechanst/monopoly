use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};
pub enum YellowProperty {
    AtlanticAve { houses: HouseCount },
    VentnorAve { houses: HouseCount },
    MarvinGardens { houses: HouseCount },
}

impl YellowProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            YellowProperty::AtlanticAve { houses } | YellowProperty::VentnorAve { houses } => {
                let rent = 22;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 330,
                    Three => 800,
                    Four => 975,
                    Hotel => 1150,
                }
            }
            YellowProperty::MarvinGardens { houses } => {
                let rent = 24;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 360,
                    Three => 850,
                    Four => 1025,
                    Hotel => 1200,
                }
            }
        }
    }

    pub fn as_space(self) -> Space {
        // instance methods
        Space::Property(ColoredProperty::Yellow(self))
    }

    pub fn atlantic() -> Self {
        // class method
        YellowProperty::AtlanticAve {
            houses: HouseCount::Zero,
        }
    }
    // YellowPropery.atlantic().as_space()
}