mod cell_events;
mod piece_types;
mod playfield;
mod render;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;

use crate::setup::GameState;

use self::{
    cell_events::CellEvent,
    piece_types::{get_random_piece_type, iter_cells_at, PieceType},
    playfield::{Cell, Playfield},
    render::RenderPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_event::<CellEvent>()
            .insert_resource(Playfield::new([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(OnEnter(GameState::InGame), spawn_piece)
            .add_systems(Update, (move_piece).run_if(in_state(GameState::InGame)))
            .add_plugins(RenderPlugin);
    }
}

fn spawn_piece(mut commands: Commands, rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    let piece_type = get_random_piece_type(rng);

    commands.spawn((Piece {
        position: IVec2::new(5, 23),
        piece_type,
    },));
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Reflect, Component)]
struct Piece {
    position: IVec2,
    piece_type: PieceType,
}

fn move_piece(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut query: Query<(Entity, &mut Piece)>,
    mut playfield: ResMut<Playfield>,
    cell_events_writer: EventWriter<CellEvent>,
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    keys: Res<Input<KeyCode>>,
) {
    let (entity, mut piece) = query.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        while check_move(&playfield, &piece, piece.position + IVec2::NEG_Y) {
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
        let move_possible = check_move(&playfield, &piece, new_pos);

        if move_possible {
            piece.position = new_pos;
        }
    }

    if timer.0.tick(time.delta()).just_finished() {
        let new_pos = piece.position - IVec2::Y;

        let move_possible = check_move(&playfield, &piece, new_pos);

        if move_possible {
            piece.position = new_pos;
        } else {
            commands.entity(entity).despawn_recursive();
            spawn_piece(commands, rng);

            set_cells(&mut playfield, &piece, cell_events_writer);
        }
    }
}

fn check_move(playfield: &Playfield, piece: &Piece, new_pos: IVec2) -> bool {
    let all_free = iter_cells_at(new_pos, piece.piece_type).all(|p| {
        let cell = playfield.get(p);
        matches!(cell, Some(Cell::Empty))
    });

    all_free
}

fn set_cells(
    playfield: &mut Playfield,
    Piece {
        position,
        piece_type,
    }: &Piece,
    mut cell_events_writer: EventWriter<CellEvent>,
) {
    iter_cells_at(*position, *piece_type).for_each(|p| {
        let cell = playfield.get_mut(p);
        if let Some(cell) = cell {
            *cell = Cell::Filled(*piece_type);
            cell_events_writer.send(CellEvent::Added {
                position: p,
                piece_type: *piece_type,
            });
        }
    });
}
