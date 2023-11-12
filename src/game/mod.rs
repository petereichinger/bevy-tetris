mod playfield;
mod render;

use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::setup::{CellTextures, GameState};

use self::{
    playfield::{Cell, Playfield},
    render::RenderPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(Playfield::new([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(OnEnter(GameState::InGame), spawn_piece_if_necessary)
            .add_systems(
                Update,
                drop_current_piece.run_if(in_state(GameState::InGame)),
            )
            .add_plugins(RenderPlugin);
    }
}

fn spawn_piece_if_necessary(
    mut commands: Commands,
    query: Query<&Piece>,
    cell_textures: Res<CellTextures>,
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

#[derive(Reflect, PartialEq, Eq, Debug, Copy, Clone)]
pub enum PieceType {
    J,
    // L,
    // S,
    // Z,
    // T,
}

#[derive(Reflect, Component)]
struct Piece {
    position: IVec2,
    piece_type: PieceType,
}

fn drop_current_piece(
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut query: Query<&mut Piece>,
    mut playfield: ResMut<Playfield>,
) {
    let mut piece = query.single_mut();
    if timer.0.tick(time.delta()).just_finished() {
        let new_pos = piece.position - IVec2::Y;

        let move_possible = check_move(&mut playfield, new_pos);

        if move_possible {
            piece.position = new_pos;
        } else {
            let cell = playfield.get_mut(new_pos);

            if let Some(cell) = cell {
                *cell = Cell::Filled(PieceType::J);
            }
        }
        // piece.position.y -= 1;
        // info!("{}", piece.position);
    }
}

fn check_move(playfield: &mut Playfield, new_pos: IVec2) -> bool {
    let cell = playfield.get(new_pos);

    if let Some(Cell::Empty) = cell {
        true
    } else {
        false
    }
}
