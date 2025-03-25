use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::Color;

pub fn get_pawn_moves(from: usize, white_pawns: u64, black_pawns: u64, side_to_move: Color) -> u64 {
    let from_bb = square_to_bitboard(from);
    let occupied = white_pawns | black_pawns;
    let mut moves = 0u64;

    match side_to_move {
        Color::White => {
            if white_pawns & from_bb == 0 {
                return 0;
            }

            let single_push = from_bb << 8;
            if single_push & occupied == 0 {
                moves |= single_push;

                if from / 8 == 1 {
                    let double_push = single_push << 8;
                    if double_push & occupied == 0 {
                        moves |= double_push;
                    }
                }
            }

            if from % 8 != 0 {
                let capture_left = from_bb << 7;
                if capture_left & black_pawns != 0 {
                    moves |= capture_left;
                }
            }
            if from % 8 != 7 {
                let capture_right = from_bb << 9;
                if capture_right & black_pawns != 0 {
                    moves |= capture_right;
                }
            }
        }
        Color::Black => {
            if black_pawns & from_bb == 0 {
                return 0;
            }

            let single_push = from_bb >> 8;
            if single_push & occupied == 0 {
                moves |= single_push;

                if from / 8 == 6 {
                    let double_push = single_push >> 8;
                    if double_push & occupied == 0 {
                        moves |= double_push;
                    }
                }
            }

            if from % 8 != 7 {
                let capture_right = from_bb >> 7;
                if capture_right & white_pawns != 0 {
                    moves |= capture_right;
                }
            }
            if from % 8 != 0 {
                let capture_left = from_bb >> 9;
                if capture_left & white_pawns != 0 {
                    moves |= capture_left;
                }
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_pawn_single_push() {
        let white_pawns = 1u64 << 8;
        let black_pawns = 0;
        let moves = get_pawn_moves(8, white_pawns, black_pawns, Color::White);

        let expected = (1u64 << 16) | (1u64 << 24);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_white_pawn_double_push() {
        let white_pawns = 1u64 << 8;
        let black_pawns = 0;
        let moves = get_pawn_moves(8, white_pawns, black_pawns, Color::White);

        let expected = (1u64 << 16) | (1u64 << 24);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_white_pawn_capture() {
        let white_pawns = 1u64 << 8;
        let black_pawns = 1u64 << 17;
        let moves = get_pawn_moves(8, white_pawns, black_pawns, Color::White);

        let expected = (1u64 << 16) | (1u64 << 24) | (1u64 << 17);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_pawn_single_push() {
        let white_pawns = 0;
        let black_pawns = 1u64 << 50;
        let moves = get_pawn_moves(50, white_pawns, black_pawns, Color::Black);

        let expected = (1u64 << 42) | (1u64 << 34);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_pawn_double_push() {
        let white_pawns = 0;
        let black_pawns = 1u64 << 50;
        let moves = get_pawn_moves(50, white_pawns, black_pawns, Color::Black);

        let expected = (1u64 << 42) | (1u64 << 34);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_pawn_capture() {
        let white_pawns = 1u64 << 43;
        let black_pawns = 1u64 << 50;
        let moves = get_pawn_moves(50, white_pawns, black_pawns, Color::Black);

        let expected = (1u64 << 42) | (1u64 << 34) | (1u64 << 43);
        assert_eq!(moves, expected);
    }
}
