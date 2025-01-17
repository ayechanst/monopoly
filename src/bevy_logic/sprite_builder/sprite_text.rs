use crate::models::spaces::space::Space;
use std::cell::RefCell;
use std::rc::Rc;

pub type SpaceRef = Rc<RefCell<Space>>;

// pub fn make_text(space: Ref<'_, Space>) -> &str {
// pub fn make_text(space: std::cell::Ref<'_, Space>) -> &str {
pub fn make_text(space: &Space) -> &str {
    // let space = space.borrow();
    let text = match *space {
        Space::Property(properties) => "Property",
        Space::Chance => "Chance",
        Space::CommunityChest => "Chest",
        Space::IncomeTax => "Tax",
        Space::LuxuryTax => "Tax",
        Space::Go => "Go",
        Space::GoToJail => "Go to Jail",
        Space::Jail => "Jail",
        Space::FreeParking => "Free Parking",
    };
    text
}
