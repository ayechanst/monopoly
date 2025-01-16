// use super::{
//     player::Player,
//     spaces::{
//         properties::{
//             colored_properties::{
//                 colored_properties::ColoredProperties, BlueProperty, BrownProperty, GreenProperty,
//                 LightBlueProperty, OrangeProperty, PinkProperty, RedProperty, YellowProperty,
//             },
//             properties::Properties,
//             railroad::Railroads,
//             utility::Utilities,
//         },
//         space::Space,
//     },
// };
use crate::models::spaces::space::Space;
use crate::{
    models::cards::chance::Chance,
    utils::{
        debug_helpers::{debug_buy_property, debug_property, debug_rent},
        prompts::prompt_player,
    },
};
use bevy::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// pub type PlayerRef = Rc<RefCell<Player>>;
pub type SpaceRef = Rc<RefCell<Space>>;

use super::sprite_builder::sprite_color::make_color;
use super::sprite_builder::sprite_text::make_text;

pub fn make_sprite(
    // space_ref: &Space,
    // space_ref: Ref<'_, Space>,
    space_ref: Rc<RefCell<Space>>,
    commands: &mut Commands,
    grid_size: f32,
    scale_factor: f32,
) -> Entity {
    // let space_ref = space;
    let fill_color = make_color(&space_ref.borrow());
    // let text = make_text(&space_ref.borrow());
    let border_size = Vec2::splat(grid_size * scale_factor);
    let inner_size = border_size * 0.9;
    // border
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(border_size),
            ..default()
        },
        ..default()
    });
    // text

    // commands.spawn(Text2dBundle {
    //     text: Text::from_section(
    //         text,
    //         TextStyle {
    //             font: Default::default(),
    //             font_size: grid_size * 0.5,
    //             color: Color::srgb(1.0, 1.0, 1.0),
    //             ..default()
    //         },
    //     ),
    //     ..default()
    // });

    // fill color
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: fill_color,
                custom_size: Some(inner_size),
                ..default()
            },
            ..default()
        })
        .id()
}
