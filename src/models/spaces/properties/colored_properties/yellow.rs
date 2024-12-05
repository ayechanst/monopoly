use crate::models::{
    self,
    space::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

#[derive(Debug)]
pub enum YellowProperty {
    AtlanticAve { houses: HouseCount },
    VentnorAve { houses: HouseCount },
    MarvinGardens { houses: HouseCount },
}

impl YellowProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            YellowProperty::AtlanticAve { houses } => {
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
            YellowProperty::VentnorAve { houses } => {
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

    pub fn atlantic_ave() -> Self {
        YellowProperty::AtlanticAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn ventnor_ave() -> Self {
        YellowProperty::VentnorAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn marvin_gardens() -> Self {
        YellowProperty::MarvinGardens {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::space::Properties::ColoredProperty(
            models::space::ColoredProperties::Yellow(self),
        ))
    }
}
