use crate::models::spaces::space::Space;
use bevy::prelude::*;

pub fn make_space(
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
