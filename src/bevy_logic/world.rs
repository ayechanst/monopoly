use bevy::prelude::*;

use crate::models::board::Board;

use super::helpers::make_material;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_camera, spawn_board));
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let board = Board::new();
    let spaces = board.spaces;
    let grid_size = 2.0;
    for (index, space_ref) in spaces.iter().enumerate() {
        let i = index as f32;
        let space = space_ref.borrow_mut();
        let material = make_material(&space, &mut materials);

        let (x, z) = if i < 10.0 {
            (-5.0, -(i * grid_size))
        } else if i < 20.0 {
            ((i - 10.0) * grid_size, -10.0)
        } else if i < 30.0 {
            (5.0, (i - 20.0) * grid_size)
        } else {
            (-(i - 30.0) * grid_size, 5.0)
        };

        commands.spawn(PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0))),
            material,
            transform: Transform::from_xyz(x, 0.0, z),
            ..default()
        });
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 80.5, 0.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    };
    commands.spawn(camera);
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
