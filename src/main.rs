mod game;
mod setup;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            setup::SetupPlugin,
            game::GamePlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
