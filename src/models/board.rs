use super::{
    player::Player,
    spaces::{
        properties::{
            colored_properties::{
                BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty,
                PinkProperty, RedProperty, YellowProperty,
            },
            railroad::Railroads,
            utility::Utilities,
        },
        space::Space,
    },
};
use std::cell::RefCell;
use std::rc::Rc;

pub type PlayerRef = Rc<RefCell<Player>>;
pub type SpaceRef = Rc<RefCell<Space>>;

#[derive(Debug)]
pub struct Board {
    // pub spaces: Vec<Space>,
    pub spaces: Vec<SpaceRef>,
    pub players: Vec<PlayerRef>,
}

impl Board {
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        // Bottom 0-9
        // spaces.push((Rc::new(RefCell::new(Space::Go); // 0
        spaces.push((Rc::new(RefCell::new(Space::Go))));
        spaces.push(Rc::new(RefCell::new(
            BrownProperty::mediterranean_ave().as_space(),
        ))); // 1
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest))); // 2
        spaces.push(Rc::new(RefCell::new(
            BrownProperty::baltic_ave().as_space(),
        ))); // 3
        spaces.push((Rc::new(RefCell::new(Space::IncomeTax)))); // 4
        spaces.push((Rc::new(RefCell::new(Railroads::reading().as_space())))); // 5
        spaces.push((Rc::new(RefCell::new(LightBlueProperty::oriental_ave().as_space())))); // 6
        spaces.push((Rc::new(RefCell::new(Space::Chance)))); // 7
        spaces.push((Rc::new(RefCell::new(LightBlueProperty::vermont_ave().as_space())))); // 8
        spaces.push(
            (Rc::new(RefCell::new(
                LightBlueProperty::connecticut_ave().as_space(),
            ))),
        ); // 9
           // Left 10-19
        spaces.push((Rc::new(RefCell::new(Space::Jail)))); // 10
        spaces.push((Rc::new(RefCell::new(PinkProperty::st_charles_place().as_space())))); // 11
        spaces.push((Rc::new(RefCell::new(Utilities::electric_company().as_space())))); // 12
        spaces.push((Rc::new(RefCell::new(PinkProperty::states_ave().as_space())))); // 13
        spaces.push((Rc::new(RefCell::new(PinkProperty::virginia_ave().as_space())))); // 14
        spaces.push(Rc::new(RefCell::new(Railroads::pennsylvania().as_space()))); // 15
        spaces.push(Rc::new(RefCell::new(
            OrangeProperty::st_james_place().as_space(),
        ))); // 16
        spaces.push((Rc::new(RefCell::new(Space::CommunityChest)))); // 17
        spaces.push(Rc::new(RefCell::new(
            OrangeProperty::tennessee_ave().as_space(),
        ))); // 18
        spaces.push((Rc::new(RefCell::new(OrangeProperty::new_york_ave().as_space())))); // 19
                                                                                         // Top 20-29
        spaces.push(Rc::new(RefCell::new(Space::FreeParking))); // 20
        spaces.push((Rc::new(RefCell::new(RedProperty::kentucky_ave().as_space())))); // 21
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 22
        spaces.push((Rc::new(RefCell::new(RedProperty::indiana_ave().as_space())))); // 23
        spaces.push((Rc::new(RefCell::new(RedProperty::illinois_ave().as_space())))); // 24
        spaces.push(Rc::new(RefCell::new(Railroads::bo().as_space()))); // 25
        spaces.push((Rc::new(RefCell::new(YellowProperty::atlantic_ave().as_space())))); // 26
        spaces.push(Rc::new(RefCell::new(
            YellowProperty::ventnor_ave().as_space(),
        ))); // 27
        spaces.push(Rc::new(RefCell::new(Utilities::water_works().as_space()))); // 28
        spaces.push(Rc::new(RefCell::new(
            YellowProperty::marvin_gardens().as_space(),
        ))); // 29
             // Right 30-39
        spaces.push(Rc::new(RefCell::new(Space::GoToJail))); // 30
        spaces.push(Rc::new(RefCell::new(
            GreenProperty::pacific_ave().as_space(),
        ))); // 31
        spaces.push(Rc::new(RefCell::new(
            GreenProperty::north_carolina_ave().as_space(),
        ))); // 32
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest))); // 33
        spaces.push(Rc::new(RefCell::new(
            GreenProperty::pennsylvania_ave().as_space(),
        ))); // 34
        spaces.push(Rc::new(RefCell::new(Railroads::shortline().as_space()))); // 35
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 36
        spaces.push(Rc::new(RefCell::new(BlueProperty::park_place().as_space()))); // 37
        spaces.push(Rc::new(RefCell::new(Space::LuxuryTax))); // 38
        spaces.push(Rc::new(RefCell::new(BlueProperty::boardwalk().as_space()))); // 39

        let players = vec![
            Rc::new(RefCell::new(Player::new(1))),
            Rc::new(RefCell::new(Player::new(2))),
            Rc::new(RefCell::new(Player::new(3))),
            Rc::new(RefCell::new(Player::new(4))),
        ];

        Board { spaces, players }
    }

    pub fn roll_player_dice(&self, index: usize) {
        let player = self.players[index].borrow_mut();
        player.clone().roll_dice();
    }

    pub fn player_turn(&mut self, index: usize) {
        // self.roll_player_dice(index);
        let player = self.players[index].borrow_mut();
        player.clone().roll_dice();
        // let position = player.borrow().position;
        let position = player.position;
        let space_landed_on = &mut *self.spaces[position as usize].borrow_mut();
        match space_landed_on {
            Space::Property(properties) => {
                println!("property landed on's state: {:?}", properties.is_for_sale());
                if properties.is_for_sale() {
                    println!("Player {:?} bought: {:?}", player.player_number, properties);
                    properties.buy_property(self.players[index].clone());
                } else {
                    let owner = properties.get_owner(self);
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
