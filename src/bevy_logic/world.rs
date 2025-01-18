use crate::{
    bevy_logic::sprite_builder::make_space::{self, make_space},
    models::board::Board,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, (spawn_light, spawn_camera, spawn_board));
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, (spawn_camera, spawn_light))
            // .add_systems(Startup, (spawn_light, spawn_board))
            .add_systems(Update, spawn_board_button);
    }
}

fn spawn_board_button(commands: Commands, mut egui_ctx: EguiContexts, mut spawned: Local<bool>) {
    egui::Window::new("Spawn Board").show(egui_ctx.ctx_mut(), |ui| {
        if ui.button("Spawn Board").clicked() && !*spawned {
            *spawned = true;
            spawn_board(commands);
        }
    });
}

pub fn spawn_board(mut commands: Commands) {
    let board = Board::new();
    let grid_size = 600.0;
    let scale_factor = 1.0;
    for (index, space_ref) in board.spaces.iter().cloned().enumerate() {
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
        let space_entity = make_space(space_ref, &mut commands, grid_size, scale_factor);
        println!("Entity: {:?} number: {:?}", space_entity, i);
        commands.entity(space_entity).insert(Transform::from_xyz(
            x * grid_size,
            y * grid_size,
            0.0,
        ));
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(12.0),
            ..default()
        },
        ..default()
    });
}

pub fn spawn_light(mut commands: Commands) {
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
