use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::bishop;
use crate::pieces::piece_type::Color;
use crate::pieces::rook;

pub fn get_queen_moves(
    from: usize,
    white_queens: u64,
    black_queens: u64,
    white_pieces: u64,
    black_pieces: u64,
    side_to_move: Color,
) -> u64 {
    let from_bb = square_to_bitboard(from);

    let is_queen_at_square = match side_to_move {
        Color::White => white_queens & from_bb != 0,
        Color::Black => black_queens & from_bb != 0,
    };

    if !is_queen_at_square {
        return 0;
    }

    let bishop_moves = bishop::get_bishop_moves(
        from,
        from_bb,
        from_bb,
        white_pieces,
        black_pieces,
        side_to_move,
    );

    let rook_moves = rook::get_rook_moves(
        from,
        from_bb,
        from_bb,
        white_pieces,
        black_pieces,
        side_to_move,
    );

    bishop_moves | rook_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queen_middle_board() {
        let white_queens = 1u64 << 27;
        let white_pieces = white_queens;
        let black_pieces = 0;

        let moves = get_queen_moves(
            27,
            white_queens,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let diagonal_moves = (1u64 << 0)
            | (1u64 << 9)
            | (1u64 << 18)
            | (1u64 << 36)
            | (1u64 << 45)
            | (1u64 << 54)
            | (1u64 << 63)
            | (1u64 << 20)
            | (1u64 << 13)
            | (1u64 << 6)
            | (1u64 << 34)
            | (1u64 << 41)
            | (1u64 << 48);

        let orthogonal_moves = (1u64 << 3)
            | (1u64 << 11)
            | (1u64 << 19)
            | (1u64 << 35)
            | (1u64 << 43)
            | (1u64 << 51)
            | (1u64 << 59)
            | (1u64 << 24)
            | (1u64 << 25)
            | (1u64 << 26)
            | (1u64 << 28)
            | (1u64 << 29)
            | (1u64 << 30)
            | (1u64 << 31);

        let expected = diagonal_moves | orthogonal_moves;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_queen_corner() {
        let white_queens = 1u64 << 0;
        let white_pieces = white_queens;
        let black_pieces = 0;

        let moves = get_queen_moves(0, white_queens, 0, white_pieces, black_pieces, Color::White);

        let diagonal_moves = (1u64 << 9)
            | (1u64 << 18)
            | (1u64 << 27)
            | (1u64 << 36)
            | (1u64 << 45)
            | (1u64 << 54)
            | (1u64 << 63);

        let orthogonal_moves = (1u64 << 8)
            | (1u64 << 16)
            | (1u64 << 24)
            | (1u64 << 32)
            | (1u64 << 40)
            | (1u64 << 48)
            | (1u64 << 56)
            | (1u64 << 1)
            | (1u64 << 2)
            | (1u64 << 3)
            | (1u64 << 4)
            | (1u64 << 5)
            | (1u64 << 6)
            | (1u64 << 7);

        let expected = diagonal_moves | orthogonal_moves;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_queen_with_blockers() {
        let white_queens = 1u64 << 27;
        let white_pawns = (1u64 << 18) | (1u64 << 19) | (1u64 << 20);
        let black_pawns = 1u64 << 36;

        let white_pieces = white_queens | white_pawns;
        let black_pieces = black_pawns;

        let moves = get_queen_moves(
            27,
            white_queens,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = moves;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_queen() {
        let black_queens = 1u64 << 35;
        let white_pieces = 0;
        let black_pieces = black_queens;

        let moves = get_queen_moves(
            35,
            0,
            black_queens,
            white_pieces,
            black_pieces,
            Color::Black,
        );

        let diagonal_moves = (1u64 << 8)
            | (1u64 << 17)
            | (1u64 << 26)
            | (1u64 << 44)
            | (1u64 << 53)
            | (1u64 << 62)
            | (1u64 << 28)
            | (1u64 << 21)
            | (1u64 << 14)
            | (1u64 << 7)
            | (1u64 << 42)
            | (1u64 << 49)
            | (1u64 << 56);

        let orthogonal_moves = (1u64 << 3)
            | (1u64 << 11)
            | (1u64 << 19)
            | (1u64 << 27)
            | (1u64 << 43)
            | (1u64 << 51)
            | (1u64 << 59)
            | (1u64 << 32)
            | (1u64 << 33)
            | (1u64 << 34)
            | (1u64 << 36)
            | (1u64 << 37)
            | (1u64 << 38)
            | (1u64 << 39);

        let expected = diagonal_moves | orthogonal_moves;
        assert_eq!(moves, expected);
    }
}
