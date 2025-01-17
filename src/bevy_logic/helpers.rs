use crate::models::spaces::space::Space;
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
    let inner_size = border_size * 0.92;
    let entity = commands.spawn_empty().id();
    commands
        .entity(entity)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent.spawn(make_border_bundle(border_size));
            parent.spawn(make_color_bundle(inner_size, fill_color));
            parent.spawn(make_text_bundle(grid_size, text.to_string()));
        });
    entity
}

pub fn make_border_bundle(border_size: Vec2) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(border_size),
            ..default()
        },
        ..default()
    }
}

pub fn make_color_bundle(inner_size: Vec2, fill_color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: fill_color,
            custom_size: Some(inner_size),
            ..default()
        },
        ..default()
    }
}

pub fn make_text_bundle(grid_size: f32, text: String) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: Default::default(),
                font_size: grid_size * 0.25,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        ),
        ..default()
    }
}
