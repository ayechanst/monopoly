use super::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    RedProperty, YellowProperty,
};
use crate::models::{board::Board, player::Player};

#[derive(Debug, PartialEq, Clone)]
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
    pub fn get_owner(&self, board: Board) -> Option<Player> {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.get_owner(board),
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
