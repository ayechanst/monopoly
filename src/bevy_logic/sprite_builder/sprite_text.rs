use crate::models::spaces::space::Space;
use bevy::prelude::*;

pub fn make_text(space: &Space) -> &str {
    let text = match space {
        Space::Property(properties) => todo!(),
        Space::Chance => "Chance",
        Space::CommunityChest => todo!(),
        Space::IncomeTax => todo!(),
        Space::LuxuryTax => todo!(),
        Space::Go => "Go",
        Space::GoToJail => todo!(),
        Space::Jail => todo!(),
        Space::FreeParking => todo!(),
    };
    text
}
