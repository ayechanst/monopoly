use bevy::prelude::*;

// #[derive(Default, Resource, Debug)]
// pub struct Player {
//     pub number: u32,
//     pub balance: u32,
//     pub position: (f32, f32),
//     pub offset: (f32, f32),
//     pub properties: Vec<String>,
// }
#[derive(Component)]
pub struct Player;

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
