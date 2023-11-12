mod cell_events;
mod playfield;
mod render;

use bevy::prelude::*;

use crate::setup::{CellTextures, GameState};

use self::{
    cell_events::{CellEvent, EventType},
    playfield::{Cell, Playfield},
    render::RenderPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_event::<CellEvent>()
            .insert_resource(Playfield::new([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(OnEnter(GameState::InGame), spawn_piece)
            .add_systems(Update, (move_piece).run_if(in_state(GameState::InGame)))
            .add_plugins(RenderPlugin);
    }
}

fn spawn_piece(mut commands: Commands, cell_textures: Res<CellTextures>) {
    let texture_atlas = cell_textures.atlas.clone();
    let sprite = TextureAtlasSprite {
        color: Color::ORANGE_RED,
        index: 1,
        ..default()
    };
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

fn move_piece(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut query: Query<(Entity, &mut Piece)>,
    mut playfield: ResMut<Playfield>,
    cell_textures: Res<CellTextures>,
    mut cell_events_writer: EventWriter<CellEvent>,
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
        let move_possible = check_move(&playfield, new_pos);

        if move_possible {
            piece.position = new_pos;
        }
    }

    if timer.0.tick(time.delta()).just_finished() {
        let new_pos = piece.position - IVec2::Y;

        let move_possible = check_move(&playfield, new_pos);

        if move_possible {
            piece.position = new_pos;
        } else {
            let cell = playfield.get_mut(piece.position);

            if let Some(cell) = cell {
                *cell = Cell::Filled(piece.piece_type);
                commands.entity(entity).despawn_recursive();
                spawn_piece(commands, cell_textures);
                cell_events_writer.send(CellEvent {
                    position: piece.position,
                    event_type: EventType::Added,
                });
            }
        }
    }
}

fn check_move(playfield: &Playfield, new_pos: IVec2) -> bool {
    let cell = playfield.get(new_pos);

    if let Some(Cell::Empty) = cell {
        true
    } else {
        false
    }
}
