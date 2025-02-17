use super::{
    colored_properties::colored_properties::ColoredProperties, railroad::Railroads,
    utility::Utilities,
};
use crate::models::board::{Board, PlayerRef};

// #[derive(Debug, PartialEq, Clone, Copy)]
#[derive(Debug, PartialEq, Clone)]
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
    pub fn buy_property(&mut self, player: PlayerRef) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.buy_property(player)
            }
            Properties::Utility(utilities) => utilities.buy_property(player),
            Properties::Railroad(railroads) => railroads.buy_property(player),
        }
    }
    pub fn auction(&mut self, board: &Board) {
        match self {
            Properties::ColoredProperty(colored_properties) => colored_properties.auction(board),
            Properties::Utility(utilities) => utilities.auction(board),
            Properties::Railroad(railroads) => railroads.auction(board),
        }
    }
    pub fn get_owner(&self, board: &Board) -> Option<PlayerRef> {
        match self {
            Properties::ColoredProperty(colored_properties) => colored_properties.get_owner(board),
            Properties::Utility(utilities) => utilities.get_owner(board),
            Properties::Railroad(railroads) => railroads.get_owner(board),
        }
    }
    pub fn pay_rent(&mut self, renter_ref: PlayerRef, board: &Board) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.pay_rent(renter_ref, board)
            }
            Properties::Utility(utilities) => utilities.pay_rent(renter_ref, board),
            Properties::Railroad(railroads) => railroads.pay_rent(renter_ref, board),
        }
    }
    pub fn mortgage(&mut self, player_ref: PlayerRef) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.mortgage(player_ref)
            }
            Properties::Utility(utilities) => utilities.mortgage(player_ref),
            Properties::Railroad(railroads) => railroads.mortgage(player_ref),
        }
    }

    pub fn buy_house(&mut self, player_ref: PlayerRef, board: &Board) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.buy_house(player_ref, board)
            }
            Properties::Utility(utilities) => todo!(),
            Properties::Railroad(railroads) => todo!(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
