use bevy::math::primitives::Cuboid;
use bevy::math::primitives::Plane3d;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Fly Cam Experiment".to_string(),
                resolution: WindowResolution::new(1600.0, 1200.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: mesh,
        material: material,
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    let mesh = meshes.add(Mesh::from(Plane3d::new(Vec3::Y)));
    let material = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: mesh,
        material: material,
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        ..Default::default()
    });
}
