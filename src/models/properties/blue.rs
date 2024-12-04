use crate::models::{
    self,
    spaces::{
        HouseCount::{self, Four, Hotel, One, Three, Two, Zero},
        Space,
    },
};
#[derive(Debug)]
pub enum BlueProperty {
    ParkPlace { houses: HouseCount },
    Boardwalk { houses: HouseCount },
}

impl BlueProperty {
    pub fn rent_price(&self) -> i32 {
        match self {
            BlueProperty::ParkPlace { houses } => {
                let rent = 35;
                match houses {
                    Zero => rent,
                    One => rent * 5,
                    Two => 500,
                    Three => 1100,
                    Four => 1300,
                    Hotel => 1500,
                }
            }
            BlueProperty::Boardwalk { houses } => {
                let rent = 50;
                match houses {
                    Zero => rent,
                    One => rent * 4,
                    Two => 600,
                    Three => 1400,
                    Four => 1700,
                    Hotel => 2000,
                }
            }
        }
    }
    pub fn park_place() -> Self {
        // class method: creates an instance of itself
        BlueProperty::ParkPlace {
            houses: HouseCount::Zero,
        }
    }
    pub fn boardwalk() -> Self {
        BlueProperty::Boardwalk {
            houses: HouseCount::Zero,
        }
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::ColoredProperty(
            models::spaces::ColoredProperties::Blue(self),
        ))
    }
}
