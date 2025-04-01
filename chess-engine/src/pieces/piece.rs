use crate::bitboard::Square;

pub type PieceName = &'static str;

pub trait Piece {
    fn piece_type(&self) -> PieceName;
    fn has_moved(&self) -> bool;
    fn mark_as_moved(&mut self);
    fn moveable_squares(&self, from: Square) -> Vec<Square>;
}
