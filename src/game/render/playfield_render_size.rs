use bevy::prelude::*;

use crate::{
    game::{playfield::Playfield, Piece},
    setup::CellTextures,
};

#[derive(Debug, Resource, Default)]
pub(super) struct PlayfieldRenderSize {
    pub(super) cell_size: f32,
    pub(super) grid_size: Vec2,
    pub(super) scale: Vec3,
}

impl PlayfieldRenderSize {
    pub fn get_transform(&self, position: Vec2, depth: f32) -> Transform {
        let position = self.cell_size * position - 0.5 * self.grid_size;
        let position = position.extend(depth);
        Transform::from_translation(position).with_scale(self.scale)
    }

    pub fn get_piece_transform(&self, piece: &Piece, depth: f32) -> Transform {
        let position = self.cell_size * piece.position.as_vec2() - 0.5 * self.grid_size;
        let position = position.extend(depth);

        use crate::game::Rotation::*;
        let angle: f32 = match piece.rotation {
            R0 => 0.0,
            R90 => 90.0,
            R180 => 180.0,
            R270 => 270.0,
        };

        let rotation = Quat::from_axis_angle(Vec3::Z, angle.to_radians());
        Transform::from_translation(position)
            .with_rotation(rotation)
            .with_scale(self.scale)
    }
}

pub(super) fn set_playfield_dimensions(
    playfield: Res<Playfield>,
    mut windows: Query<&Window>,
    mut playfield_dimensions: ResMut<PlayfieldRenderSize>,
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

        *playfield_dimensions = PlayfieldRenderSize {
            cell_size,
            grid_size,
            scale: Vec2::splat(scale).extend(1.0),
        };
    }
}