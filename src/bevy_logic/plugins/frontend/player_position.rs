use super::frontend_plugin::GridSize;
use crate::bevy_logic::player_components::Position;
use bevy::prelude::*;

pub fn player_position(mut query: Query<(&mut Transform, &Position)>, grid_size: Res<GridSize>) {
    for (mut transform, position) in query.iter_mut() {
        let (x, y) = position.0;
        transform.translation =
            Vec3::new(x * grid_size.0, y * grid_size.0, transform.translation.z);
    }
}
