use crate::bevy_logic::board_and_player::Player;
use bevy::{gizmos::grid, prelude::*};
use bevy_prototype_lyon::shapes;

use super::make_space::make_text_bundle;

pub fn make_player(
    player: &Player,
    commands: &mut Commands,
    grid_size: f32,
    scale_factor: f32,
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
            parent.spawn(make_text_bundle(grid_size, text_and_size));
            parent.spawn(make_shape_and_color_bundle(grid_size, scale_factor));
        });
    entity
}

// pub fn make_shape_bundle(grid_size: f32, scale_factor: f32) -> SpriteBundle {
//     let radius = (grid_size * scale_factor) / 2.0;
//     let circle = shapes::Circle {
//         radius,
//         ..default()
//     };
//     circle
// }

pub fn make_shape_and_color_bundle(grid_size: f32, scale_factor: f32) -> SpriteBundle {
    // let radius = (grid_size * scale_factor) / 2.0;
    // let circle = shapes::Circle {
    //     radius,
    //     ..default()
    // };
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
