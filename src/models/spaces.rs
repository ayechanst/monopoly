mod properties;
use crate::models::properties::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    RedProperty, Utilities, YellowProperty,
};

#[derive(Debug)]
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
#[derive(Debug)]
pub enum Properties {
    ColoredProperty(ColoredProperties),
    Utility(Utilities),
    Station(Stations),
}

// Colored Properties Logic
#[derive(Debug)]
pub enum ColoredProperties {
    Brown(BrownProperty),
    LightBlue(LightBlueProperty),
    Pink(PinkProperty),
    Orange(OrangeProperty),
    Red(RedProperty),
    Yellow(YellowProperty),
    Green(GreenProperty),
    Blue(BlueProperty),
}

impl ColoredProperties {
    pub fn rent_price(&self) -> i32 {
        match self {
            ColoredProperties::Brown(prop) => prop.rent_price(),
            ColoredProperties::LightBlue(prop) => prop.rent_price(),
            ColoredProperties::Pink(prop) => prop.rent_price(),
            ColoredProperties::Orange(prop) => prop.rent_price(),
            ColoredProperties::Red(prop) => prop.rent_price(),
            ColoredProperties::Yellow(prop) => prop.rent_price(), // prop is YellowProperty
            ColoredProperties::Green(prop) => prop.rent_price(),  // prop is YellowProperty
            ColoredProperties::Blue(prop) => prop.rent_price(),   // prop is YellowProperty
        }
    }
}

// Utilities Logic

// pub enum Utilities {
//     ElectricCompany,
//     WaterWorks,
// }

// Stations Logic

#[derive(Debug)]
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
#[derive(Debug)]
pub enum TaxType {
    IncomeTax,
    LuxuryTax,
}

#[derive(Debug)]
pub enum HouseCount {
    Zero,
    One,
    Two,
    Three,
    Four,
    Hotel,
}
