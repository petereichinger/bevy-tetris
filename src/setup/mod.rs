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
            .add_systems(OnEnter(GameState::Setup), (load_sprites, setup_camera))
            .add_systems(
                Update,
                (wait_for_loading).run_if(in_state(GameState::Setup)),
            )
            .add_systems(OnExit(GameState::Setup), finished_loading);
    }
}

#[derive(Resource)]
pub struct CellTextures(pub Handle<TextureAtlas>);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_handle = asset_server.load("sprites/cells.png");
    let texture_atlas =
        TextureAtlas::from_grid(sprite_handle, Vec2::new(32.0, 32.0), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(CellTextures(texture_atlas_handle));
}

fn wait_for_loading(
    mut commands: Commands,
    mut texture_atlas_events: EventReader<AssetEvent<TextureAtlas>>,
    cell_textures: Res<CellTextures>,
) {
    let atlas_id = cell_textures.0.id();
    for &event in texture_atlas_events.iter() {
        if let AssetEvent::Added { id } = event {
            if id == atlas_id {
                commands.insert_resource(NextState(Some(GameState::InGame)));
            }
        }
    }
}
fn finished_loading() {
    info!("finished loading");
}
