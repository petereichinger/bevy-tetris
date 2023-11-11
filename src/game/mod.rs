use bevy::{ecs::query::QuerySingleError, prelude::*};

use crate::setup::{CellBackground, CellTextures, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PlayfieldSize(IVec2::new(10, 24)))
            .insert_resource(PlayfieldDimensions::default())
            .register_type::<BackgroundCell>()
            .register_type::<Piece>()
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_piece_if_necessary, spawn_grid_background),
            )
            .add_systems(
                PreUpdate,
                update_playfield_dimensions.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                drop_current_piece.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                (update_piece_sprite, update_background_grid_sprites)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource)]
struct PlayfieldSize(IVec2);

#[derive(Component)]
struct BackgroundGrid;

#[derive(Reflect, Component)]
struct BackgroundCell(Vec2);

#[derive(Debug, Resource, Default)]
struct PlayfieldDimensions {
    cell_size: f32,
    grid_size: Vec2,
    scale: Vec3,
}

fn spawn_grid_background(
    mut commands: Commands,
    playfield_size: Res<PlayfieldSize>,
    background_sprite: Res<CellBackground>,
) {
    let texture = &background_sprite.0;
    let size = &playfield_size.0;

    let fsize = size.as_vec2();
    commands
        .spawn((BackgroundGrid, SpatialBundle::default()))
        .with_children(|cb| {
            for y in 0..size.y {
                for x in 0..size.x {
                    let position = IVec2::new(x, y).as_vec2();
                    cb.spawn((
                        BackgroundCell(position),
                        SpriteBundle {
                            texture: texture.clone(),
                            ..default()
                        },
                    ));
                }
            }
        });
}

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

fn update_playfield_dimensions(
    playfield: Res<PlayfieldSize>,
    mut windows: Query<&Window>,
    mut playfield_dimensions: ResMut<PlayfieldDimensions>,
    cell_textures: Res<CellTextures>,
) {
    let size = &playfield.0;
    let window = &windows.get_single_mut();
    if let Err(e) = window {
        return;
    } else if let Ok(window) = window {
        let resolution = &window.resolution;

        let resolution = Vec2::new(resolution.width(), resolution.height());

        let padded_res = 0.75 * resolution;

        let max_cell_size = padded_res / size.as_vec2();

        let cell_size = max_cell_size.min_element();

        let grid_size = cell_size * size.as_vec2();

        let scale = cell_size / cell_textures.size;
        *playfield_dimensions = PlayfieldDimensions {
            cell_size,
            grid_size,
            scale: Vec2::splat(scale).extend(1.0),
        };
    }
}

fn update_piece_sprite(
    mut commands: Commands,
    mut query: Query<(&Piece, &mut Transform)>,
    cell_textures: Res<CellTextures>,
    playfield_dimensions: Res<PlayfieldDimensions>,
) {
    let (piece, mut transform) = query.single_mut();

    let (cell_size, grid_size) = (
        playfield_dimensions.cell_size,
        playfield_dimensions.grid_size,
    );

    let position = cell_size * piece.position.as_vec2() - 0.5 * grid_size;
    let position = position.extend(0.0);

    let scale = cell_size / cell_textures.size;
    *transform = transform
        .with_translation(position)
        .with_scale(Vec2::splat(scale).extend(1.0));
}

fn update_background_grid_sprites(
    playfield_dimensions: Res<PlayfieldDimensions>,
    mut background_grid_query: Query<(&BackgroundCell, &mut Transform)>,
) {
    let PlayfieldDimensions {
        cell_size,
        grid_size,
        scale,
    } = &*playfield_dimensions;
    for (cell, mut transform) in background_grid_query.iter_mut() {
        let position = *cell_size * cell.0 - 0.5 * *grid_size;
        let position = position.extend(-1.0);
        *transform = transform.with_translation(position).with_scale(*scale);
    }
}
