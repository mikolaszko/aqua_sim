#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (hello_world_system, (update_people, greet_people).chain()),
        )
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("John Smith".to_string())));
    commands.spawn((Person, Name("Joanna Smith".to_string())));
    commands.spawn((Person, Name("Joline Smith".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0)
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Joanna Smith" {
            name.0 = "Joanna Not Smith".to_string();
            break;
        }
    }
}

fn hello_world_system() {
    println!("Hello, world!");
}
