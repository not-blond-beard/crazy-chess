use crate::pieces::piece_type::{Color, PieceType};

#[derive(Clone)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
    has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            has_moved: false,
        }
    }
}

impl Piece {
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn mark_as_moved(&mut self) {
        self.has_moved = true;
    }
}
