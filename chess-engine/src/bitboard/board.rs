use crate::bitboard::constants::*;
use crate::bitboard::square::Square;
use crate::pieces::piece_type::Color;

#[derive(Debug, Clone, Default)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn full() -> Self {
        Self::init(UNIVERSE)
    }

    pub fn init(board: u64) -> Self {
        Self { board }
    }

    pub fn rank(rank: u8) -> Self {
        match rank {
            1 => Self::init(RANK_1),
            2 => Self::init(RANK_2),
            7 => Self::init(RANK_7),
            8 => Self::init(RANK_8),
            _ => Self::init(EMPTY),
        }
    }

    pub fn file(file: u8) -> Self {
        match file {
            1 => Self::init(FILE_A),
            2 => Self::init(FILE_B),
            7 => Self::init(FILE_G),
            8 => Self::init(FILE_H),
            _ => Self::init(EMPTY),
        }
    }

    pub fn colored(color: Color) -> Self {
        match color {
            Color::White => Self::init(LIGHT_SQUARES),
            Color::Black => Self::init(DARK_SQUARES),
        }
    }
}

impl BitBoard {
    pub fn is_occupied(&self, square: Square) -> bool {
        self.board & square != 0
    }

    pub fn set(&mut self, square: Square) {
        self.board |= square;
    }

    pub fn unset(&mut self, square: Square) {
        self.board &= !square;
    }
}
