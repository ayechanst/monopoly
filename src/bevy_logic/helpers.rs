use crate::models::spaces::space::Space;
use bevy::prelude::*;

pub fn make_material(
    space: &Space,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let base_color = match space {
        Space::Property(properties) => Color::srgb(0.0, 9.0, 0.0),
        Space::Chance => Color::srgb(0.9, 0.0, 0.0),
        Space::CommunityChest => Color::srgb(0.9, 0.0, 0.0),
        Space::LuxuryTax => Color::srgb(0.9, 0.0, 0.0),
        Space::IncomeTax => Color::srgb(0.9, 0.0, 0.0),
        Space::Go => Color::srgb(0.0, 0.0, 9.0),
        Space::GoToJail => Color::srgb(0.0, 0.0, 9.0),
        Space::Jail => Color::srgb(0.0, 0.0, 9.0),
        Space::FreeParking => Color::srgb(0.0, 0.0, 9.0),
    };
    materials.add(StandardMaterial {
        base_color,
        ..default()
    })

    // let space_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    // space_mesh
}

pub fn make_sprite(space: &Space, commands: &mut Commands, grid_size: f32) {
    let fill_color = match space {
        Space::Property(properties) => Color::srgb(0.0, 1.0, 0.0),
        Space::Chance | Space::CommunityChest | Space::IncomeTax | Space::LuxuryTax => {
            Color::srgb(1.0, 0.0, 0.0)
        }
        Space::Go | Space::GoToJail | Space::Jail | Space::FreeParking => {
            Color::srgb(0.0, 0.0, 1.0)
        }
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::splat(grid_size)),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: fill_color,
            custom_size: Some(Vec2::splat(grid_size * 0.9)),
            ..default()
        },
        ..default()
    });
}
