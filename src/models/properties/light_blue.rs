use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};

#[derive(Debug)]
pub enum LightBlueProperty {
    OrientalAve { houses: HouseCount },
    VermontAve { houses: HouseCount },
    ConnecticutAve { houses: HouseCount },
}

impl LightBlueProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            LightBlueProperty::OrientalAve { houses } => {
                let rent = 6;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => 400,
                    Hotel => 550,
                }
            }
            LightBlueProperty::VermontAve { houses } => {
                let rent = 6;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => rent * 15,
                    Three => rent * 45,
                    Four => 400,
                    Hotel => 550,
                }
            }
            LightBlueProperty::ConnecticutAve { houses } => {
                let rent = 8;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 100,
                    Three => 300,
                    Four => 450,
                    Hotel => 600,
                }
            }
        }
    }
    pub fn oriental_ave() -> Self {
        LightBlueProperty::OrientalAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn vermont_ave() -> Self {
        LightBlueProperty::VermontAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn connecticut_ave() -> Self {
        LightBlueProperty::ConnecticutAve {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::LightBlue(self),
        ))
    }
}
