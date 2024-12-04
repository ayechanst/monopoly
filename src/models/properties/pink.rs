use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

pub enum PinkProperty {
    StCharlesPlace { houses: HouseCount },
    StatesAve { houses: HouseCount },
    VirginiaAve { houses: HouseCount },
}

impl PinkProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            PinkProperty::StCharlesPlace { houses } | PinkProperty::StatesAve { houses } => {
                let rent = 10;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => 625,
                    Hotel => 750,
                }
            }
            PinkProperty::VirginiaAve { houses } => {
                let rent = 12;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => 500,
                    Four => 700,
                    Hotel => 900,
                }
            }
        }
    }
    pub fn st_charles_place() -> Self {
        PinkProperty::StCharlesPlace {
            houses: HouseCount::Zero,
        }
    }
    pub fn states_ave() -> Self {
        PinkProperty::StatesAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn virginia_ave() -> Self {
        PinkProperty::VirginiaAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Pink(self),
        ))
    }
}
