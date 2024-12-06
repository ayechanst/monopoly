use super::{
    colored_properties::colored_properties::ColoredProperties, railroad::Railroads,
    utility::Utilities,
};
use crate::models::{board::Board, player::Player};

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
    pub fn buy_property(&mut self, player: &mut Player) {
        // pub fn buy_property(&mut self, player: Player) {
        match self {
            Properties::ColoredProperty(colored_properties) => {
                colored_properties.buy_property(player)
            }
            Properties::Utility(utilities) => utilities.buy_property(player),
            Properties::Railroad(railroads) => railroads.buy_property(player),
        }
    }
    pub fn get_owner(&self, board: Board) -> Option<Player> {
        match self {
            Properties::ColoredProperty(colored_properties) => colored_properties.get_owner(board),
            Properties::Utility(utilities) => utilities.get_owner(board),
            Properties::Railroad(railroads) => railroads.get_owner(board),
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
