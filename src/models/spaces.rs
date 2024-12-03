mod properties;
use crate::models::properties::{BrownProperty, LightBlueProperty, YellowProperty};

pub enum Space {
    Property(Properties), // ColoredProperty | Utility | Station
    Chance,
    CommunityChest,
    Tax(TaxType),
    Go,
    GoToJail,
    Jail,
    FreeParking,
}

pub enum Properties {
    ColoredProperty(ColoredProperties),
    Utility(Utilities),
    Station(Stations),
}
pub enum ColoredProperties {
    Brown(BrownProperty),
    LightBlue(LightBlueProperty),
    Yellow(YellowProperty),
}
// ColoredProperty Logic

impl ColoredProperties {
    pub fn rent_price(&self) -> i32 {
        match self {
            ColoredProperties::Brown(prop) => prop.rent_price(),
            ColoredProperties::LightBlue(prop) => prop.rent_price(),
            ColoredProperties::Yellow(prop) => prop.rent_price(), // prop is YellowProperty
        }
    }
}

pub enum Utilities {
    ElectricCompany,
    WaterWorks,
}

pub enum Stations {
    Reading,
    Pennsylvania,
    Bo,
    ShortLine,
}

impl Stations {
    // NewRailRoad.rent_price(count of railroads)
    pub fn rent_price(count: u32) -> u32 {
        count * 100
    }
}

//////////////////// other
pub enum TaxType {
    IncomeTax,
    LuxuryTax,
}

pub enum HouseCount {
    Zero,
    One,
    Two,
    Three,
    Four,
    Hotel,
}
