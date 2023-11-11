use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Setup,
    InGame,
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn name(&self) -> &str {
        "setup_tetris"
    }

    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Setup), load_sprites)
            .add_systems(
                Update,
                (wait_for_loading).run_if(in_state(GameState::Setup)),
            );
    }
}

#[derive(Resource)]
pub struct CellTextures {
    pub texture_atlas: Handle<TextureAtlas>,
}

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_handle = asset_server.load("sprites/cells.png");
    let texture_atlas = TextureAtlas::from_grid(
        sprite_handle,
        Vec2::new(53.0, 55.0),
        4,
        4,
        Some(Vec2::new(14.0, 12.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CellTextures {
        texture_atlas: texture_atlas_handle,
    });
}

fn wait_for_loading(mut texture_atlas_events: EventReader<AssetEvent<TextureAtlas>>) {
    for &event in texture_atlas_events.iter() {
        match event {
            AssetEvent::Added { id } => println!("added"),
            AssetEvent::Modified { id } => println!("modified"),
            AssetEvent::Removed { id } => println!("removed"),
            AssetEvent::LoadedWithDependencies { id } => println!("load with deps"),
        }
    }
}
