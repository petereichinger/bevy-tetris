mod game;
mod setup;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    // for (key, value) in std::env::vars().into_iter() {
    //     println!("{} {}", key, value);
    // }
    App::new()
        .add_plugins((DefaultPlugins, setup::SetupPlugin, game::GamePlugin))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
