#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Property {
    property: Properties,
    owner: Option<Player>, // RC or RefCell
}
