use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    SetupGame,
    InGame,
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn name(&self) -> &str {
        "setup_tetris"
    }

    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), (load_sprites, setup_camera))
            .add_systems(
                Update,
                (wait_for_loading).run_if(in_state(GameState::Loading)),
            );
    }
}

#[derive(Resource)]
pub struct CellTextures {
    pub atlas: Handle<TextureAtlas>,
    pub size: f32,
    pub count: usize,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("sprites/cells.png");

    let tile_size = Vec2::new(32.0, 32.0);
    let texture_atlas = TextureAtlas::from_grid(texture, tile_size, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(CellTextures {
        atlas: texture_atlas_handle,
        size: tile_size.x,
        count: 16,
    });
}

fn wait_for_loading(
    mut commands: Commands,
    mut texture_atlas_events: EventReader<AssetEvent<TextureAtlas>>,
    cell_textures: Res<CellTextures>,
) {
    let atlas_id = cell_textures.atlas.id();
    for &event in texture_atlas_events.read() {
        if let AssetEvent::Added { id } = event {
            if id == atlas_id {
                commands.insert_resource(NextState(Some(GameState::SetupGame)));
            }
        }
    }
}
