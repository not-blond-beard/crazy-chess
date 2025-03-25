use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::Color;

pub fn get_rook_moves(
    from: usize,
    white_rooks: u64,
    black_rooks: u64,
    white_pieces: u64,
    black_pieces: u64,
    side_to_move: Color,
) -> u64 {
    let from_bb = square_to_bitboard(from);
    let own_pieces = match side_to_move {
        Color::White => white_pieces,
        Color::Black => black_pieces,
    };
    let opponent_pieces = match side_to_move {
        Color::White => black_pieces,
        Color::Black => white_pieces,
    };

    let is_rook_at_square = match side_to_move {
        Color::White => white_rooks & from_bb != 0,
        Color::Black => black_rooks & from_bb != 0,
    };

    if !is_rook_at_square {
        return 0;
    }

    let mut moves = 0u64;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let file = from % 8;
    let rank = from / 8;

    for &(file_delta, rank_delta) in &directions {
        let mut curr_file = file as i32;
        let mut curr_rank = rank as i32;

        loop {
            curr_file += file_delta;
            curr_rank += rank_delta;

            if curr_file < 0 || curr_file > 7 || curr_rank < 0 || curr_rank > 7 {
                break;
            }

            let target = (curr_rank as usize) * 8 + (curr_file as usize);
            let target_bb = square_to_bitboard(target);

            if target_bb & own_pieces != 0 {
                break;
            }

            moves |= target_bb;

            if target_bb & opponent_pieces != 0 {
                break;
            }
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_middle_board() {
        let white_rooks = 1u64 << 27;
        let white_pieces = white_rooks;
        let black_pieces = 0;

        let moves = get_rook_moves(27, white_rooks, 0, white_pieces, black_pieces, Color::White);

        let vertical = (1u64 << 3)
            | (1u64 << 11)
            | (1u64 << 19)
            | (1u64 << 35)
            | (1u64 << 43)
            | (1u64 << 51)
            | (1u64 << 59);

        let horizontal = (1u64 << 24)
            | (1u64 << 25)
            | (1u64 << 26)
            | (1u64 << 28)
            | (1u64 << 29)
            | (1u64 << 30)
            | (1u64 << 31);

        let expected = vertical | horizontal;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_rook_corner() {
        let white_rooks = 1u64 << 0;
        let white_pieces = white_rooks;
        let black_pieces = 0;

        let moves = get_rook_moves(0, white_rooks, 0, white_pieces, black_pieces, Color::White);

        let vertical = (1u64 << 8)
            | (1u64 << 16)
            | (1u64 << 24)
            | (1u64 << 32)
            | (1u64 << 40)
            | (1u64 << 48)
            | (1u64 << 56);

        let horizontal = (1u64 << 1)
            | (1u64 << 2)
            | (1u64 << 3)
            | (1u64 << 4)
            | (1u64 << 5)
            | (1u64 << 6)
            | (1u64 << 7);

        let expected = vertical | horizontal;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_rook_with_blockers() {
        let white_rooks = 1u64 << 27;
        let white_pawns = 1u64 << 19;
        let black_pawns = 1u64 << 35;

        let white_pieces = white_rooks | white_pawns;
        let black_pieces = black_pawns;

        let moves = get_rook_moves(27, white_rooks, 0, white_pieces, black_pieces, Color::White);

        let horizontal = (1u64 << 24)
            | (1u64 << 25)
            | (1u64 << 26)
            | (1u64 << 28)
            | (1u64 << 29)
            | (1u64 << 30)
            | (1u64 << 31);

        let vertical = 1u64 << 35;

        let expected = horizontal | vertical;
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_rook() {
        let black_rooks = 1u64 << 35;
        let white_pieces = 0;
        let black_pieces = black_rooks;

        let moves = get_rook_moves(35, 0, black_rooks, white_pieces, black_pieces, Color::Black);

        let vertical = (1u64 << 3)
            | (1u64 << 11)
            | (1u64 << 19)
            | (1u64 << 27)
            | (1u64 << 43)
            | (1u64 << 51)
            | (1u64 << 59);

        let horizontal = (1u64 << 32)
            | (1u64 << 33)
            | (1u64 << 34)
            | (1u64 << 36)
            | (1u64 << 37)
            | (1u64 << 38)
            | (1u64 << 39);

        let expected = vertical | horizontal;
        assert_eq!(moves, expected);
    }
}
