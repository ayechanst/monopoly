use bevy::prelude::*;

use crate::bevy_logic::{
    player_components::Player,
    plugins::frontend_plugin::{GridSize, ScaleFactor},
};

use super::make_space::make_text_bundle;

pub fn make_player(
    player: &Player,
    commands: &mut Commands,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
    // we want this to just return a sprite bundle
) -> Entity {
    let text_and_size = (&*player.number.to_string(), 0.2);
    let entity = commands.spawn_empty().id();
    // let text_and_size = make_text_bundle(grid_size, )
    commands
        .entity(entity)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            parent.spawn(make_text_bundle(grid_size.0, text_and_size));
            parent.spawn(make_shape_and_color_bundle(grid_size.0, scale_factor.0));
        });
    entity
}

pub fn make_shape_and_color_bundle(grid_size: f32, scale_factor: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(
                (grid_size * scale_factor) / 2.0,
                (grid_size * scale_factor) / 2.0,
            )),
            ..default()
        },
        transform: Transform::default(),
        ..default()
    }
}
