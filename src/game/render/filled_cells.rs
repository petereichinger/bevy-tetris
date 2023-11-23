use bevy::prelude::*;

use crate::{
    game::{cell_events::CellEvent, piece_types::get_sprite_for_piece},
    setup::CellTextures,
};

use super::playfield_render_size::PlayfieldRenderSize;

#[derive(Component)]
pub(super) struct FilledCell(IVec2);

pub(super) fn read_cell_events(
    mut commands: Commands,
    mut cell_event_reader: EventReader<CellEvent>,
    cell_textures: Res<CellTextures>,
    playfield_dimensions: Res<PlayfieldRenderSize>,
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

pub(super) fn update_cell_sprites(
    playfield_dimensions: Res<PlayfieldRenderSize>,
    mut filled_cells: Query<(&FilledCell, &mut Transform)>,
) {
    for (filled_cell, mut transform) in filled_cells.iter_mut() {
        *transform = playfield_dimensions.get_transform(filled_cell.0.as_vec2(), 1.0);
    }
}
