use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

#[derive(Debug)]
pub enum RedProperty {
    KentuckyAve { houses: HouseCount },
    IndianaAve { houses: HouseCount },
    IllinoisAve { houses: HouseCount },
}

impl RedProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            RedProperty::KentuckyAve { houses } => {
                let rent = 18;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 250,
                    Three => 700,
                    Four => 875,
                    Hotel => 1050,
                }
            }
            RedProperty::IndianaAve { houses } => {
                let rent = 18;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 250,
                    Three => 700,
                    Four => 875,
                    Hotel => 1050,
                }
            }
            RedProperty::IllinoisAve { houses } => {
                let rent = 20;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 300,
                    Three => 750,
                    Four => 925,
                    Hotel => 1100,
                }
            }
        }
    }
    pub fn kentucky_ave() -> Self {
        RedProperty::KentuckyAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn indiana_ave() -> Self {
        RedProperty::IndianaAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn illinois_ave() -> Self {
        RedProperty::IllinoisAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Red(self),
        ))
    }
}
