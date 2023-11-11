mod game;
mod setup;

use bevy::prelude::*;

fn main() {
    for (key, value) in std::env::vars().into_iter() {
        println!("{} {}", key, value);
    }
    App::new()
        .add_plugins((DefaultPlugins, setup::SetupPlugin, game::GamePlugin))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
