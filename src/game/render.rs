use bevy::prelude::*;

use crate::setup::{CellBackground, CellTextures, GameState};

use super::{
    cell_events::{CellEvent, EventType},
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
    mut query: Query<(&Piece, &mut Transform)>,
    playfield_dimensions: Res<PlayfieldDimensions>,
) {
    let piece = query.get_single_mut().ok();

    if let Some((piece, mut transform)) = piece {
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
    let sprite = TextureAtlasSprite {
        color: Color::ORANGE_RED,
        index: 1,
        ..default()
    };
    for CellEvent {
        position,
        event_type,
    } in cell_event_reader.read()
    {
        match event_type {
            EventType::Added => {
                commands.spawn((
                    FilledCell(*position),
                    SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas: texture_atlas.clone(),
                        transform: playfield_dimensions.get_transform(position.as_vec2(), 1.0),
                        ..Default::default()
                    },
                ));
            } // EventType::Removed => todo!(),
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
