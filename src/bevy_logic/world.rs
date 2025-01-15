use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_camera, spawn_floor));
    }
}

// fn spawn_board()
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
        transform: Transform::from_xyz(0.0, 30.5, 0.0).looking_at(Vec3::ZERO, Vec3::X),
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
