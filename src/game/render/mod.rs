mod cells;
mod piece;
mod playfield_render_size;

use bevy::prelude::*;

use crate::setup::GameState;

use self::{
    cells::{spawn_cells, update_cells},
    piece::update_piece_sprite,
    playfield_render_size::{set_playfield_dimensions, PlayfieldRenderSize},
};

pub(super) struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayfieldRenderSize::default())
            .add_systems(OnEnter(GameState::InGame), spawn_cells)
            .add_systems(
                PreUpdate,
                set_playfield_dimensions.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                PostUpdate,
                (update_piece_sprite, update_cells).run_if(in_state(GameState::InGame)),
            );
    }
}
