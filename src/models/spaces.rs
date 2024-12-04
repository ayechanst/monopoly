mod properties;
use crate::models::properties::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    Railroads, RedProperty, Utilities, YellowProperty,
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
    Railroad(Railroads),
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
