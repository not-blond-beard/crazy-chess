use crate::bitboard::operations::square_to_bitboard;
use crate::board::Board;
use crate::pieces::piece_type::Color;

pub fn is_in_check(board: &Board, color: Color) -> bool {
    let king_position = find_king_position(board, color);
    if king_position == 64 {
        return false;
    }

    check_pawn_attack(board, king_position, color)
        || check_knight_attack(board, king_position, color)
        || check_bishop_attack(board, king_position, color)
        || check_rook_attack(board, king_position, color)
}

pub fn find_king_position(board: &Board, color: Color) -> usize {
    let king_bitboard = match color {
        Color::White => board.white_kings,
        Color::Black => board.black_kings,
    };

    for i in 0..64 {
        if king_bitboard & (1u64 << i) != 0 {
            return i;
        }
    }

    64
}

fn check_pawn_attack(board: &Board, king_pos: usize, king_color: Color) -> bool {
    let king_file = king_pos % 8;
    let king_rank = king_pos / 8;

    match king_color {
        Color::White => {
            let attack_pos = [
                if king_file > 0 && king_rank < 7 {
                    Some(king_pos + 7)
                } else {
                    None
                },
                if king_file < 7 && king_rank < 7 {
                    Some(king_pos + 9)
                } else {
                    None
                },
            ];

            for pos in attack_pos.iter().flatten() {
                let pos_bb = square_to_bitboard(*pos);
                if board.black_pawns & pos_bb != 0 {
                    return true;
                }
            }
        }
        Color::Black => {
            let attack_pos = [
                if king_file > 0 && king_rank > 0 {
                    Some(king_pos - 9)
                } else {
                    None
                },
                if king_file < 7 && king_rank > 0 {
                    Some(king_pos - 7)
                } else {
                    None
                },
            ];

            for pos in attack_pos.iter().flatten() {
                let pos_bb = square_to_bitboard(*pos);
                if board.white_pawns & pos_bb != 0 {
                    return true;
                }
            }
        }
    }

    false
}

fn check_knight_attack(board: &Board, king_pos: usize, king_color: Color) -> bool {
    let king_file = king_pos % 8;
    let king_rank = king_pos / 8;

    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    let enemy_knights = match king_color {
        Color::White => board.black_knights,
        Color::Black => board.white_knights,
    };

    for (file_offset, rank_offset) in offsets.iter() {
        let new_file = king_file as i32 + file_offset;
        let new_rank = king_rank as i32 + rank_offset;

        if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
            let pos = (new_rank as usize) * 8 + (new_file as usize);
            let pos_bb = square_to_bitboard(pos);

            if enemy_knights & pos_bb != 0 {
                return true;
            }
        }
    }

    false
}

fn check_bishop_attack(board: &Board, king_pos: usize, king_color: Color) -> bool {
    let king_file = king_pos % 8;
    let king_rank = king_pos / 8;

    let enemy_bishops = match king_color {
        Color::White => board.black_bishops,
        Color::Black => board.white_bishops,
    };

    let enemy_queens = match king_color {
        Color::White => board.black_queens,
        Color::Black => board.white_queens,
    };

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for &(file_dir, rank_dir) in &directions {
        let mut curr_file = king_file as i32 + file_dir;
        let mut curr_rank = king_rank as i32 + rank_dir;

        while (0..8).contains(&curr_file) && (0..8).contains(&curr_rank) {
            let pos = (curr_rank as usize) * 8 + (curr_file as usize);
            let pos_bb = square_to_bitboard(pos);

            if board.all_pieces() & pos_bb != 0 {
                if (enemy_bishops | enemy_queens) & pos_bb != 0 {
                    return true;
                }
                break;
            }

            curr_file += file_dir;
            curr_rank += rank_dir;
        }
    }

    false
}

fn check_rook_attack(board: &Board, king_pos: usize, king_color: Color) -> bool {
    let king_file = king_pos % 8;
    let king_rank = king_pos / 8;

    let enemy_rooks = match king_color {
        Color::White => board.black_rooks,
        Color::Black => board.white_rooks,
    };

    let enemy_queens = match king_color {
        Color::White => board.black_queens,
        Color::Black => board.white_queens,
    };

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    for &(file_dir, rank_dir) in &directions {
        let mut curr_file = king_file as i32 + file_dir;
        let mut curr_rank = king_rank as i32 + rank_dir;

        while (0..8).contains(&curr_file) && (0..8).contains(&curr_rank) {
            let pos = (curr_rank as usize) * 8 + (curr_file as usize);
            let pos_bb = square_to_bitboard(pos);

            if board.all_pieces() & pos_bb != 0 {
                if (enemy_rooks | enemy_queens) & pos_bb != 0 {
                    return true;
                }
                break;
            }

            curr_file += file_dir;
            curr_rank += rank_dir;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_king_position() {
        let board = Board::new();

        let white_king_pos = find_king_position(&board, Color::White);
        assert_eq!(white_king_pos, 4); // e1

        let black_king_pos = find_king_position(&board, Color::Black);
        assert_eq!(black_king_pos, 60); // e8
    }

    #[test]
    fn test_is_not_in_check_initially() {
        let board = Board::new();

        assert!(!is_in_check(&board, Color::White));
        assert!(!is_in_check(&board, Color::Black));
    }

    #[test]
    fn test_is_in_check_by_queen() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_queens = 1u64 << 12; // e2
        board.black_pawns = 0;

        assert!(is_in_check(&board, Color::White));
    }

    #[test]
    fn test_is_in_check_by_rook() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_rooks = 1u64 << 36; // e5
        board.black_pawns = 0;
        board.white_pawns = 0;

        assert!(is_in_check(&board, Color::White));
    }

    #[test]
    fn test_is_not_in_check_with_blocked_path() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_rooks = 1u64 << 36; // e5
        board.white_pawns = 1u64 << 12; // e2

        assert!(!is_in_check(&board, Color::White));
    }

    #[test]
    fn test_is_in_check_by_knight() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_knights = 1u64 << 14; // g2

        assert!(is_in_check(&board, Color::White));
    }

    #[test]
    fn test_is_in_check_by_bishop() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_bishops = 1u64 << 31; // h4
        board.white_pawns = 0;

        assert!(is_in_check(&board, Color::White));
    }

    #[test]
    fn test_is_in_check_by_pawn() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 28; // e4
        board.black_pawns = 1u64 << 37; // f5

        assert!(is_in_check(&board, Color::White));
    }
}
