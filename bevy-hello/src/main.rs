use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Keith Sharp".to_string())));
    commands.spawn((Person, Name("Cart Anderson".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    people: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in people.iter() {
            println!("Hello, {}!", name.0)
        }
    }
}
