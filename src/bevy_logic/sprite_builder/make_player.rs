use super::make_space::make_text_bundle;
use crate::bevy_logic::plugins::frontend_plugin::{GridSize, ScaleFactor};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: Sprite,
    text: Text2dBundle,
}

pub fn make_player(
    player_number: usize,
    grid_size: f32,
    scale_factor: f32,
    offset: (f32, f32),
) -> PlayerBundle {
    let text_and_size = (&*player_number.to_string(), 0.2);
    let sprite = make_shape_and_color_bundle(grid_size, scale_factor, offset);
    let text = make_text_bundle(grid_size, text_and_size);
    PlayerBundle {
        sprite: sprite.sprite,
        text,
    }
}

pub fn make_shape_and_color_bundle(
    grid_size: f32,
    scale_factor: f32,
    offset: (f32, f32),
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(
                (grid_size * scale_factor) / 2.0,
                (grid_size * scale_factor) / 2.0,
            )),
            ..default()
        },
        // transform: Transform::default(),
        transform: Transform::from_xyz(
            (offset.0 + 5.0) * grid_size,
            (offset.1 - 5.0) * grid_size,
            0.5,
        ),
        ..default()
    }
}
