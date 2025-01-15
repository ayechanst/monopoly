use bevy::prelude::*;

use crate::models::board::Board;

use super::helpers::make_material;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, (spawn_light, spawn_camera, spawn_floor));
        app.add_systems(Startup, (spawn_light, spawn_camera, spawn_board));
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut board = Board::new();
    let spaces = board.spaces;
    for (i, space_ref) in spaces.iter().enumerate() {
        let space = space_ref.borrow_mut();
        let material = make_material(&space, &mut materials);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0))),
            material,
            transform: Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0),
            ..default()
        });
    }
}
// all same mesh, but different materials

fn spawn_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let floor = PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
        ..default()
    };
    commands.spawn(floor);
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 80.5, 20.0).looking_at(Vec3::ZERO, Vec3::X),
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
