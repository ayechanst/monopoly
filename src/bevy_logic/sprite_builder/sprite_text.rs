use crate::models::spaces::space::Space;

pub fn make_text(space: &Space) -> &str {
    let text = match *space {
        Space::Property(properties) => "",
        Space::Chance => "",
        Space::CommunityChest => "",
        Space::IncomeTax => "",
        Space::LuxuryTax => "",
        Space::Go => "Go",
        Space::GoToJail => "",
        Space::Jail => "",
        Space::FreeParking => "",
    };
    text
}
