mod piece_order;
mod piece_types;
mod playfield;
mod render;
mod rotation;

use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;

use crate::{game::playfield::CheckRotationResult, setup::GameState};

use self::{
    piece_order::{create_piece_order, PieceOrder},
    piece_types::PieceType,
    playfield::{Playfield, PlayfieldSize},
    render::RenderPlugin,
    rotation::Rotation,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(PlayfieldSize([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(
                OnEnter(GameState::SetupGame),
                (spawn_playfield, create_piece_order, move_to_ingame),
            )
            .add_systems(
                Update,
                (spawn_piece, move_piece).run_if(in_state(GameState::InGame)),
            )
            .add_plugins(RenderPlugin);
    }
}

fn spawn_playfield(mut commands: Commands, playfield_size: Res<PlayfieldSize>) {
    commands.spawn((Name::new("Playfield"), Playfield::new(playfield_size.0)));
}

fn move_to_ingame(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameState::InGame)));
}

fn spawn_piece(
    piece: Query<&Piece>,
    mut commands: Commands,
    mut piece_order: ResMut<PieceOrder>,
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    if let Err(QuerySingleError::NoEntities(_)) = piece.get_single() {
        if piece_order.is_finished() {
            *piece_order = PieceOrder::new(rng);
        }

        commands.spawn((
            Name::new("Current Piece"),
            Piece::new(piece_order.next_piece().expect("Should not be empty")),
        ));
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Reflect, Component, Debug)]
pub struct Piece {
    position: IVec2,
    rotation: Rotation,
    piece_type: PieceType,
}

impl Piece {
    fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            position: IVec2::new(5, 23),
            rotation: default(),
        }
    }
}

fn move_piece(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut query: Query<(Entity, &mut Piece)>,
    mut playfield_query: Query<&mut Playfield>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok((entity, mut piece)) = query.get_single_mut() else {
        return;
    };
    let mut playfield = playfield_query.single_mut();
    if keys.just_pressed(KeyCode::Up) {
        use Rotation::*;

        let new_rotation = match piece.rotation {
            R0 => R90,
            R90 => R180,
            R180 => R270,
            R270 => R0,
        };

        let check_result = playfield.check_rotation(&Piece {
            rotation: new_rotation,
            ..*piece
        });

        if let CheckRotationResult::ValidWithOffset(offset) = check_result {
            *piece = Piece {
                rotation: new_rotation,
                position: piece.position + offset,
                ..*piece
            }
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        while playfield.check_move(&Piece {
            position: piece.position + IVec2::NEG_Y,
            ..*piece
        }) {
            piece.position += IVec2::NEG_Y;
        }
    }

    let direction = {
        if keys.just_pressed(KeyCode::Right) {
            Some(IVec2::X)
        } else if keys.just_pressed(KeyCode::Left) {
            Some(IVec2::NEG_X)
        } else if keys.just_pressed(KeyCode::Down) {
            Some(IVec2::NEG_Y)
        } else {
            None
        }
    };

    if let Some(direction) = direction {
        let new_pos = piece.position + direction;
        let move_possible = playfield.check_move(&Piece {
            position: new_pos,
            ..*piece
        });

        if move_possible {
            piece.position = new_pos;
        }
    }

    if timer.0.tick(time.delta()).just_finished() {
        let new_pos = piece.position - IVec2::Y;

        let move_possible = playfield.check_move(&Piece {
            position: new_pos,
            ..*piece
        });

        if move_possible {
            piece.position = new_pos;
        } else {
            commands.entity(entity).despawn_recursive();
            playfield.set_cells(&piece);
            playfield.clear_rows();
        }
    }
}
