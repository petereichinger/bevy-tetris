use bevy::prelude::*;

use itertools::{Itertools, MinMaxResult};

use super::{piece_types::iter_piece_cells, Piece, PieceType};

#[derive(Resource)]
pub struct PlayfieldSize(pub UVec2);

#[derive(Component)]
pub struct Playfield {
    size: UVec2,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(PieceType),
}

#[derive(Debug, Copy, Clone)]
pub enum CheckRotationResult {
    ValidWithOffset(IVec2),
    Invalid,
}

impl Playfield {
    pub fn new(size: UVec2) -> Self {
        let row = vec![Cell::Empty; size.x as usize];
        let cells = (0..size.y).map(|_| row.clone()).collect::<Vec<_>>();

        Self { size, cells }
    }

    pub fn get(&self, coordinate: IVec2) -> Option<&Cell> {
        if !self.valid_coordinate(coordinate) {
            return None;
        }
        self.cells
            .get(coordinate.y as usize)
            .and_then(|row| row.get(coordinate.x as usize))
    }

    pub fn get_mut(&mut self, coordinate: IVec2) -> Option<&mut Cell> {
        if !self.valid_coordinate(coordinate) {
            return None;
        }
        self.cells
            .get_mut(coordinate.y as usize)
            .and_then(|row| row.get_mut(coordinate.x as usize))
    }

    fn valid_coordinate(&self, IVec2 { x, y }: IVec2) -> bool {
        x >= 0 && y >= 0 && x < self.size.x as i32 && y < self.size.y as i32
    }

    pub fn clear_rows(&mut self) {
        let mut cleared_rows = vec![];
        for y in 0..self.size.y {
            let cleared = (0..self.size.x)
                .map(|x| {
                    self.get(IVec2 {
                        x: x as i32,
                        y: y as i32,
                    })
                })
                .all(|c| matches!(c, Some(Cell::Filled(_))));

            if cleared {
                cleared_rows.push(y);
            }
        }

        cleared_rows.iter().rev().for_each(|row| {
            self.cells.remove(*row as usize);
        });

        (0..cleared_rows.len())
            .for_each(|_| self.cells.push(vec![Cell::Empty; self.size.x as usize]));
    }

    pub fn check_move(&self, piece: &Piece) -> bool {
        let all_free = iter_piece_cells(piece).all(|p| {
            let cell = self.get(p);
            matches!(cell, Some(Cell::Empty))
        });

        all_free
    }

    pub fn check_rotation(&self, piece: &Piece) -> CheckRotationResult {
        let minmax = iter_piece_cells(piece).map(|p| p.x).minmax();
        let IVec2 { x: width, y: _ } = self.size.as_ivec2();
        let (min, max) = match minmax {
            MinMaxResult::NoElements => panic!("empty piece"),
            MinMaxResult::OneElement(xpos) => (xpos, xpos),
            MinMaxResult::MinMax(min, max) => (min, max),
        };

        let offset = if min < 0 {
            -min
        } else if max >= width {
            -(max + 1 - width)
        } else {
            0
        };

        let offset = IVec2::new(offset, 0);
        let after_wall_pos = piece.position + offset;

        let valid_pos = self.check_move(&Piece {
            piece_type: piece.piece_type,
            position: after_wall_pos,
            rotation: piece.rotation,
        });

        if valid_pos {
            CheckRotationResult::ValidWithOffset(offset)
        } else {
            CheckRotationResult::Invalid
        }
    }

    pub fn set_cells(&mut self, piece: &Piece) {
        iter_piece_cells(piece).for_each(|p| {
            let cell = self.get_mut(p);
            if let Some(cell) = cell {
                *cell = Cell::Filled(piece.piece_type);
            }
        });
    }
}
