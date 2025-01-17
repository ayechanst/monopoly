use std::cell::RefCell;
use std::rc::Rc;

use crate::models::spaces::properties::colored_properties::colored_properties::ColoredProperties;
use crate::models::spaces::space::Space;
use bevy::prelude::*;
pub type SpaceRef = Rc<RefCell<Space>>;

pub fn make_color(space: &Space) -> Color {
    // let space = space_ref.borrow();
    let fill_color = match *space {
        Space::Property(properties) => match properties {
            crate::models::spaces::properties::properties::Properties::ColoredProperty(
                colored_properties,
            ) => match colored_properties {
                ColoredProperties::Brown(brown_property) => Color::srgb(0.54, 0.17, 0.0),
                ColoredProperties::LightBlue(light_blue_property) => Color::srgb(0.39, 0.72, 0.8),
                ColoredProperties::Pink(pink_property) => Color::srgb(0.9, 0.55, 0.81),
                ColoredProperties::Orange(orange_property) => Color::srgb(0.96, 0.56, 0.25),
                ColoredProperties::Red(red_property) => Color::srgb(0.83, 0.0, 0.19),
                ColoredProperties::Yellow(yellow_property) => Color::srgb(0.8, 0.85, 0.60),
                ColoredProperties::Green(green_property) => Color::srgb(0.0, 0.47, 0.0),
                ColoredProperties::Blue(blue_property) => Color::srgb(0., 0.4, 0.55),
            },
            crate::models::spaces::properties::properties::Properties::Utility(utilities) => {
                Color::srgb(1.0, 1.0, 1.0)
            }
            crate::models::spaces::properties::properties::Properties::Railroad(railroads) => {
                Color::srgb(0.37, 0.41, 0.44)
            }
        },
        Space::Chance | Space::CommunityChest | Space::IncomeTax | Space::LuxuryTax => {
            Color::srgb(1.0, 0.0, 0.0)
        }
        Space::Go | Space::GoToJail | Space::Jail | Space::FreeParking => {
            Color::srgb(0.0, 0.0, 1.0)
        }
    };
    fill_color
}
