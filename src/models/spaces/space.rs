use super::properties::properties::Properties;

#[derive(Debug, PartialEq, Clone)]
pub enum Space {
    Property(Properties), // ColoredProperty | Utility | Station
    Chance,
    CommunityChest,
    IncomeTax,
    LuxuryTax,
    Go,
    GoToJail,
    Jail,
    FreeParking,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyState {
    ForSale,
    Owned,
    Houses(HouseCount),
    Mortgaged,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HouseCount {
    Zero,
    One,
    Two,
    Three,
    Four,
    Hotel,
}
