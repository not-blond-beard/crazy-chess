use crate::bitboard::Square;

pub trait Piece {
    fn name(&self) -> &str;
    fn color(&self) -> &str;
    fn has_moved(&self) -> bool;
    fn mark_as_moved(&mut self);
    fn moveable_squares(&self, from: Square) -> Vec<Square>;
}
