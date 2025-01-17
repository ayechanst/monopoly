use crate::models::spaces::space::{self, Space};
use bevy::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
pub type SpaceRef = Rc<RefCell<Space>>;
use super::sprite_builder::sprite_color::make_color;
use super::sprite_builder::sprite_text::make_text;

pub fn make_sprite(
    space_ref: SpaceRef,
    commands: &mut Commands,
    grid_size: f32,
    scale_factor: f32,
) -> Entity {
    let borrowed_space = space_ref.borrow();
    let fill_color = make_color(&borrowed_space);
    let text = make_text(&borrowed_space);

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

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: Default::default(),
                font_size: grid_size * 0.5,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ),
        ..default()
    });

    // fill color
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: fill_color,
                // color: fill_color.clone(),
                // color: fill_color.borrow(),
                custom_size: Some(inner_size),
                ..default()
            },
            ..default()
        })
        .id()
}
