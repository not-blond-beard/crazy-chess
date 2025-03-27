use crate::bitboard::{BitBoard, Square};
use crate::pieces::piece_type::{Color, PieceType};
use std::collections::HashMap;

#[derive(Default)]
pub struct GameBoard {
    boards: HashMap<(Color, PieceType), BitBoard>,
}

impl GameBoard {
    pub fn is_occupied(&self, square: Square) -> bool {
        self.boards
            .iter()
            .any(|(_, board)| board.is_occupied(square))
    }

    pub fn occupied_bits(&self) -> u64 {
        self.boards
            .iter()
            .fold(0u64, |acc, (_, board)| acc | *board)
    }

    pub fn occupied_squares(&self) -> Vec<Square> {
        let mut vec = Vec::with_capacity(64);
        let bits = self.occupied_bits();
        for i in 0..64 {
            let space = 1u64 << i;
            if bits & (space) != 0 {
                vec.push(Square::from(space));
            }
        }
        vec
    }
}
