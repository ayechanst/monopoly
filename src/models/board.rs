use super::properties::{
    BlueProperty, BrownProperty, GreenProperty, LightBlueProperty, OrangeProperty, PinkProperty,
    RedProperty, Utilities, YellowProperty,
};
use crate::models::spaces::Space;

#[derive(Debug)]
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
        spaces.push(Space::CommunityChest);
        spaces.push(BrownProperty::baltic_ave().as_space());
        // Light Blue
        spaces.push(LightBlueProperty::oriental_ave().as_space());
        spaces.push(Space::Chance);
        spaces.push(LightBlueProperty::vermont_ave().as_space());
        spaces.push(LightBlueProperty::connecticut_ave().as_space());
        spaces.push(Space::Jail);
        // Pink
        spaces.push(PinkProperty::st_charles_place().as_space());
        spaces.push(Utilities::electric_company().as_space());
        spaces.push(PinkProperty::states_ave().as_space());
        spaces.push(PinkProperty::virginia_ave().as_space());
        // Orange
        spaces.push(OrangeProperty::st_james_place().as_space());
        spaces.push(OrangeProperty::tennessee_ave().as_space());
        spaces.push(OrangeProperty::new_york_ave().as_space());
        // Red
        spaces.push(RedProperty::kentucky_ave().as_space());
        spaces.push(RedProperty::indiana_ave().as_space());
        spaces.push(RedProperty::illinois_ave().as_space());
        // Yellow
        spaces.push(YellowProperty::atlantic_ave().as_space());
        spaces.push(YellowProperty::ventnor_ave().as_space());
        spaces.push(YellowProperty::marvin_gardens().as_space());
        // Green
        spaces.push(GreenProperty::pacific_ave().as_space());
        spaces.push(GreenProperty::north_carolina_ave().as_space());
        spaces.push(GreenProperty::pennsylvania_ave().as_space());
        // Blue
        spaces.push(BlueProperty::park_place().as_space());
        spaces.push(BlueProperty::boardwalk().as_space());

        Board { spaces }
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
