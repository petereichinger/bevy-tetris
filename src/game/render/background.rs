use bevy::prelude::*;

use crate::{game::playfield::Playfield, setup::CellBackground};

use super::PlayfieldRenderSize;

#[derive(Component)]
pub(super) struct BackgroundGrid;

#[derive(Reflect, Component)]
pub(super) struct BackgroundCell(Vec2);

pub(super) fn spawn_grid_background(
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

pub(super) fn update_background_grid_sprites(
    playfield_dimensions: Res<PlayfieldRenderSize>,
    mut background_grid_query: Query<(&BackgroundCell, &mut Transform)>,
) {
    for (cell, mut transform) in background_grid_query.iter_mut() {
        *transform = playfield_dimensions.get_transform(cell.0, 0.0);
    }
}
