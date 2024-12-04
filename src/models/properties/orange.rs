use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

pub enum OrangeProperty {
    StJamesPlace { houses: HouseCount },
    TennesseeAve { houses: HouseCount },
    NewYorkAve { houses: HouseCount },
}

impl OrangeProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            OrangeProperty::StJamesPlace { houses } => {
                let rent = 14;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 200,
                    Three => 550,
                    Four => 750,
                    Hotel => 950,
                }
            }
            OrangeProperty::TennesseeAve { houses } => {
                let rent = 14;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 200,
                    Three => 550,
                    Four => 750,
                    Hotel => 950,
                }
            }
            OrangeProperty::NewYorkAve { houses } => {
                let rent = 16;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 220,
                    Three => 600,
                    Four => 800,
                    Hotel => 1000,
                }
            }
        }
    }
    pub fn st_james_place() -> Self {
        OrangeProperty::StJamesPlace {
            houses: HouseCount::Zero,
        }
    }
    pub fn tennessee_ave() -> Self {
        OrangeProperty::TennesseeAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn new_york_ave() -> Self {
        OrangeProperty::NewYorkAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Orange(self),
        ))
    }
}
