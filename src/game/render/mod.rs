mod background;
mod filled_cells;
mod piece;
mod playfield_render_size;

use bevy::prelude::*;

use crate::setup::{CellTextures, GameState};

use self::{
    background::{spawn_grid_background, update_background_grid_sprites},
    filled_cells::{read_cell_events, update_cell_sprites},
    piece::update_piece_sprite,
    playfield_render_size::{set_playfield_dimensions, PlayfieldRenderSize},
};

use super::{cell_events::CellEvent, piece_types::get_sprite_for_piece};

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayfieldRenderSize::default())
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
