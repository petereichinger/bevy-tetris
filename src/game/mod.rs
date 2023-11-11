mod render;

use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::setup::{CellBackground, CellTextures, GameState};

use self::render::RenderPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PlayfieldSize(IVec2::new(10, 24)))
            .register_type::<Piece>()
            .add_systems(OnEnter(GameState::InGame), (spawn_piece_if_necessary))
            .add_systems(
                Update,
                drop_current_piece.run_if(in_state(GameState::InGame)),
            )
            .add_plugins(RenderPlugin);
    }
}

#[derive(Resource)]
struct PlayfieldSize(IVec2);

fn spawn_piece_if_necessary(
    mut commands: Commands,
    query: Query<&Piece>,
    cell_textures: Res<CellTextures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas = cell_textures.atlas.clone();
    let sprite = TextureAtlasSprite {
        color: Color::ORANGE_RED,
        index: 1,
        ..default()
    };
    if let Err(QuerySingleError::NoEntities(_)) = query.get_single() {
        commands.spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas,
                ..Default::default()
            },
            Piece {
                position: IVec2::new(5, 23),
                piece_type: PieceType::J,
            },
        ));
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Reflect)]
enum PieceType {
    J,
    L,
    S,
    Z,
    T,
}

#[derive(Reflect, Component)]
struct Piece {
    position: IVec2,
    piece_type: PieceType,
}

fn drop_current_piece(time: Res<Time>, mut timer: ResMut<StepTimer>, mut query: Query<&mut Piece>) {
    let mut piece = query.single_mut();
    if timer.0.tick(time.delta()).just_finished() {
        piece.position.y = piece.position.y - 1;
        // info!("{}", piece.position);
    }
}
