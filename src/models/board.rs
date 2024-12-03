use super::properties::BrownProperty;
use crate::models::spaces::Space;

pub struct Board {
    pub spaces: Vec<Space>,
}

impl Board {
    pub fn new() -> Board {
        let mut spaces = Vec::new();
        // Go
        spaces.push(Space::Go);
        // Brown
        spaces.push(BrownProperty::mediterranean_ave().as_space());
        spaces.push(BrownProperty::baltic_ave().as_space());
        // 1st Community Chest
        spaces.push(Space::CommunityChest);
        // Brown
        Board { spaces }
    }
}
// Tax
//         spaces.push(Space::Tax(TaxType {
//             name: "Income Tax",
//             amount: 200,
//         }));
//         ///////////////////////////////////////////// Train
//         spaces.push(Space::Train(Railroad {
//             name: "Reading Railroad",
//             amount: 200,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Oriental Avenue",
//             color: "light blue",
//             price: 100,
//         }));
//         spaces.push(Space::Chance); ////////////////// chance
//         spaces.push(Space::Property(Deed {
//             name: "Vermont Avenue",
//             color: "light blue",
//             price: 100,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Connecticut Avenue",
//             color: "light blue",
//             price: 120,
//         }));
//         spaces.push(Space::Jail); // make an enum for this
//         spaces.push(Space::Property(Deed {
//             name: "St. Charles Place",
//             color: "pink",
//             price: 140,
//         }));
//         spaces.push(Space::Utilities(Utility {
//             name: "Electric Company",
//             price: 150,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "States Avenue",
//             color: "pink",
//             price: 140,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Virginia Avenue",
//             color: "pink",
//             price: 160,
//         }));
//         // train
//         spaces.push(Space::Train(Railroad {
//             name: "Pennsylvania Railroad",
//             amount: 200,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "St. James Place",
//             color: "orange",
//             price: 180,
//         }));
//         spaces.push(Space::CommunityChest);
//         spaces.push(Space::Property(Deed {
//             name: "Tennessee Avenue",
//             color: "orange",
//             price: 180,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "New York Avenue",
//             color: "orange",
//             price: 200,
//         }));
//         spaces.push(Space::FreeParking);
//         spaces.push(Space::Property(Deed {
//             name: "Kentucky Avenue",
//             color: "red",
//             price: 220,
//         }));
//         spaces.push(Space::Chance); ////////////////// chance
//         spaces.push(Space::Property(Deed {
//             name: "Indiana Avenue",
//             color: "red",
//             price: 220,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Illinois Avenue",
//             color: "red",
//             price: 240,
//         }));
//         spaces.push(Space::Train(Railroad {
//             // railroad
//             name: "B&O Railroad",
//             amount: 200,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Atlantic Avenue",
//             color: "yellow",
//             price: 260,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Ventnor Avenue",
//             color: "yellow",
//             price: 260,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Marvin Gardens",
//             color: "yellow",
//             price: 280,
//         }));
//         spaces.push(Space::GoToJail); // go to jail
//         spaces.push(Space::Property(Deed {
//             name: "Pacific Avenue",
//             color: "green",
//             price: 300,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "North Carolina Avenue",
//             color: "green",
//             price: 300,
//         }));
//         spaces.push(Space::CommunityChest); // chest
//         spaces.push(Space::Property(Deed {
//             name: "Pennsylvania Avenue",
//             color: "green",
//             price: 320,
//         }));
//         spaces.push(Space::Train(Railroad {
//             name: "Short Line",
//             amount: 200,
//         }));
//         spaces.push(Space::Chance); ////////////////// chance
//         spaces.push(Space::Property(Deed {
//             name: "Park Place",
//             color: "blue",
//             price: 350,
//         }));
//         spaces.push(Space::Tax(TaxType {
//             name: "Luxury Tax",
//             amount: 100,
//         }));
//         spaces.push(Space::Property(Deed {
//             name: "Boardwalk",
//             color: "blue",
//             price: 400,
//         }));
//     }
// }

// 22 properties
// 4 trains
// 2 utilities

// 3 chance
// 3 community chest
// 2 taxes (Income tax, Luxury tax)
// 1 free parking
// 1 go to jail
// 1 jail
// 1 Go
