use bevy::prelude::*;

use crate::setup::{CellBackground, CellTextures, GameState};

use super::{
    cell_events::CellEvent,
    piece_types::{get_sprite_for_piece, iter_cells},
    Piece, Playfield,
};

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayfieldDimensions::default())
            .register_type::<BackgroundCell>()
            .add_systems(OnEnter(GameState::InGame), spawn_grid_background)
            .add_systems(
                PreUpdate,
                set_playfield_dimensions.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                (
                    update_piece_sprite,
                    update_background_grid_sprites,
                    read_cell_events,
                    update_cell_sprites,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct PieceRender;

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

impl PlayfieldDimensions {
    pub fn get_transform(&self, position: Vec2, depth: f32) -> Transform {
        let position = self.cell_size * position - 0.5 * self.grid_size;
        let position = position.extend(depth);
        Transform::from_translation(position).with_scale(self.scale)
    }
}

#[derive(Component)]
struct FilledCell(IVec2);

fn spawn_grid_background(
    mut commands: Commands,
    playfield_size: Res<Playfield>,
    background_sprite: Res<CellBackground>,
) {
    let texture = &background_sprite.0;
    let Playfield { size, .. } = &*playfield_size;

    commands
        .spawn((BackgroundGrid, SpatialBundle::default()))
        .with_children(|cb| {
            for y in 0..size.y {
                for x in 0..size.x {
                    let position = UVec2::new(x, y).as_vec2();
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
fn set_playfield_dimensions(
    playfield: Res<Playfield>,
    mut windows: Query<&Window>,
    mut playfield_dimensions: ResMut<PlayfieldDimensions>,
    cell_textures: Res<CellTextures>,
) {
    let Playfield { size, .. } = &*playfield;
    let window = &windows.get_single_mut();
    if let Ok(window) = window {
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
    new_piece_query: Query<&Piece, Added<Piece>>,
    piece_query: Query<&Piece>,
    mut render_piece_query: Query<(&PieceRender, &mut Transform, Entity)>,
    playfield_dimensions: Res<PlayfieldDimensions>,
    cell_textures: Res<CellTextures>,
) {
    if let Ok(&Piece {
        piece_type,
        position,
    }) = new_piece_query.get_single()
    {
        // we got a new piece replace RenderPiece entity

        if let Ok((_, _, entity)) = render_piece_query.get_single() {
            commands.entity(entity).despawn_recursive()
        }

        let sprite = get_sprite_for_piece(piece_type);
        commands
            .spawn((
                PieceRender,
                SpatialBundle {
                    transform: playfield_dimensions.get_transform(position.as_vec2(), 1.0),
                    ..default()
                },
            ))
            .with_children(|cb| {
                iter_cells(piece_type).for_each(|pos| {
                    let texture_atlas = cell_textures.atlas.clone();
                    cb.spawn(SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas,
                        transform: Transform::from_translation(32.0 * pos.as_vec2().extend(0.0)),
                        ..Default::default()
                    });
                })
            });
    } else {
        // update position of render piece

        let piece = piece_query.single();
        let (_, mut transform, _) = render_piece_query.single_mut();

        *transform = playfield_dimensions.get_transform(piece.position.as_vec2(), 1.0);
    }
}

fn update_background_grid_sprites(
    playfield_dimensions: Res<PlayfieldDimensions>,
    mut background_grid_query: Query<(&BackgroundCell, &mut Transform)>,
) {
    for (cell, mut transform) in background_grid_query.iter_mut() {
        *transform = playfield_dimensions.get_transform(cell.0, 0.0);
    }
}

fn read_cell_events(
    mut commands: Commands,
    mut cell_event_reader: EventReader<CellEvent>,
    cell_textures: Res<CellTextures>,
    playfield_dimensions: Res<PlayfieldDimensions>,
) {
    let texture_atlas = cell_textures.atlas.clone();

    for event in cell_event_reader.read() {
        match event {
            CellEvent::Added {
                position,
                piece_type,
            } => {
                let sprite = get_sprite_for_piece(*piece_type);
                commands.spawn((
                    FilledCell(*position),
                    SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas: texture_atlas.clone(),
                        transform: playfield_dimensions.get_transform(position.as_vec2(), 1.0),
                        ..Default::default()
                    },
                ));
            } // EventType::Removed => todo!(),,
        }
    }
}

fn update_cell_sprites(
    playfield_dimensions: Res<PlayfieldDimensions>,
    mut filled_cells: Query<(&FilledCell, &mut Transform)>,
) {
    for (filled_cell, mut transform) in filled_cells.iter_mut() {
        *transform = playfield_dimensions.get_transform(filled_cell.0.as_vec2(), 1.0);
    }
}
