use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;

use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sprite Picking Experiment".to_string(),
                resolution: WindowResolution::new(800.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, (setup_camera, setup_sprites))
        // .add_systems(Update, got_clicked)
        .run();
}

#[derive(Component)]
struct Name(String);

fn setup_sprites(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name("Left".to_string()),
        On::<Pointer<Click>>::target_component_mut::<Name>(|_, name| {
            info!("You clicked on {}", name.0)
        }),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.0)).into(),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name("Middle".to_string()),
        On::<Pointer<Click>>::run(got_clicked),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.0)).into(),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name("Right".to_string()),
        On::<Pointer<Click>>::target_component_mut::<Name>(|_, name| {
            info!("You clicked on {}", name.0)
        }),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.0)).into(),
            material: materials.add(Color::GREEN),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn got_clicked(click: Listener<Pointer<Click>>, query: Query<&Name>) {
    if let Ok(name) = query.get(click.target()) {
        info!("You clicked on {}", name.0)
    } else {
        info!("Got click, but no name")
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
