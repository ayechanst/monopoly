use crate::bevy_logic::board_and_player::Player;
use bevy::prelude::*;

use super::make_space::make_text_bundle;

pub fn make_player(
    // player: Res<Player>,
    player: &Player,
    commands: &mut Commands,
    grid_size: f32,
    scale_factor: f32,
) -> Entity {
    // let player_size = grid_size / 4.0;
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(
                    (grid_size * scale_factor) / 2.0,
                    (grid_size * scale_factor) / 2.0,
                    // player_size * scale_factor,
                    // player_size * scale_factor,
                )),
                ..Default::default()
            },
            transform: Transform::default(),
            ..Default::default()
        })
        .id();
    let text_and_size = (&*player.number.to_string(), 0.2);
    // let text_and_size = make_text_bundle(grid_size, )
    commands
        .entity(entity)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            parent.spawn(make_text_bundle(grid_size, text_and_size));
        });
    entity
}
