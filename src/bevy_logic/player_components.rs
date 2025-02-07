use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FrontendPlayer {
    pub player_number: u8,
    pub active_player: bool,
    pub balance: u32,
    pub position: (f32, f32),
    pub offset: (f32, f32),
    pub properties: Vec<String>,
}
