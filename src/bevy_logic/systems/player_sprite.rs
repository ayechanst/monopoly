use crate::bevy_logic::player_components::Position;
use bevy::prelude::*;

pub fn attatch_sprite(
    mut commands: Commands,
    query: Query<Entity, (With<Position>, Without<Sprite>)>,
) {
}
