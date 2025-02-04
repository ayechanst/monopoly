use bevy::prelude::*;

// #[derive(Component)]
// pub struct Player;

#[derive(Component, Debug)]
pub struct FrontendPlayer {
    pub player_number: u8,
    pub active_player: bool,
    pub balance: u32,
    pub position: (f32, f32),
    pub offset: (f32, f32),
    pub properties: Vec<String>,
}

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
