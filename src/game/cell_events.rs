use bevy::prelude::*;

use super::piece_types::PieceType;

#[derive(Debug, Event)]
pub enum CellEvent {
    Added {
        position: IVec2,
        piece_type: PieceType,
    },
}
