use bevy::prelude::*;

use crate::setup::{CellBackground, CellTextures, GameState};

use super::{Piece, Playfield};

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayfieldDimensions::default())
            .register_type::<BackgroundCell>()
            .add_systems(OnEnter(GameState::InGame), spawn_grid_background)
            .add_systems(
                PreUpdate,
                update_playfield_dimensions.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                (update_piece_sprite, update_background_grid_sprites)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
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
fn update_playfield_dimensions(
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
