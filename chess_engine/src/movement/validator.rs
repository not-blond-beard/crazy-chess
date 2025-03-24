use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::{Color, MoveError};

pub fn validate_move(
    from: usize,
    to: usize,
    side_to_move: Color,
    white_pieces: u64,
    black_pieces: u64,
    legal_moves: u64,
) -> Result<(), MoveError> {
    let from_bb = square_to_bitboard(from);
    let to_bb = square_to_bitboard(to);

    let is_white_piece = white_pieces & from_bb != 0;
    let is_black_piece = black_pieces & from_bb != 0;

    if !is_white_piece && !is_black_piece {
        return Err(MoveError::NoPieceAtSource);
    }

    match side_to_move {
        Color::White if !is_white_piece => return Err(MoveError::WrongColorPiece),
        Color::Black if !is_black_piece => return Err(MoveError::WrongColorPiece),
        _ => {}
    }

    let is_destination_occupied_by_same_color = match side_to_move {
        Color::White => white_pieces & to_bb != 0,
        Color::Black => black_pieces & to_bb != 0,
    };

    if is_destination_occupied_by_same_color {
        return Err(MoveError::DestinationOccupiedBySameColor);
    }

    if legal_moves & to_bb == 0 {
        return Err(MoveError::InvalidDestination);
    }

    Ok(())
}
