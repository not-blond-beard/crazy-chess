#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}

impl PieceType {
    pub fn default_board() -> u64 {
        0
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPieceAtSource,
    WrongColorPiece,
    InvalidDestination,
    PathBlocked,
    DestinationOccupiedBySameColor,
}
