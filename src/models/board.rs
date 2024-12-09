use super::{
    player::Player,
    spaces::{
        properties::{
            colored_properties::{
                colored_properties::ColoredProperties, BlueProperty, BrownProperty, GreenProperty,
                LightBlueProperty, OrangeProperty, PinkProperty, RedProperty, YellowProperty,
            },
            properties::Properties,
            railroad::Railroads,
            utility::Utilities,
        },
        space::Space,
    },
};

#[derive(Debug)]
pub struct Board {
    pub spaces: Vec<Space>,
    pub players: Vec<Player>, // have a Vec of players, with their current position
}

impl Board {
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        // Bottom 0-9
        spaces.push(Space::Go); // 0
        spaces.push(BrownProperty::mediterranean_ave().as_space()); // 1
        spaces.push(Space::CommunityChest); // 2
        spaces.push(BrownProperty::baltic_ave().as_space()); // 3
        spaces.push(Space::IncomeTax); // 4
        spaces.push(Railroads::reading().as_space()); // 5
        spaces.push(LightBlueProperty::oriental_ave().as_space()); // 6
        spaces.push(Space::Chance); // 7
        spaces.push(LightBlueProperty::vermont_ave().as_space()); // 8
        spaces.push(LightBlueProperty::connecticut_ave().as_space()); // 9
                                                                      // Left 10-19
        spaces.push(Space::Jail); // 10
        spaces.push(PinkProperty::st_charles_place().as_space()); // 11
        spaces.push(Utilities::electric_company().as_space()); // 12
        spaces.push(PinkProperty::states_ave().as_space()); // 13
        spaces.push(PinkProperty::virginia_ave().as_space()); // 14
        spaces.push(Railroads::pennsylvania().as_space()); // 15
        spaces.push(OrangeProperty::st_james_place().as_space()); // 16
        spaces.push(Space::CommunityChest); // 17
        spaces.push(OrangeProperty::tennessee_ave().as_space()); // 18
        spaces.push(OrangeProperty::new_york_ave().as_space()); // 19
                                                                // Top 20-29
        spaces.push(Space::FreeParking); // 20
        spaces.push(RedProperty::kentucky_ave().as_space()); // 21
        spaces.push(Space::Chance); // 22
        spaces.push(RedProperty::indiana_ave().as_space()); // 23
        spaces.push(RedProperty::illinois_ave().as_space()); // 24
        spaces.push(Railroads::bo().as_space()); // 25
        spaces.push(YellowProperty::atlantic_ave().as_space()); // 26
        spaces.push(YellowProperty::ventnor_ave().as_space()); // 27
        spaces.push(Utilities::water_works().as_space()); // 28
        spaces.push(YellowProperty::marvin_gardens().as_space()); // 29
                                                                  // Right 30-39
        spaces.push(Space::GoToJail); // 30
        spaces.push(GreenProperty::pacific_ave().as_space()); // 31
        spaces.push(GreenProperty::north_carolina_ave().as_space()); // 32
        spaces.push(Space::CommunityChest); // 33
        spaces.push(GreenProperty::pennsylvania_ave().as_space()); // 34
        spaces.push(Railroads::shortline().as_space()); // 35
        spaces.push(Space::Chance); // 36
        spaces.push(BlueProperty::park_place().as_space()); // 37
        spaces.push(Space::LuxuryTax); // 38
        spaces.push(BlueProperty::boardwalk().as_space()); // 39

        let players = vec![
            Player::new(1),
            Player::new(2),
            Player::new(3),
            Player::new(4),
        ];

        Board { spaces, players }
    }
    pub fn roll_player_dice(&mut self, index: usize) {
        self.players.get_mut(index).unwrap().roll_dice();
    }
    pub fn player_turn(&mut self, index: usize) {
        self.roll_player_dice(index);
        let player = self.players.get(index).unwrap();
        let position = player.position;
        let space_landed_on = &mut self.spaces[position as usize];
        match space_landed_on {
            Space::Property(mut properties) => {
                println!("property landed on's state: {:?}", properties.is_for_sale());
                if properties.is_for_sale() {
                    println!("Player {:?} bought: {:?}", player.player_number, properties);
                    let player = self.players.get_mut(index).unwrap();
                    properties.buy_property(player);
                    // testing
                    let player_properties = &player.properties;
                    println!(
                        "Player {:?} properties: {:?}",
                        player.player_number, player_properties
                    );
                } else {
                    let owner = properties.get_owner(&self);
                    if let Some(owner) = owner {
                        println!(
                            "Player {:?} has landed on {:?}'s property",
                            player.player_number, owner
                        );
                    }
                }
            }
            Space::Chance => {
                println!("Player {:?} has landed on Chance!", player.player_number)
            }
            Space::CommunityChest => {
                println!(
                    "Player {:?} has landed on Community Chest!",
                    player.player_number
                )
            }
            Space::IncomeTax => {
                println!("Player {:?} has landed on IncomeTax!", player.player_number)
            }
            Space::LuxuryTax => {
                println!("Player {:?} has farted", player.player_number)
            }
            Space::Go => {
                println!("Player {:?} has  pooped", player.player_number)
            }
            Space::GoToJail => {
                println!(
                    "Player {:?} has landed on Go To Jail Bitch!",
                    player.player_number
                )
            }
            Space::Jail => {
                println!(
                    "Player {:?} has landed on Jail (just passing)",
                    player.player_number
                )
            }
            Space::FreeParking => {
                println!(
                    "Player {:?} has landed on Free Parking",
                    player.player_number
                )
            }
        }
    }
}

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
