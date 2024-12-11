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
use std::cell::RefCell;
use std::rc::Rc;

pub type PlayerRef = Rc<RefCell<Player>>;
pub type SpaceRef = Rc<RefCell<Space>>;

#[derive(Debug)]
pub struct Board {
    pub spaces: Vec<SpaceRef>,
    pub players: Vec<PlayerRef>,
}

impl Board {
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        // Bottom 0-9
        spaces.push(Rc::new(RefCell::new(Space::Go)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Brown(
                BrownProperty::mediterranean_ave(),
            )),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Brown(BrownProperty::baltic_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::IncomeTax)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::reading()),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::oriental_ave(),
            )),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Chance)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::vermont_ave(),
            )),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::LightBlue(
                LightBlueProperty::connecticut_ave(),
            )),
        ))));
        // Left 10-19
        spaces.push(Rc::new(RefCell::new(Space::Jail))); // 10
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::st_charles_place())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(Properties::Utility(
            Utilities::electric_company(),
        )))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::states_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Pink(PinkProperty::virginia_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::pennsylvania()),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Orange(OrangeProperty::st_james_place()),
            ),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Orange(OrangeProperty::tennessee_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Orange(OrangeProperty::new_york_ave())),
        ))));
        // Top 20-29
        spaces.push(Rc::new(RefCell::new(Space::FreeParking))); // 20
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::kentucky_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Chance))); // 22
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::indiana_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Red(RedProperty::illinois_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::bo()),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Yellow(YellowProperty::atlantic_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Yellow(YellowProperty::ventnor_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(Properties::Utility(
            Utilities::water_works(),
        )))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Yellow(YellowProperty::marvin_gardens()),
            ),
        ))));
        // Right 30-39
        spaces.push(Rc::new(RefCell::new(Space::GoToJail)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Green(GreenProperty::pacific_ave())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Green(
                GreenProperty::north_carolina_ave(),
            )),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::CommunityChest)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(
                ColoredProperties::Green(GreenProperty::pennsylvania_ave()),
            ),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::Railroad(Railroads::shortline()),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::Chance)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::park_place())),
        ))));
        spaces.push(Rc::new(RefCell::new(Space::LuxuryTax)));
        spaces.push(Rc::new(RefCell::new(Space::Property(
            Properties::ColoredProperty(ColoredProperties::Blue(BlueProperty::boardwalk())),
        ))));

        let players = vec![
            Rc::new(RefCell::new(Player::new(1))),
            Rc::new(RefCell::new(Player::new(2))),
            Rc::new(RefCell::new(Player::new(3))),
            Rc::new(RefCell::new(Player::new(4))),
        ];

        Board { spaces, players }
    }
    pub fn roll_player_dice(&self, index: usize) {
        let mut player = self.players[index].borrow_mut();
        player.roll_dice();
    }
    pub fn get_position(&self, index: usize) -> usize {
        (self.players[index].borrow().position) as usize
    }
    pub fn player_turn(&mut self, index: usize) {
        self.roll_player_dice(index);
        let position = self.get_position(index);
        let mut space_landed_on = self.spaces[position].borrow_mut();
        let player_ref = self.players[index].clone();
        match &mut *space_landed_on {
            Space::Property(properties) => {
                if properties.is_for_sale() {
                    properties.buy_property(player_ref.clone());
                    println!(
                        "Player {:?} bought: {:?}, and has {:?} money left",
                        player_ref.borrow().player_number,
                        properties,
                        player_ref.borrow().money
                    );
                } else {
                    let renter_initial_balance = player_ref.borrow().money;
                    if let Some(owner) = properties.get_owner(self) {
                        let owner_initial_balance = owner.borrow().money;
                        properties.pay_rent(player_ref.clone(), self); // PAYING RENT HERE
                        println!(
                            "BOOM! Player {:?} landed on {:?}'s property",
                            player_ref.borrow().player_number,
                            owner.borrow().player_number,
                        );
                        println!("renter og balance: {:?}", renter_initial_balance);
                        println!("renter balance after rent: {:?}", player_ref.borrow().money);
                        println!("owner og balance: {:?}", owner_initial_balance);
                        println!("owner balance after rent: {:?}", owner.borrow().money);
                    } else {
                        println!("owner not found");
                    }
                }
            }
            Space::Chance => {
                println!(
                    "Player {:?} has landed on Chance!",
                    player_ref.borrow().player_number
                )
            }
            Space::CommunityChest => {
                println!(
                    "Player {:?} has landed on Community Chest!",
                    player_ref.borrow().player_number
                )
            }
            Space::IncomeTax => {
                println!(
                    "Player {:?} has landed on IncomeTax!",
                    player_ref.borrow().player_number
                )
            }
            Space::LuxuryTax => {
                println!("Player {:?} has farted", player_ref.borrow().player_number)
            }
            Space::Go => {
                println!("Player {:?} has  pooped", player_ref.borrow().player_number)
            }
            Space::GoToJail => {
                println!(
                    "Player {:?} has landed on Go To Jail Bitch!",
                    player_ref.borrow().player_number
                )
            }
            Space::Jail => {
                println!(
                    "Player {:?} has landed on Jail (just passing)",
                    player_ref.borrow().player_number
                )
            }
            Space::FreeParking => {
                println!(
                    "Player {:?} has landed on Free Parking",
                    player_ref.borrow().player_number
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
