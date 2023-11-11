use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::setup::{CellTextures, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PlayfieldSize(IVec2::new(10, 24)))
            .add_systems(OnEnter(GameState::InGame), spawn_piece_if_necessary)
            .add_systems(
                Update,
                drop_current_piece.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                update_sprite.run_if(in_state(GameState::InGame)),
            );
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
                position: IVec2::new(5, 24),
                piece_type: PieceType::J,
            },
        ));
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

enum PieceType {
    J,
    L,
    S,
    Z,
    T,
}

#[derive(Component)]
struct Piece {
    position: IVec2,
    piece_type: PieceType,
}

fn drop_current_piece(time: Res<Time>, mut timer: ResMut<StepTimer>, mut query: Query<&mut Piece>) {
    let mut piece = query.single_mut();
    if timer.0.tick(time.delta()).just_finished() {
        piece.position.y = piece.position.y - 1;
        info!("{}", piece.position);
    }
}

fn update_sprite(
    mut commands: Commands,
    mut query: Query<(&Piece, &mut Transform)>,
    playfield: Res<PlayfieldSize>,
    mut windows: Query<&mut Window>,
    cell_textures: Res<CellTextures>,
) {
    let (piece, mut transform) = query.single_mut();
    let size = &playfield.0;
    let resolution = &windows.single_mut().resolution;

    let resolution = Vec2::new(resolution.width(), resolution.height());

    let padded_size = 0.75 * resolution;

    let cell_sizes = padded_size / size.as_vec2();

    let cell_size = cell_sizes.min_element();

    let grid_size = cell_size * size.as_vec2();

    let position = cell_size * piece.position.as_vec2() - 0.5 * grid_size;
    let position = position.extend(0.0);

    let scale = cell_size / cell_textures.size;
    *transform = transform
        .with_translation(position)
        .with_scale(Vec2::splat(scale).extend(1.0));
}
