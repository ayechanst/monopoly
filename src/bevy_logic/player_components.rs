use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerNumber(pub u32);

#[derive(Component)]
pub struct Balance(pub u32);

#[derive(Component)]
pub struct Position(pub (f32, f32));

#[derive(Component)]
pub struct Offset(pub (f32, f32));

#[derive(Component)]
pub struct Properties(pub Vec<String>);
