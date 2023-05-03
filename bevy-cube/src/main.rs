use bevy::prelude::*;

use std::f32::consts::PI;

const FULL_TURN: f32 = 2.0 * PI;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bevy Cube Experiment".to_string(),
            width: 1600.,
            height: 1200.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1. }));
    let material = materials.add(StandardMaterial {
        base_color: Color::PINK,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: mesh,
        material: material,
        ..Default::default()
    })
    .insert(Rotatable { speed: 0.3 });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(DirectionalLightBundle { ..Default::default() });
}

fn rotate_cube(timer: Res<Time>, mut cubes: Query<(&mut Transform, &Rotatable)>) {
    for (mut transform, cube) in cubes.iter_mut() {
        let rotation_change = Quat::from_rotation_y(FULL_TURN * cube.speed * timer.delta_seconds());
        transform.rotate(rotation_change);
    }
}