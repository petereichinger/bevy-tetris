use bevy::prelude::*;

use super::PieceType;

#[derive(Resource)]
pub struct Playfield {
    pub size: UVec2,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(PieceType),
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
}
