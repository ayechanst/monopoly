use crate::models::spaces::{
    properties::properties::Properties,
    space::{ColoredProperties, HouseCount, Space},
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
                    HouseCount::Zero => rent,
                    HouseCount::One => rent * 5,
                    HouseCount::Two => 330,
                    HouseCount::Three => 800,
                    HouseCount::Four => 975,
                    HouseCount::Hotel => 1150,
                }
            }
            GreenProperty::NorthCarolinaAve { houses } => {
                let rent = 22;
                match houses {
                    HouseCount::Zero => rent,
                    HouseCount::One => rent * 5,
                    HouseCount::Two => 330,
                    HouseCount::Three => 800,
                    HouseCount::Four => 975,
                    HouseCount::Hotel => 1150,
                }
            }
            GreenProperty::PennsylvaniaAve { houses } => {
                let rent = 24;
                match houses {
                    HouseCount::Zero => rent,
                    HouseCount::One => rent * 5,
                    HouseCount::Two => 360,
                    HouseCount::Three => 850,
                    HouseCount::Four => 1025,
                    HouseCount::Hotel => 1200,
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
        Space::Property(Properties::ColoredProperty(ColoredProperties::Green(self)))
    }
}
