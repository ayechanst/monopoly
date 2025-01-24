use crate::bevy_logic::{
    player_components::{Offset, Position},
    plugins::frontend_plugin::{GridSize, ScaleFactor},
};
use bevy::prelude::*;
pub fn player_position(
    mut query: Query<(&mut Transform, &Position, &Offset)>,
    grid_size: Res<GridSize>,
    scale_factor: Res<ScaleFactor>,
) {
    for (mut transform, position, offset) in query.iter_mut() {
        println!(
            "Player Position: ({}, {}), Offset: ({}, {}), Grid Size: {}",
            position.0 .0, position.0 .1, offset.0 .0, offset.0 .1, grid_size.0
        );
        println!("doing player_position shit");
        let (x, y) = position.0;
        let (x_offset, y_offset) = offset.0;
        transform.translation = Vec3::new(
            x * grid_size.0 + x_offset,
            y * grid_size.0 + y_offset,
            transform.translation.z,
        );
        println!("Updated Transform: {:?}", transform.translation);
    }
}
