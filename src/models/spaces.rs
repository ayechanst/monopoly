use crate::models::properties::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    Railroads, RedProperty, Utilities, YellowProperty,
};

use super::player::{self, Player};

#[derive(Debug, PartialEq)]
pub enum Space {
    Property(Properties), // ColoredProperty | Utility | Station
    Chance,
    CommunityChest,
    IncomeTax,
    LuxuryTax,
    Go,
    GoToJail,
    Jail,
    FreeParking,
}
#[derive(Debug, PartialEq)]
pub enum Properties {
    ColoredProperty(ColoredProperties),
    Utility(Utilities),
    Railroad(Railroads),
}

impl Properties {
    pub fn is_for_sale(&self) -> bool {
        match self {
            Properties::ColoredProperty(colored_properties) => colored_properties.for_sale(),
            Properties::Utility(utilities) => utilities.for_sale(),
            Properties::Railroad(railroads) => railroads.for_sale(),
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.buy_property(player)
            }
            Properties::Utility(utilities) => utilities.buy_property(player),
            Properties::Railroad(railroads) => railroads.buy_property(player),
        }
    }
    pub fn get_owner(&self) -> Player {
        match self {
            Properties::ColoredProperty(colored_properties) => todo!(),
            Properties::Utility(utilities) => todo!(),
            Properties::Railroad(railroads) => todo!(),
        }
    }
    pub fn pay_rent(&mut self, renter: &mut Player) {
        match self {
            Properties::ColoredProperty(colored_properties) => todo!(),
            Properties::Utility(utilities) => todo!(),
            Properties::Railroad(railroads) => todo!(),
        }
    }
}

// Colored Properties Logic
#[derive(Debug, PartialEq)]
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
    pub fn for_sale(&self) -> bool {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.for_sale(),
            ColoredProperties::LightBlue(light_blue_property) => todo!(),
            ColoredProperties::Pink(pink_property) => todo!(),
            ColoredProperties::Orange(orange_property) => todo!(),
            ColoredProperties::Red(red_property) => todo!(),
            ColoredProperties::Yellow(yellow_property) => todo!(),
            ColoredProperties::Green(green_property) => todo!(),
            ColoredProperties::Blue(blue_property) => todo!(),
        }
    }
    pub fn buy_property(&mut self, player: &mut Player) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.buy_property(player),
            ColoredProperties::LightBlue(light_blue_property) => todo!(),
            ColoredProperties::Pink(pink_property) => todo!(),
            ColoredProperties::Orange(orange_property) => todo!(),
            ColoredProperties::Red(red_property) => todo!(),
            ColoredProperties::Yellow(yellow_property) => todo!(),
            ColoredProperties::Green(green_property) => todo!(),
            ColoredProperties::Blue(blue_property) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyState {
    ForSale,
    Owned(u8),
    Houses(HouseCount),
    Mortgaged,
}

// #[derive(Debug, PartialEq)]
// pub enum Players {
//     PlayerOne,
//     PlayerTwo,
//     PlayerThree,
//     PlayerFour,
// }

#[derive(Debug, PartialEq)]
pub enum HouseCount {
    Zero,
    One,
    Two,
    Three,
    Four,
    Hotel,
}
