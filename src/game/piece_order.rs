use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

use super::{piece_types::PieceType, Piece};

#[derive(Debug, Resource)]
pub(super) struct PieceOrder {
    pieces: Vec<PieceType>,
    index: usize,
}

impl PieceOrder {
    pub(super) fn new(rng: ResMut<GlobalEntropy<ChaCha8Rng>>) -> Self {
        use PieceType::*;
        let mut pieces = vec![O, J, L, S, T, Z, I];
        fisher_yates_shuffle(&mut pieces, rng);

        Self { pieces, index: 0 }
    }

    pub(super) fn is_finished(&self) -> bool {
        self.index >= self.pieces.len()
    }

    pub(super) fn next_piece(&mut self) -> Option<PieceType> {
        let result = match self.index {
            index if index < self.pieces.len() => Some(self.pieces[index]),
            _ => None,
        };

        self.index += 1;

        result
    }
}

pub(super) fn create_piece_order(mut commands: Commands, rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands.insert_resource(PieceOrder::new(rng));
}

fn fisher_yates_shuffle<T>(items: &mut [T], mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    for i in (1..items.len()).rev() {
        let j = rng.next_u32() as usize % i;
        items.swap(i, j);
    }
}
