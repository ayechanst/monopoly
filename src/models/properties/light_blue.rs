use crate::models::spaces::HouseCount::{self, Four, Hotel, One, Three, Two, Zero};

pub enum LightBlueProperty {
    OrientalAve { houses: HouseCount },
    VermontAve { houses: HouseCount },
    ConnecticutAve { houses: HouseCount },
}

impl LightBlueProperty {
    pub fn rent_price(&self) -> i32 {
        // TODO: if a person has all properties, rent is doubled
        match self {
            LightBlueProperty::OrientalAve { houses }
            | LightBlueProperty::VermontAve { houses } => {
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
}
