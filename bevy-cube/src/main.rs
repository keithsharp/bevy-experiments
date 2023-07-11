use bevy::{prelude::*, window::WindowResolution};

use std::f32::consts::TAU;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Cube Experiment".to_string(),
                resolution: WindowResolution::new(1600.0, 1200.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let material = materials.add(StandardMaterial {
        base_color: Color::PINK,
        ..default()
    });

    // Create a Cube
    commands
        .spawn(PbrBundle {
            mesh: mesh,
            material: material,
            ..default()
        })
        .insert(Rotatable { speed: 0.3 });

    // Create a Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        ..Default::default()
    });
}

fn rotate_cube(timer: Res<Time>, mut cubes: Query<(&mut Transform, &Rotatable)>) {
    for (mut transform, cube) in cubes.iter_mut() {
        let rotation_change = Quat::from_rotation_y(TAU * cube.speed * timer.delta_seconds());
        transform.rotate(rotation_change);
    }
}
