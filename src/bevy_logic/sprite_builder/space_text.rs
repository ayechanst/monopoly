use crate::models::spaces::space::Space;

pub fn make_text(space: &Space) -> (&str, f32) {
    let text = match *space {
        Space::Property(properties) => ("", 0.0),
        Space::Chance => ("?", 0.6),
        Space::CommunityChest => ("!", 0.6),
        Space::IncomeTax => ("Income Tax", 0.2),
        Space::LuxuryTax => ("Luxury Tax", 0.2),
        Space::Go => ("Go", 0.5),
        Space::GoToJail => ("Go to Jail", 0.2),
        Space::Jail => ("Jail", 0.2),
        Space::FreeParking => ("Free Parking", 0.2),
    };
    text
}

pub fn make_coordinates_text(coordinates: (f32, f32)) -> String {
    let (x, y) = coordinates;
    let text = format!("{}, {}", x, y);
    text
}
