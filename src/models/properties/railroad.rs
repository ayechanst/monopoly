use crate::models::spaces::Space;

#[derive(Debug)]
pub enum Railroads {
    Reading,
    Pennsylvania,
    Bo,
    ShortLine,
}

impl Railroads {
    // NewRailRoad.rent_price(count of railroads)
    pub fn rent_price(&self) -> i32 {
        6699
    }

    pub fn reading() -> Self {
        Railroads::Reading
    }

    pub fn pennsylvania() -> Self {
        Railroads::Pennsylvania
    }

    pub fn bo() -> Self {
        Railroads::Bo
    }

    pub fn shortline() -> Self {
        Railroads::ShortLine
    }

    pub fn as_space(self) -> Space {
        // instance method: takes in itself, and executes code on itself
        Space::Property(crate::models::spaces::Properties::Railroad(self))
    }
}
