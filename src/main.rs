mod setup;

use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use setup::CellTextures;

use crate::setup::GameState;

fn main() {
    println!("{:?}", std::env::current_dir());
    App::new()
        .add_plugins((DefaultPlugins, setup::SetupPlugin))
        .insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(
            Update,
            drop_current_piece.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_piece_if_necessary)
        .run();
}

fn spawn_piece_if_necessary(
    mut commands: Commands,
    query: Query<&Piece>,
    cell_textures: Res<CellTextures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let texture_atlas = cell_textures.0.clone();
    let sprite = TextureAtlasSprite {
        color: Color::ORANGE_RED,
        index: 1,
        ..default()
    };
    if let Err(QuerySingleError::NoEntities(_)) = query.get_single() {
        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_xyz(5.0, 12.0, 0.0),
                sprite,
                texture_atlas,
                ..Default::default()
            },
            Piece {
                position: IVec2::new(5, 12),
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
        println!("{}", piece.position);
    }
}
