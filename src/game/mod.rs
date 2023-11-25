mod piece_types;
mod playfield;
mod render;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;

use crate::setup::GameState;

use self::{
    piece_types::{get_random_piece_type, PieceType},
    playfield::{Playfield, PlayfieldSize},
    render::RenderPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
            .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(PlayfieldSize([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(OnEnter(GameState::InGame), (spawn_piece, spawn_playfield))
            .add_systems(Update, (move_piece).run_if(in_state(GameState::InGame)))
            .add_plugins(RenderPlugin);
    }
}

fn spawn_playfield(mut commands: Commands) {
    commands.spawn((Name::new("Playfield"), Playfield::new([10, 24].into())));
}

fn spawn_piece(mut commands: Commands, rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    let piece_type = get_random_piece_type(rng);
    commands.spawn((Name::new("Current Piece"), Piece::new(piece_type)));
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Debug, Default, Clone, Copy, Reflect)]
enum Rotation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl From<Rotation> for f32 {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::R0 => 0.0f32.to_radians(),
            Rotation::R90 => 90.0f32.to_radians(),
            Rotation::R180 => 180.0f32.to_radians(),
            Rotation::R270 => 270.0f32.to_radians(),
        }
    }
}

#[derive(Reflect, Component)]
struct Piece {
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
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    keys: Res<Input<KeyCode>>,
) {
    let (entity, mut piece) = query.single_mut();
    let mut playfield = playfield_query.single_mut();
    if keys.just_pressed(KeyCode::Up) {
        use Rotation::*;

        piece.rotation = match piece.rotation {
            R0 => R90,
            R90 => R180,
            R180 => R270,
            R270 => R0,
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
            spawn_piece(commands, rng);

            playfield.set_cells(&piece);
            playfield.clear_rows();
        }
    }
}
