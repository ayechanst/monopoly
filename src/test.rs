// 22 properties
// 4 trains
// 3 chance
// 3 community chest
// 2 taxes (Income tax, Luxury tax)
// 2 utilities
// 1 free parking
// 1 go to jail
// 1 jail
// 1 Go

pub struct Board {
    pub spaces: Vec<Space>,
}

pub enum Space {
    Property(Deed),
    Chance,
    CommunityChest,
    Go,
    Jail,
    FreeParking,
}

pub struct Deed {
    name: String,
    price: u64,
}
