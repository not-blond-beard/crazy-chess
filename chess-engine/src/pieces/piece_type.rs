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

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPieceAtSource,
    WrongColorPiece,
    InvalidDestination,
    PathBlocked,
    DestinationOccupiedBySameColor,
}
