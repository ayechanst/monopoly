use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

#[derive(Debug)]
pub enum GreenProperty {
    PacificAve { houses: HouseCount },
    NorthCarolinaAve { houses: HouseCount },
    PennsylvaniaAve { houses: HouseCount },
}

impl GreenProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            GreenProperty::PacificAve { houses } => {
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
            GreenProperty::NorthCarolinaAve { houses } => {
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
            GreenProperty::PennsylvaniaAve { houses } => {
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

    pub fn pacific_ave() -> Self {
        GreenProperty::PacificAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn north_carolina_ave() -> Self {
        GreenProperty::NorthCarolinaAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn pennsylvania_ave() -> Self {
        GreenProperty::PennsylvaniaAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Green(self),
        ))
    }
}
