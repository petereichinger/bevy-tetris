use bevy::prelude::*;

use super::PieceType;

#[derive(Resource)]
pub struct Playfield {
    pub size: UVec2,
    pub cells: Box<[Cell]>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Cell {
    #[default]
    Empty,
    Filled(PieceType),
}

impl Playfield {
    pub fn new(size: UVec2) -> Self {
        let cell_count = (size.x * size.y) as usize;
        let cells = (0..cell_count)
            .map(|_| Cell::Empty)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { size, cells }
    }

    pub fn get(&self, coordinate: IVec2) -> Option<&Cell> {
        if !self.valid_coordinate(coordinate) {
            return None;
        }
        let index = self.size.x * coordinate.y as u32 + coordinate.x as u32;
        return self.cells.get(index as usize);
    }

    pub fn get_mut(&mut self, coordinate: IVec2) -> Option<&mut Cell> {
        if !self.valid_coordinate(coordinate) {
            return None;
        }
        let index = self.size.x * coordinate.y as u32 + coordinate.x as u32;
        return self.cells.get_mut(index as usize);
    }

    fn valid_coordinate(&self, coordinate: IVec2) -> bool {
        let IVec2 { x, y } = coordinate;

        x >= 0 && y >= 0 && x < self.size.x as i32 && y < self.size.y as i32
    }
}
