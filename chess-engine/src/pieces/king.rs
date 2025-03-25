use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::Color;

pub fn get_king_moves(
    from: usize,
    white_kings: u64,
    black_kings: u64,
    white_pieces: u64,
    black_pieces: u64,
    side_to_move: Color,
) -> u64 {
    let from_bb = square_to_bitboard(from);
    let own_pieces = match side_to_move {
        Color::White => white_pieces,
        Color::Black => black_pieces,
    };
    let _opponent_pieces = match side_to_move {
        Color::White => black_pieces,
        Color::Black => white_pieces,
    };

    let is_king_at_square = match side_to_move {
        Color::White => white_kings & from_bb != 0,
        Color::Black => black_kings & from_bb != 0,
    };

    if !is_king_at_square {
        return 0;
    }

    let mut moves = 0u64;
    
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let file = from % 8;
    let rank = from / 8;

    for &(file_delta, rank_delta) in &directions {
        let new_file = file as i32 + file_delta;
        let new_rank = rank as i32 + rank_delta;

        if new_file < 0 || new_file > 7 || new_rank < 0 || new_rank > 7 {
            continue;
        }

        let target = (new_rank as usize) * 8 + (new_file as usize);
        let target_bb = square_to_bitboard(target);

        if target_bb & own_pieces != 0 {
            continue;
        }

        moves |= target_bb;
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_middle_board() {
        let white_kings = 1u64 << 27;
        let white_pieces = white_kings;
        let black_pieces = 0;

        let moves = get_king_moves(
            27,
            white_kings,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 18) |
            (1u64 << 19) |
            (1u64 << 20) |
            (1u64 << 26) |
            (1u64 << 28) |
            (1u64 << 34) |
            (1u64 << 35) |
            (1u64 << 36);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_king_corner() {
        let white_kings = 1u64 << 0;
        let white_pieces = white_kings;
        let black_pieces = 0;

        let moves = get_king_moves(
            0,
            white_kings,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 1) |
            (1u64 << 8) |
            (1u64 << 9);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_king_with_blockers() {
        let white_kings = 1u64 << 27;
        let white_pawns = (1u64 << 18) | (1u64 << 19) | (1u64 << 20);
        let black_pawns = 1u64 << 35;

        let white_pieces = white_kings | white_pawns;
        let black_pieces = black_pawns;

        let moves = get_king_moves(
            27,
            white_kings,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 26) |
            (1u64 << 28) |
            (1u64 << 34) |
            (1u64 << 35) |
            (1u64 << 36);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_king() {
        let black_kings = 1u64 << 35;
        let white_pieces = 0;
        let black_pieces = black_kings;

        let moves = get_king_moves(
            35,
            0,
            black_kings,
            white_pieces,
            black_pieces,
            Color::Black,
        );

        let expected = 
            (1u64 << 26) |
            (1u64 << 27) |
            (1u64 << 28) |
            (1u64 << 34) |
            (1u64 << 36) |
            (1u64 << 42) |
            (1u64 << 43) |
            (1u64 << 44);

        assert_eq!(moves, expected);
    }
} 