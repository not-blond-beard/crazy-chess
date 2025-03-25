use crate::board::Board;
use crate::game_state::check::is_in_check;
use crate::pieces::piece_type::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus {
    Ongoing,
    Check,
    Checkmate,
    Stalemate,
}

pub fn get_game_status(board: &Board) -> GameStatus {
    let side_to_move = board.side_to_move;

    if is_in_check(board, side_to_move) {
        if is_checkmate(board, side_to_move) {
            GameStatus::Checkmate
        } else {
            GameStatus::Check
        }
    } else if is_stalemate(board, side_to_move) {
        GameStatus::Stalemate
    } else {
        GameStatus::Ongoing
    }
}

pub fn is_checkmate(board: &Board, color: Color) -> bool {
    if !is_in_check(board, color) {
        return false;
    }

    has_no_legal_moves(board, color)
}

pub fn is_stalemate(board: &Board, color: Color) -> bool {
    if is_in_check(board, color) {
        return false;
    }

    has_no_legal_moves(board, color)
}

fn has_no_legal_moves(board: &Board, color: Color) -> bool {
    let pieces_bitboard = match color {
        Color::White => board.white_pieces(),
        Color::Black => board.black_pieces(),
    };

    for i in 0..64 {
        let square_bb = 1u64 << i;

        if pieces_bitboard & square_bb != 0 {
            let legal_moves = get_safe_moves(board, i);

            if legal_moves != 0 {
                return false;
            }
        }
    }

    true
}

fn get_safe_moves(board: &Board, from: usize) -> u64 {
    let original_side_to_move = board.side_to_move;
    let legal_moves = board.get_legal_moves(from);
    let mut safe_moves = 0u64;

    if legal_moves == 0 {
        return 0;
    }

    for to in 0..64 {
        let to_bb = 1u64 << to;

        if legal_moves & to_bb != 0 {
            let simulated_board = simulate_move(board, from, to);

            if !is_in_check(&simulated_board, original_side_to_move) {
                safe_moves |= to_bb;
            }
        }
    }

    safe_moves
}

fn simulate_move(board: &Board, from: usize, to: usize) -> Board {
    let from_bb = 1u64 << from;
    let to_bb = 1u64 << to;
    let moving_side = board.side_to_move;

    let mut new_board = Board {
        white_pawns: board.white_pawns,
        black_pawns: board.black_pawns,
        white_knights: board.white_knights,
        black_knights: board.black_knights,
        white_bishops: board.white_bishops,
        black_bishops: board.black_bishops,
        white_rooks: board.white_rooks,
        black_rooks: board.black_rooks,
        white_queens: board.white_queens,
        black_queens: board.black_queens,
        white_kings: board.white_kings,
        black_kings: board.black_kings,
        side_to_move: moving_side,
    };

    if let Some((_, color)) = board.get_piece_type_at(from) {
        if color != moving_side {
            return new_board;
        }

        if moving_side == Color::White {
            new_board.black_pawns &= !to_bb;
            new_board.black_knights &= !to_bb;
            new_board.black_bishops &= !to_bb;
            new_board.black_rooks &= !to_bb;
            new_board.black_queens &= !to_bb;
        } else {
            new_board.white_pawns &= !to_bb;
            new_board.white_knights &= !to_bb;
            new_board.white_bishops &= !to_bb;
            new_board.white_rooks &= !to_bb;
            new_board.white_queens &= !to_bb;
        }

        if moving_side == Color::White {
            if new_board.white_pawns & from_bb != 0 {
                new_board.white_pawns &= !from_bb;
                new_board.white_pawns |= to_bb;
            } else if new_board.white_knights & from_bb != 0 {
                new_board.white_knights &= !from_bb;
                new_board.white_knights |= to_bb;
            } else if new_board.white_bishops & from_bb != 0 {
                new_board.white_bishops &= !from_bb;
                new_board.white_bishops |= to_bb;
            } else if new_board.white_rooks & from_bb != 0 {
                new_board.white_rooks &= !from_bb;
                new_board.white_rooks |= to_bb;
            } else if new_board.white_queens & from_bb != 0 {
                new_board.white_queens &= !from_bb;
                new_board.white_queens |= to_bb;
            } else if new_board.white_kings & from_bb != 0 {
                new_board.white_kings &= !from_bb;
                new_board.white_kings |= to_bb;
            }
        } else if new_board.black_pawns & from_bb != 0 {
            new_board.black_pawns &= !from_bb;
            new_board.black_pawns |= to_bb;
        } else if new_board.black_knights & from_bb != 0 {
            new_board.black_knights &= !from_bb;
            new_board.black_knights |= to_bb;
        } else if new_board.black_bishops & from_bb != 0 {
            new_board.black_bishops &= !from_bb;
            new_board.black_bishops |= to_bb;
        } else if new_board.black_rooks & from_bb != 0 {
            new_board.black_rooks &= !from_bb;
            new_board.black_rooks |= to_bb;
        } else if new_board.black_queens & from_bb != 0 {
            new_board.black_queens &= !from_bb;
            new_board.black_queens |= to_bb;
        } else if new_board.black_kings & from_bb != 0 {
            new_board.black_kings &= !from_bb;
            new_board.black_kings |= to_bb;
        }
    }

    new_board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_game_status_is_ongoing() {
        let board = Board::new();

        assert_eq!(get_game_status(&board), GameStatus::Ongoing);
    }

    #[test]
    #[ignore]
    fn test_checkmate_status() {
        let mut board = Board::new();

        board.white_pawns = 0;
        board.black_pawns = 0;
        board.white_knights = 0;
        board.black_knights = 0;
        board.white_bishops = 0;
        board.black_bishops = 0;
        board.white_rooks = 0;
        board.white_queens = 0;
        board.black_queens = 0;

        board.white_kings = 1u64 << 0; // a1
        board.black_kings = 1u64 << 63; // h8

        board.black_rooks = (1u64 << 8) | (1u64 << 1); // a2, b1
        board.side_to_move = Color::White;

        assert_eq!(get_game_status(&board), GameStatus::Checkmate);
    }

    #[test]
    fn test_check_status() {
        let mut board = Board::new();

        board.white_kings = 1u64 << 4; // e1
        board.black_rooks = 1u64 << 36; // e5
        board.black_pawns = 0;
        board.white_pawns = 0;
        board.white_queens = 1u64 << 5; // f1
        board.side_to_move = Color::White;

        board.white_knights = 0;
        board.white_bishops = 0;
        board.white_rooks = 0;
        board.black_knights = 0;
        board.black_bishops = 0;
        board.black_queens = 0;
        board.black_kings = 1u64 << 60; // e8

        assert_eq!(get_game_status(&board), GameStatus::Check);
    }

    #[test]
    fn test_stalemate_status() {
        let mut board = Board::new();

        board.white_pawns = 0;
        board.black_pawns = 0;
        board.white_knights = 0;
        board.black_knights = 0;
        board.white_bishops = 0;
        board.black_bishops = 0;
        board.white_rooks = 0;
        board.white_queens = 0;
        board.black_queens = 0;
        board.black_rooks = 0;

        board.white_kings = 1u64 << 0; // a1
        board.black_kings = 1u64 << 63; // h8

        board.black_queens = 1u64 << 10; // c2
        board.side_to_move = Color::White;

        assert_eq!(get_game_status(&board), GameStatus::Stalemate);
    }
}
