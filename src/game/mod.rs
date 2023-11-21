mod cell_events;
mod piece_types;
mod playfield;
mod render;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::setup::{CellTextures, GameState};

use self::{
    cell_events::{CellEvent, EventType},
    piece_types::{get_random_piece_type, iter_cells, PieceType},
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

fn spawn_piece(
    mut commands: Commands,
    cell_textures: Res<CellTextures>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let texture_atlas = cell_textures.atlas.clone();
    let sprite = TextureAtlasSprite {
        color: Color::ORANGE_RED,
        index: 1,
        ..default()
    };

    let piece_type = get_random_piece_type(rng);
    commands.spawn((
        SpriteSheetBundle {
            sprite,
            texture_atlas,
            ..Default::default()
        },
        Piece {
            position: IVec2::new(5, 23),
            piece_type,
        },
    ));
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
    cell_textures: Res<CellTextures>,
    cell_events_writer: EventWriter<CellEvent>,
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    keys: Res<Input<KeyCode>>,
) {
    let (entity, mut piece) = query.single_mut();

    let direction = {
        if keys.just_pressed(KeyCode::K) {
            Some(IVec2::X)
        } else if keys.just_pressed(KeyCode::J) {
            Some(IVec2::NEG_X)
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
            spawn_piece(commands, cell_textures, rng);

            set_cells(&mut playfield, &piece, cell_events_writer);
        }
    }
}

fn check_move(playfield: &Playfield, piece: &Piece, new_pos: IVec2) -> bool {
    let all_free = iter_cells(new_pos, piece.piece_type).all(|p| {
        let cell = playfield.get(p);
        matches!(cell, Some(Cell::Empty))
    });

    all_free
}

fn set_cells(
    playfield: &mut Playfield,
    piece: &Piece,
    mut cell_events_writer: EventWriter<CellEvent>,
) {
    iter_cells(piece.position, piece.piece_type).for_each(|p| {
        let cell = playfield.get_mut(p);
        if let Some(cell) = cell {
            *cell = Cell::Filled(piece.piece_type);
            cell_events_writer.send(CellEvent {
                position: p,
                event_type: EventType::Added,
            });
        }
    });
}
