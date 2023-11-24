use bevy::prelude::*;

use crate::{
    game::{
        piece_types::{get_sprite_for_piece, iter_cells},
        Piece,
    },
    setup::CellTextures,
};

use super::playfield_render_size::PlayfieldRenderSize;

#[derive(Component)]
pub(super) struct PieceRender;

pub(super) fn update_piece_sprite(
    mut commands: Commands,
    new_piece_query: Query<&Piece, Added<Piece>>,
    piece_query: Query<&Piece>,
    mut render_piece_query: Query<(&PieceRender, &mut Transform, Entity)>,
    playfield_dimensions: Res<PlayfieldRenderSize>,
    cell_textures: Res<CellTextures>,
) {
    if let Ok(&Piece {
        piece_type,
        rotation,
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
                iter_cells(piece_type, rotation).for_each(|pos| {
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

        *transform = playfield_dimensions.get_piece_transform(piece, 1.0);
    }
}
