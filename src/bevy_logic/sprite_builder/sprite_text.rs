use crate::models::spaces::space::Space;

pub fn make_text(space: &Space) -> (&str, f32) {
    let text = match *space {
        Space::Property(properties) => ("", 0.0),
        Space::Chance => ("Chance", 0.2),
        Space::CommunityChest => ("", 0.1),
        Space::IncomeTax => ("", 0.1),
        Space::LuxuryTax => ("", 0.1),
        Space::Go => ("Go", 0.5),
        Space::GoToJail => ("", 0.0),
        Space::Jail => ("Jail", 0.2),
        Space::FreeParking => ("", 0.0),
    };
    text
}
