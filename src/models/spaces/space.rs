use super::properties::properties::Properties;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyState {
    ForSale,
    Owned,
    Houses(HouseCount),
    Mortgaged,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HouseCount {
    Zero,
    One,
    Two,
    Three,
    Four,
    Hotel,
}
