use bevy::prelude::*;

use crate::models::board::Board;

use super::helpers::make_sprite;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_camera, spawn_board));
    }
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new();
    let grid_size = 5.0;
    let scale_factor = 1.0;
    for (index, space) in board.spaces.iter().enumerate() {
        let i = index as f32;
        let (x, y) = if i < 10.0 {
            (5.0 - i, -5.0)
        } else if i < 20.0 {
            (-5.0, i - 15.0)
        } else if i < 30.0 {
            (i - 25.0, 5.0)
        } else {
            (5.0, -(i - 35.0))
        };
        // let space = space_ref.borrow_mut();
        let space_ref = space.clone();
        let entity = make_sprite(space_ref, &mut commands, grid_size, scale_factor);
        commands
            .entity(entity)
            .insert(Transform::from_xyz(x * grid_size, y * grid_size, 0.0));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(0.10),
            ..default()
        },
        ..default()
    });
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}
