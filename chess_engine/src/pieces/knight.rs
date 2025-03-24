use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::Color;

pub fn get_knight_moves(
    from: usize,
    white_knights: u64,
    black_knights: u64,
    white_pieces: u64,
    black_pieces: u64,
    side_to_move: Color,
) -> u64 {
    let from_bb = square_to_bitboard(from);
    let own_pieces = match side_to_move {
        Color::White => white_pieces,
        Color::Black => black_pieces,
    };

    let is_knight_at_square = match side_to_move {
        Color::White => white_knights & from_bb != 0,
        Color::Black => black_knights & from_bb != 0,
    };

    if !is_knight_at_square {
        return 0;
    }

    let mut moves = 0u64;

    let file = from % 8;
    let rank = from / 8;

    let possible_moves = [
        if file > 1 && rank < 7 {
            Some(from + 6)
        } else {
            None
        },
        if file > 0 && rank < 6 {
            Some(from + 15)
        } else {
            None
        },
        if file < 7 && rank < 6 {
            Some(from + 17)
        } else {
            None
        },
        if file < 6 && rank < 7 {
            Some(from + 10)
        } else {
            None
        },
        if file < 6 && rank > 0 {
            Some(from - 6)
        } else {
            None
        },
        if file < 7 && rank > 1 {
            Some(from - 15)
        } else {
            None
        },
        if file > 0 && rank > 1 {
            Some(from - 17)
        } else {
            None
        },
        if file > 1 && rank > 0 {
            Some(from - 10)
        } else {
            None
        },
    ];

    for target_opt in possible_moves.iter() {
        if let Some(target) = target_opt {
            let target_bb = square_to_bitboard(*target);
            if target_bb & own_pieces == 0 {
                moves |= target_bb;
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_middle_board() {
        let white_knights = 1u64 << 35;
        let white_pieces = white_knights;
        let black_pieces = 0;

        let moves = get_knight_moves(
            35,
            white_knights,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = (1u64 << 18) | 
            (1u64 << 20) | 
            (1u64 << 25) | 
            (1u64 << 41) | 
            (1u64 << 45) | 
            (1u64 << 52) | 
            (1u64 << 50) | 
            (1u64 << 29);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_corner() {
        let white_knights = 1u64 << 7;
        let white_pieces = white_knights;
        let black_pieces = 0;

        let moves = get_knight_moves(
            7,
            white_knights,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = (1u64 << 13) | 
            (1u64 << 22);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_with_friendly_blockers() {
        let white_knights = 1u64 << 35;
        let white_pawns = (1u64 << 41) | (1u64 << 45);
        let white_pieces = white_knights | white_pawns;
        let black_pieces = 0;

        let moves = get_knight_moves(
            35,
            white_knights,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = (1u64 << 18) | 
            (1u64 << 20) | 
            (1u64 << 25) | 
            (1u64 << 29) | 
            (1u64 << 50) | 
            (1u64 << 52);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_with_enemy_capture() {
        let white_knights = 1u64 << 35;
        let white_pieces = white_knights;
        let black_pawns = (1u64 << 41) | (1u64 << 45);
        let black_pieces = black_pawns;

        let moves = get_knight_moves(
            35,
            white_knights,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = (1u64 << 18) | 
            (1u64 << 20) | 
            (1u64 << 25) | 
            (1u64 << 29) | 
            (1u64 << 41) | 
            (1u64 << 45) | 
            (1u64 << 50) | 
            (1u64 << 52);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_knight() {
        let black_knights = 1u64 << 35;
        let white_knights = 0;
        let white_pieces = 0;
        let black_pieces = black_knights;

        let moves = get_knight_moves(
            35,
            white_knights,
            black_knights,
            white_pieces,
            black_pieces,
            Color::Black,
        );

        let expected = (1u64 << 18) | 
            (1u64 << 20) | 
            (1u64 << 25) | 
            (1u64 << 29) | 
            (1u64 << 41) | 
            (1u64 << 45) | 
            (1u64 << 50) | 
            (1u64 << 52);

        assert_eq!(moves, expected);
    }
}
