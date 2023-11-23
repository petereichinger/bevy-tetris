use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

#[derive(Reflect, PartialEq, Eq, Debug, Copy, Clone)]
pub enum PieceType {
    O,
    J,
    L,
    S,
    T,
    Z,
}

const O_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
];

const J_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
];

const L_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(1, 1),
];

const S_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(-1, 0),
];

const T_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
];

const Z_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(-1, 1),
];

pub fn iter_cells(piece_type: PieceType) -> impl Iterator<Item = &'static IVec2> {
    let cells: &[IVec2] = match piece_type {
        PieceType::O => &O_CELLS,
        PieceType::J => &J_CELLS,
        PieceType::L => &L_CELLS,
        PieceType::S => &S_CELLS,
        PieceType::T => &T_CELLS,
        PieceType::Z => &Z_CELLS,
    };

    cells.iter()
}

pub fn iter_cells_at(position: IVec2, piece_type: PieceType) -> impl Iterator<Item = IVec2> {
    iter_cells(piece_type).map(move |c| position + *c)
}

pub fn get_random_piece_type(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) -> PieceType {
    match rng.next_u32() % 6 {
        0 => PieceType::O,
        1 => PieceType::J,
        2 => PieceType::L,
        3 => PieceType::S,
        4 => PieceType::T,
        5 => PieceType::Z,
        _ => panic!("NOT POSSIBLE"),
    }
}

pub fn get_sprite_for_piece(piece_type: PieceType) -> TextureAtlasSprite {
    let (color, index) = match piece_type {
        PieceType::O => (Color::YELLOW_GREEN, 1),
        PieceType::J => (Color::BLUE, 2),
        PieceType::L => (Color::ORANGE_RED, 3),
        PieceType::S => (Color::SEA_GREEN, 4),
        PieceType::T => (Color::PURPLE, 5),
        PieceType::Z => (Color::RED, 6),
    };

    TextureAtlasSprite {
        color,
        index,
        ..default()
    }
}
