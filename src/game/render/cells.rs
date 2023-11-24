use bevy::prelude::*;

use crate::{
    game::{
        piece_types::{get_sprite_for_piece, EMPTY_SPRITE},
        playfield::{Cell, Playfield},
    },
    setup::CellTextures,
};

use super::PlayfieldRenderSize;

#[derive(Component)]
pub(super) struct CellRenderGrid;

#[derive(Reflect, Component)]
pub(super) struct CellRender(UVec2);

pub(super) fn spawn_cells(
    mut commands: Commands,
    playfield_size: Res<Playfield>,
    cell_textures: Res<CellTextures>,
) {
    let Playfield { size, .. } = &*playfield_size;
    let texture_atlas = cell_textures.atlas.clone();
    commands
        .spawn((CellRenderGrid, SpatialBundle::default()))
        .with_children(|cb| {
            for y in 0..size.y {
                for x in 0..size.x {
                    cb.spawn((
                        CellRender(UVec2::new(x, y)),
                        SpriteSheetBundle {
                            texture_atlas: texture_atlas.clone(),
                            ..default()
                        },
                    ));
                }
            }
        });
}

pub(super) fn update_cells(
    playfield_dimensions: Res<PlayfieldRenderSize>,
    mut playfield: ResMut<Playfield>,
    mut background_grid_query: Query<(&CellRender, &mut Transform, &mut TextureAtlasSprite)>,
) {
    for (CellRender(pos), mut transform, mut atlas_sprite) in background_grid_query.iter_mut() {
        if let Some(cell) = playfield.get_mut(pos.as_ivec2()) {
            *atlas_sprite = match cell {
                Cell::Empty => EMPTY_SPRITE,
                Cell::Filled(piece_type) => get_sprite_for_piece(*piece_type),
            }
        }
        *transform = playfield_dimensions.get_transform(pos.as_vec2(), 0.0);
    }
}
