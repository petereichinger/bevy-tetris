mod piece_order;
mod piece_types;
mod playfield;
mod render;
mod rotation;

use std::time::Duration;

use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_prng::ChaCha8Rng;
use bevy_rand::{prelude::*, resource};

use bevy_egui::{egui, EguiContexts};

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
            .insert_resource(PlayfieldSize([10, 24].into()))
            .register_type::<Piece>()
            .add_systems(
                OnEnter(GameState::SetupGame),
                (setup_game, create_piece_order),
            )
            .add_systems(
                Update,
                (spawn_piece, move_piece).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), tear_down_game)
            .add_systems(Update, score_ui.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                game_over_screen.run_if(in_state(GameState::GameOver)),
            )
            .add_plugins(RenderPlugin);
    }
}

fn setup_game(mut commands: Commands, playfield_size: Res<PlayfieldSize>) {
    commands.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    commands.insert_resource(Score { score: 0 });
    commands.spawn((Name::new("Playfield"), Playfield::new(playfield_size.0)));
    commands.insert_resource(NextState(Some(GameState::InGame)));
}

fn tear_down_game(mut commands: Commands, playfield_query: Query<Entity, With<Playfield>>) {
    commands.remove_resource::<StepTimer>();
    commands.remove_resource::<Score>();
    commands
        .entity(playfield_query.single())
        .despawn_recursive();
}

fn spawn_piece(
    piece: Query<&Piece>,
    mut commands: Commands,
    mut piece_order: ResMut<PieceOrder>,
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    playfield_query: Query<&Playfield>,
) {
    if let Err(QuerySingleError::NoEntities(_)) = piece.get_single() {
        if piece_order.is_finished() {
            *piece_order = PieceOrder::new(rng);
        }
        let piece_type = piece_order.next_piece().expect("Should not be empty");

        let playfield = playfield_query.single();

        let new_piece = Piece::new(piece_type);
        if playfield.check_move(&new_piece) {
            commands.spawn((Name::new("Current Piece"), new_piece));
        } else {
            commands.insert_resource(NextState(Some(GameState::GameOver)))
        }
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
            position: IVec2::new(5, 22),
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
    mut score: ResMut<Score>,
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
        let old_pos = piece.position;
        while playfield.check_move(&Piece {
            position: piece.position + IVec2::NEG_Y,
            ..*piece
        }) {
            piece.position += IVec2::NEG_Y;
        }
        let new_pos = piece.position;

        if new_pos != old_pos {}
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
            let cleared_rows = playfield.clear_rows();

            score.score += cleared_rows as u32;
        }

        timer.0.set_duration(Duration::from_secs_f32(score.speed()))
    }
}

#[derive(Debug, Resource)]
struct Score {
    score: u32,
}

impl Score {
    fn level(&self) -> u32 {
        self.score / 5
    }

    fn speed(&self) -> f32 {
        1.0 * (0.75f32.powf(self.level() as f32))
    }
}

fn score_ui(mut contexts: EguiContexts, score: Res<Score>) {
    egui::Window::new("Bevy Tetris").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Current Score: {}", score.score));
        ui.label(format!("Current Level: {}", score.level() + 1));
    });
}

fn game_over_screen(mut contexts: EguiContexts, mut commands: Commands) {
    egui::Window::new("GAME OVER").show(contexts.ctx_mut(), |ui| {
        if ui.button("Restart!").clicked() {
            commands.insert_resource(NextState(Some(GameState::SetupGame)))
        }
    });
}
