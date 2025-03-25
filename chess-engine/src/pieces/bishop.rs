use crate::bitboard::operations::square_to_bitboard;
use crate::pieces::piece_type::Color;

pub fn get_bishop_moves(
    from: usize,
    white_bishops: u64,
    black_bishops: u64,
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

    let is_bishop_at_square = match side_to_move {
        Color::White => white_bishops & from_bb != 0,
        Color::Black => black_bishops & from_bb != 0,
    };

    if !is_bishop_at_square {
        return 0;
    }

    let mut moves = 0u64;
    
    let directions = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

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
    fn test_bishop_middle_board() {
        let white_bishops = 1u64 << 27;
        let white_pieces = white_bishops;
        let black_pieces = 0;

        let moves = get_bishop_moves(
            27,
            white_bishops,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 18) | 
            (1u64 << 9) |  
            (1u64 << 0) |  
            (1u64 << 36) | 
            (1u64 << 45) | 
            (1u64 << 54) | 
            (1u64 << 63) | 
            (1u64 << 20) | 
            (1u64 << 13) | 
            (1u64 << 6) |  
            (1u64 << 34) | 
            (1u64 << 41) | 
            (1u64 << 48);  

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_bishop_corner() {
        let white_bishops = 1u64 << 0;
        let white_pieces = white_bishops;
        let black_pieces = 0;

        let moves = get_bishop_moves(
            0,
            white_bishops,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 9) |  
            (1u64 << 18) | 
            (1u64 << 27) | 
            (1u64 << 36) | 
            (1u64 << 45) | 
            (1u64 << 54) | 
            (1u64 << 63);  

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_bishop_with_blockers() {
        let white_bishops = 1u64 << 27;
        let white_pawns = 1u64 << 18;
        let black_pawns = 1u64 << 36;

        let white_pieces = white_bishops | white_pawns;
        let black_pieces = black_pawns;

        let moves = get_bishop_moves(
            27,
            white_bishops,
            0,
            white_pieces,
            black_pieces,
            Color::White,
        );

        let expected = 
            (1u64 << 20) | 
            (1u64 << 13) | 
            (1u64 << 6) |  
            (1u64 << 36) | 
            (1u64 << 34) | 
            (1u64 << 41) | 
            (1u64 << 48);  

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_black_bishop() {
        let black_bishops = 1u64 << 35;
        let white_pieces = 0;
        let black_pieces = black_bishops;

        let moves = get_bishop_moves(
            35,
            0,
            black_bishops,
            white_pieces,
            black_pieces,
            Color::Black,
        );

        let expected = 
            (1u64 << 26) | 
            (1u64 << 17) | 
            (1u64 << 8) |  
            (1u64 << 44) | 
            (1u64 << 53) | 
            (1u64 << 62) | 
            (1u64 << 28) | 
            (1u64 << 21) | 
            (1u64 << 14) | 
            (1u64 << 7) |  
            (1u64 << 42) | 
            (1u64 << 49) | 
            (1u64 << 56);  

        assert_eq!(moves, expected);
    }
}
