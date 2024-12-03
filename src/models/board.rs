pub struct Board {
    pub spaces: Vec<Space>,
}

impl Board {
    pub fn new() -> Board {
        let mut spaces = Vec::new();
        spaces.push(Space::Go);
        spaces.push(Space::Property(Deed {
            name: "Mediterranean Avenue",
            color: "brown",
            price: 60,
        }))
    }
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
    color: String,
    price: u64,
}
