use crate::models::spaces::Space;

#[derive(Debug)]
pub enum Utilities {
    ElectricCompany,
    WaterWorks,
}

impl Utilities {
    // needs to take in dice roll to calculate rent_price
    pub fn rent_price(&self) -> i32 {
        420
    }
    pub fn electric_company() -> Self {
        Utilities::ElectricCompany
    }
    pub fn water_works() -> Self {
        Utilities::WaterWorks
    }
    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::Utility(self))
    }
}
