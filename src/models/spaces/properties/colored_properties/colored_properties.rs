use super::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    RedProperty, YellowProperty,
};
use crate::models::board::{Board, PlayerRef};

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub fn rent_price(&self, board: &Board) -> i32 {
        match self {
            ColoredProperties::Brown(prop) => prop.rent_price(board),
            ColoredProperties::LightBlue(prop) => prop.rent_price(board),
            ColoredProperties::Pink(prop) => prop.rent_price(board),
            ColoredProperties::Orange(prop) => prop.rent_price(board),
            ColoredProperties::Red(prop) => prop.rent_price(board),
            ColoredProperties::Yellow(prop) => prop.rent_price(board),
            ColoredProperties::Green(prop) => prop.rent_price(board),
            ColoredProperties::Blue(prop) => prop.rent_price(board),
        }
    }
    pub fn for_sale(&self) -> bool {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.for_sale(),
            ColoredProperties::LightBlue(light_blue_property) => light_blue_property.for_sale(),
            ColoredProperties::Pink(pink_property) => pink_property.for_sale(),
            ColoredProperties::Orange(orange_property) => orange_property.for_sale(),
            ColoredProperties::Red(red_property) => red_property.for_sale(),
            ColoredProperties::Yellow(yellow_property) => yellow_property.for_sale(),
            ColoredProperties::Green(green_property) => green_property.for_sale(),
            ColoredProperties::Blue(blue_property) => blue_property.for_sale(),
        }
    }
    pub fn buy_property(&mut self, player: PlayerRef) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.buy_property(player),
            ColoredProperties::LightBlue(light_blue_property) => {
                light_blue_property.buy_property(player)
            }
            ColoredProperties::Pink(pink_property) => pink_property.buy_property(player),
            ColoredProperties::Orange(orange_property) => orange_property.buy_property(player),
            ColoredProperties::Red(red_property) => red_property.buy_property(player),
            ColoredProperties::Yellow(yellow_property) => yellow_property.buy_property(player),
            ColoredProperties::Green(green_property) => green_property.buy_property(player),
            ColoredProperties::Blue(blue_property) => blue_property.buy_property(player),
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.get_owner(board),
            ColoredProperties::LightBlue(light_blue_property) => {
                light_blue_property.get_owner(board)
            }
            ColoredProperties::Pink(pink_property) => pink_property.get_owner(board),
            ColoredProperties::Orange(orange_property) => orange_property.get_owner(board),
            ColoredProperties::Red(red_property) => red_property.get_owner(board),
            ColoredProperties::Yellow(yellow_property) => yellow_property.get_owner(board),
            ColoredProperties::Green(green_property) => green_property.get_owner(board),
            ColoredProperties::Blue(blue_property) => blue_property.get_owner(board),
        }
    }
    pub fn auction(&mut self, board: &Board) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.auction(board),
            ColoredProperties::LightBlue(light_blue_property) => light_blue_property.auction(board),
            ColoredProperties::Pink(pink_property) => pink_property.auction(board),
            ColoredProperties::Orange(orange_property) => orange_property.auction(board),
            ColoredProperties::Red(red_property) => red_property.auction(board),
            ColoredProperties::Yellow(yellow_property) => yellow_property.auction(board),
            ColoredProperties::Green(green_property) => green_property.auction(board),
            ColoredProperties::Blue(blue_property) => blue_property.auction(board),
        }
    }
    pub fn pay_rent(&mut self, renter_ref: PlayerRef, board: &Board) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.pay_rent(renter_ref, board),
            ColoredProperties::LightBlue(light_blue_property) => {
                light_blue_property.pay_rent(renter_ref, board)
            }
            ColoredProperties::Pink(pink_property) => pink_property.pay_rent(renter_ref, board),
            ColoredProperties::Orange(orange_property) => {
                orange_property.pay_rent(renter_ref, board)
            }
            ColoredProperties::Red(red_property) => red_property.pay_rent(renter_ref, board),
            ColoredProperties::Yellow(yellow_property) => {
                yellow_property.pay_rent(renter_ref, board)
            }
            ColoredProperties::Green(green_property) => green_property.pay_rent(renter_ref, board),
            ColoredProperties::Blue(blue_property) => blue_property.pay_rent(renter_ref, board),
        }
    }
    pub fn mortgage(&mut self, player_ref: PlayerRef) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.mortgage(player_ref),
            ColoredProperties::LightBlue(light_blue_property) => {
                light_blue_property.mortgage(player_ref)
            }
            ColoredProperties::Pink(pink_property) => pink_property.mortgage(player_ref),
            ColoredProperties::Orange(orange_property) => orange_property.mortgage(player_ref),
            ColoredProperties::Red(red_property) => red_property.mortgage(player_ref),
            ColoredProperties::Yellow(yellow_property) => yellow_property.mortgage(player_ref),
            ColoredProperties::Green(green_property) => green_property.mortgage(player_ref),
            ColoredProperties::Blue(blue_property) => blue_property.mortgage(player_ref),
        }
    }
    pub fn buy_house(&mut self, player_ref: PlayerRef, board: &Board) {
        match self {
            ColoredProperties::Brown(brown_property) => brown_property.buy_house(player_ref, board),
            ColoredProperties::LightBlue(light_blue_property) => {
                light_blue_property.buy_house(player_ref, board)
            }
            ColoredProperties::Pink(pink_property) => pink_property.buy_house(player_ref, board),
            ColoredProperties::Orange(orange_property) => {
                orange_property.buy_house(player_ref, board)
            }
            ColoredProperties::Red(red_property) => red_property.buy_house(player_ref, board),
            ColoredProperties::Yellow(yellow_property) => {
                yellow_property.buy_house(player_ref, board)
            }
            ColoredProperties::Green(green_property) => green_property.buy_house(player_ref, board),
            ColoredProperties::Blue(blue_property) => blue_property.buy_house(player_ref, board),
        }
    }
}
