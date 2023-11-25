use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use super::{Piece, Rotation};

#[derive(Reflect, PartialEq, Eq, Debug, Copy, Clone)]
pub enum PieceType {
    O,
    J,
    L,
    S,
    T,
    Z,
    I,
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

const I_CELLS: [IVec2; 4] = [
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(-2, 0),
];

pub fn iter_cells(piece_type: PieceType, rotation: Rotation) -> impl Iterator<Item = IVec2> {
    let cells: &[IVec2] = match piece_type {
        PieceType::O => &O_CELLS,
        PieceType::J => &J_CELLS,
        PieceType::L => &L_CELLS,
        PieceType::S => &S_CELLS,
        PieceType::T => &T_CELLS,
        PieceType::Z => &Z_CELLS,
        PieceType::I => &I_CELLS,
    };

    cells.iter().map(move |c| match rotation {
        Rotation::R0 => *c,
        Rotation::R90 => IVec2 { x: -c.y, y: c.x },
        Rotation::R180 => IVec2 { x: -c.x, y: -c.y },
        Rotation::R270 => IVec2 { x: c.y, y: -c.x },
    })
}

pub fn iter_piece_cells(
    Piece {
        position,
        rotation,
        piece_type,
    }: &Piece,
) -> impl Iterator<Item = IVec2> + '_ {
    iter_cells(*piece_type, *rotation).map(move |c| *position + c)
}

pub fn get_random_piece_type(mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) -> PieceType {
    match rng.next_u32() % 7 {
        0 => PieceType::O,
        1 => PieceType::J,
        2 => PieceType::L,
        3 => PieceType::S,
        4 => PieceType::T,
        5 => PieceType::Z,
        6 => PieceType::I,
        _ => panic!("NOT POSSIBLE"),
    }
}

pub const EMPTY_SPRITE: TextureAtlasSprite = TextureAtlasSprite {
    color: Color::DARK_GRAY,
    index: 0,
    flip_x: false,
    flip_y: false,
    custom_size: None,
    anchor: bevy::sprite::Anchor::Center,
};

pub fn get_sprite_for_piece(piece_type: PieceType) -> TextureAtlasSprite {
    let (color, index) = match piece_type {
        PieceType::O => (BRIGHT_ORANGE, 1),
        PieceType::J => (PETROL, 2),
        PieceType::L => (LIME_GREEN, 3),
        PieceType::S => (CRISP_LAVENDER, 4),
        PieceType::T => (SILVER_GREY, 5),
        PieceType::Z => (DEEP_GREEN, 6),
        PieceType::I => (Color::ORANGE_RED, 6),
    };

    TextureAtlasSprite {
        color,
        index,
        ..default()
    }
}

const PETROL: Color = Color::rgb(0.009, 0.2, 0.33);
const BRIGHT_ORANGE: Color = Color::rgb(1.00, 0.55, 0.00);
const LIME_GREEN: Color = Color::rgb(0.80, 1.00, 0.00);
const SILVER_GREY: Color = Color::rgb(0.75, 0.75, 0.75);
const DEEP_GREEN: Color = Color::rgb(0.00, 0.39, 0.32);
const CRISP_LAVENDER: Color = Color::rgb(0.70, 0.53, 0.80);
