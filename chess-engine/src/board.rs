use crate::bitboard::constants;
use crate::bitboard::operations::square_to_bitboard;
use crate::movement::validator;
use crate::pieces::bishop;
use crate::pieces::king;
use crate::pieces::knight;
use crate::pieces::pawn;
use crate::pieces::piece_type::{Color, MoveError, PieceType};
use crate::pieces::queen;
use crate::pieces::rook;

pub struct Board {
    pub white_pawns: u64,
    pub black_pawns: u64,
    pub white_knights: u64,
    pub black_knights: u64,
    pub white_bishops: u64,
    pub black_bishops: u64,
    pub white_rooks: u64,
    pub black_rooks: u64,
    pub white_kings: u64,
    pub black_kings: u64,
    pub white_queens: u64,
    pub black_queens: u64,
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pawns: constants::RANK_2,
            black_pawns: constants::RANK_7,
            white_knights: 0x0000_0000_0000_0042, // b1, g1
            black_knights: 0x4200_0000_0000_0000, // b8, g8
            white_bishops: 0x0000_0000_0000_0024, // c1, f1
            black_bishops: 0x2400_0000_0000_0000, // c8, f8
            white_rooks: 0x0000_0000_0000_0081,   // a1, h1
            black_rooks: 0x8100_0000_0000_0000,   // a8, h8
            white_kings: 0x0000_0000_0000_0010,   // e1
            black_kings: 0x1000_0000_0000_0000,   // e8
            white_queens: 0x0000_0000_0000_0008,  // d1
            black_queens: 0x0800_0000_0000_0000,  // d8
            side_to_move: Color::White,
        }
    }

    pub fn print(&self) {
        println!("  +---+---+---+---+---+---+---+---+");
        for rank in (0..8).rev() {
            print!("{} |", rank + 1);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let mask = 1u64 << sq;
                let is_dark_square = (rank + file) % 2 == 1;

                let symbol = if self.white_pawns & mask != 0 {
                    " ♙ "
                } else if self.black_pawns & mask != 0 {
                    " ♟ "
                } else if self.white_knights & mask != 0 {
                    " ♘ "
                } else if self.black_knights & mask != 0 {
                    " ♞ "
                } else if self.white_bishops & mask != 0 {
                    " ♗ "
                } else if self.black_bishops & mask != 0 {
                    " ♝ "
                } else if self.white_rooks & mask != 0 {
                    " ♖ "
                } else if self.black_rooks & mask != 0 {
                    " ♜ "
                } else if self.white_kings & mask != 0 {
                    " ♔ "
                } else if self.black_kings & mask != 0 {
                    " ♚ "
                } else if self.white_queens & mask != 0 {
                    " ♕ "
                } else if self.black_queens & mask != 0 {
                    " ♛ "
                } else {
                    if is_dark_square {
                        "░░░"
                    } else {
                        "   "
                    }
                };

                print!("{}", symbol);
                print!("|");
            }
            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }
        println!("    a   b   c   d   e   f   g   h  ");
        println!();
        println!("  White: ♙ ♘ ♗ ♖ ♔ ♕   Black: ♟ ♞ ♝ ♜ ♚ ♛");
    }

    pub fn white_pieces(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_kings
            | self.white_queens
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_kings
            | self.black_queens
    }

    pub fn all_pieces(&self) -> u64 {
        self.white_pieces() | self.black_pieces()
    }

    pub fn toggle_side_to_move(&mut self) {
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn get_piece_type_at(&self, square: usize) -> Option<(PieceType, Color)> {
        let bb = square_to_bitboard(square);

        if self.white_pawns & bb != 0 {
            Some((PieceType::Pawn, Color::White))
        } else if self.black_pawns & bb != 0 {
            Some((PieceType::Pawn, Color::Black))
        } else if self.white_knights & bb != 0 {
            Some((PieceType::Knight, Color::White))
        } else if self.black_knights & bb != 0 {
            Some((PieceType::Knight, Color::Black))
        } else if self.white_bishops & bb != 0 {
            Some((PieceType::Bishop, Color::White))
        } else if self.black_bishops & bb != 0 {
            Some((PieceType::Bishop, Color::Black))
        } else if self.white_rooks & bb != 0 {
            Some((PieceType::Rook, Color::White))
        } else if self.black_rooks & bb != 0 {
            Some((PieceType::Rook, Color::Black))
        } else if self.white_kings & bb != 0 {
            Some((PieceType::King, Color::White))
        } else if self.black_kings & bb != 0 {
            Some((PieceType::King, Color::Black))
        } else if self.white_queens & bb != 0 {
            Some((PieceType::Queen, Color::White))
        } else if self.black_queens & bb != 0 {
            Some((PieceType::Queen, Color::Black))
        } else {
            None
        }
    }

    pub fn get_legal_moves(&self, from: usize) -> u64 {
        if let Some((piece_type, color)) = self.get_piece_type_at(from) {
            if color != self.side_to_move {
                return 0;
            }

            match piece_type {
                PieceType::Pawn => pawn::get_pawn_moves(
                    from,
                    self.white_pawns,
                    self.black_pawns,
                    self.side_to_move,
                ),
                PieceType::Knight => knight::get_knight_moves(
                    from,
                    self.white_knights,
                    self.black_knights,
                    self.white_pieces(),
                    self.black_pieces(),
                    self.side_to_move,
                ),
                PieceType::Bishop => bishop::get_bishop_moves(
                    from,
                    self.white_bishops,
                    self.black_bishops,
                    self.white_pieces(),
                    self.black_pieces(),
                    self.side_to_move,
                ),
                PieceType::Rook => rook::get_rook_moves(
                    from,
                    self.white_rooks,
                    self.black_rooks,
                    self.white_pieces(),
                    self.black_pieces(),
                    self.side_to_move,
                ),
                PieceType::King => king::get_king_moves(
                    from,
                    self.white_kings,
                    self.black_kings,
                    self.white_pieces(),
                    self.black_pieces(),
                    self.side_to_move,
                ),
                PieceType::Queen => queen::get_queen_moves(
                    from,
                    self.white_queens,
                    self.black_queens,
                    self.white_pieces(),
                    self.black_pieces(),
                    self.side_to_move,
                ),
            }
        } else {
            0
        }
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<(), MoveError> {
        let legal_moves = self.get_legal_moves(from);

        validator::validate_move(
            from,
            to,
            self.side_to_move,
            self.white_pieces(),
            self.black_pieces(),
            legal_moves,
        )?;

        let from_bb = square_to_bitboard(from);
        let to_bb = square_to_bitboard(to);

        let piece_opt = self.get_piece_type_at(from);
        if piece_opt.is_none() {
            return Err(MoveError::NoPieceAtSource);
        }

        let (piece_type, _) = piece_opt.unwrap();

        match (self.side_to_move, piece_type) {
            (Color::White, PieceType::Pawn) => {
                self.white_pawns &= !from_bb;
                self.white_pawns |= to_bb;
            }
            (Color::Black, PieceType::Pawn) => {
                self.black_pawns &= !from_bb;
                self.black_pawns |= to_bb;
            }
            (Color::White, PieceType::Knight) => {
                self.white_knights &= !from_bb;
                self.white_knights |= to_bb;
            }
            (Color::Black, PieceType::Knight) => {
                self.black_knights &= !from_bb;
                self.black_knights |= to_bb;
            }
            (Color::White, PieceType::Bishop) => {
                self.white_bishops &= !from_bb;
                self.white_bishops |= to_bb;
            }
            (Color::Black, PieceType::Bishop) => {
                self.black_bishops &= !from_bb;
                self.black_bishops |= to_bb;
            }
            (Color::White, PieceType::Rook) => {
                self.white_rooks &= !from_bb;
                self.white_rooks |= to_bb;
            }
            (Color::Black, PieceType::Rook) => {
                self.black_rooks &= !from_bb;
                self.black_rooks |= to_bb;
            }
            (Color::White, PieceType::King) => {
                self.white_kings &= !from_bb;
                self.white_kings |= to_bb;
            }
            (Color::Black, PieceType::King) => {
                self.black_kings &= !from_bb;
                self.black_kings |= to_bb;
            }
            (Color::White, PieceType::Queen) => {
                self.white_queens &= !from_bb;
                self.white_queens |= to_bb;
            }
            (Color::Black, PieceType::Queen) => {
                self.black_queens &= !from_bb;
                self.black_queens |= to_bb;
            }
        }

        match self.side_to_move {
            Color::White => {
                self.black_pawns &= !to_bb;
                self.black_knights &= !to_bb;
                self.black_bishops &= !to_bb;
                self.black_rooks &= !to_bb;
                self.black_kings &= !to_bb;
                self.black_queens &= !to_bb;
            }
            Color::Black => {
                self.white_pawns &= !to_bb;
                self.white_knights &= !to_bb;
                self.white_bishops &= !to_bb;
                self.white_rooks &= !to_bb;
                self.white_kings &= !to_bb;
                self.white_queens &= !to_bb;
            }
        }

        self.toggle_side_to_move();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board_state() {
        let board = Board::new();
        assert_eq!(board.white_pawns, constants::RANK_2);
        assert_eq!(board.black_pawns, constants::RANK_7);
        assert_eq!(board.white_knights, 0x0000_0000_0000_0042);
        assert_eq!(board.black_knights, 0x4200_0000_0000_0000);
        assert_eq!(board.white_bishops, 0x0000_0000_0000_0024);
        assert_eq!(board.black_bishops, 0x2400_0000_0000_0000);
        assert_eq!(board.white_rooks, 0x0000_0000_0000_0081);
        assert_eq!(board.black_rooks, 0x8100_0000_0000_0000);
        assert_eq!(board.white_kings, 0x0000_0000_0000_0010);
        assert_eq!(board.black_kings, 0x1000_0000_0000_0000);
        assert_eq!(board.white_queens, 0x0000_0000_0000_0008);
        assert_eq!(board.black_queens, 0x0800_0000_0000_0000);
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_knight_move() {
        let mut board = Board::new();

        assert!(board.make_move(1, 16).is_ok()); // b1 to c3
        assert_eq!(board.white_knights & (1u64 << 16), 1u64 << 16);
        assert_eq!(board.white_knights & (1u64 << 1), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_knight_capture() {
        let mut board = Board::new();

        board.white_knights = 0x0000_0000_0000_0002;
        board.black_pawns = 0x0000_0000_0001_0000;

        assert!(board.make_move(1, 16).is_ok());
        assert_eq!(board.white_knights & (1u64 << 16), 1u64 << 16);
        assert_eq!(board.black_pawns & (1u64 << 16), 0);
    }

    #[test]
    fn test_bishop_move() {
        let mut board = Board::new();

        board.white_bishops = 1u64 << 27;
        board.white_pawns = 0;
        board.black_pawns = 0;

        assert!(board.make_move(27, 36).is_ok());
        assert_eq!(board.white_bishops & (1u64 << 36), 1u64 << 36);
        assert_eq!(board.white_bishops & (1u64 << 27), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_bishop_capture() {
        let mut board = Board::new();

        board.white_bishops = 0x0000_0000_0000_0004;
        board.white_pawns = 0;
        board.black_pawns = 0x0000_0000_0020_0000;

        assert!(board.make_move(2, 29).is_ok());
        assert_eq!(board.white_bishops & (1u64 << 29), 1u64 << 29);
        assert_eq!(board.black_pawns & (1u64 << 29), 0);
    }

    #[test]
    fn test_rook_move() {
        let mut board = Board::new();

        board.white_rooks = 1u64 << 27;
        board.white_pawns = 0;
        board.black_pawns = 0;

        assert!(board.make_move(27, 31).is_ok());
        assert_eq!(board.white_rooks & (1u64 << 31), 1u64 << 31);
        assert_eq!(board.white_rooks & (1u64 << 27), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_rook_capture() {
        let mut board = Board::new();

        board.white_rooks = 1u64 << 0;
        board.white_pawns = 0;
        board.black_pawns = 1u64 << 8;

        assert!(board.make_move(0, 8).is_ok());
        assert_eq!(board.white_rooks & (1u64 << 8), 1u64 << 8);
        assert_eq!(board.black_pawns & (1u64 << 8), 0);
    }

    #[test]
    fn test_get_piece_type() {
        let board = Board::new();

        assert_eq!(
            board.get_piece_type_at(8),
            Some((PieceType::Pawn, Color::White))
        );
        assert_eq!(
            board.get_piece_type_at(1),
            Some((PieceType::Knight, Color::White))
        );
        assert_eq!(
            board.get_piece_type_at(2),
            Some((PieceType::Bishop, Color::White))
        );
        assert_eq!(
            board.get_piece_type_at(0),
            Some((PieceType::Rook, Color::White))
        );
        assert_eq!(
            board.get_piece_type_at(48),
            Some((PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece_type_at(57),
            Some((PieceType::Knight, Color::Black))
        );
        assert_eq!(
            board.get_piece_type_at(58),
            Some((PieceType::Bishop, Color::Black))
        );
        assert_eq!(
            board.get_piece_type_at(56),
            Some((PieceType::Rook, Color::Black))
        );
        assert_eq!(board.get_piece_type_at(20), None);
    }

    #[test]
    fn test_king_move() {
        let mut board = Board::new();

        board.white_pawns &= !(1u64 << 12);

        board.make_move(4, 12).unwrap();

        assert_eq!(board.white_kings, 1u64 << 12);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_king_capture() {
        let mut board = Board::new();

        board.white_pawns &= !(1u64 << 12);
        board.black_pawns = (board.black_pawns & !(1u64 << 52)) | (1u64 << 12);

        board.make_move(4, 12).unwrap();

        assert_eq!(board.white_kings, 1u64 << 12);
        assert_eq!(board.black_pawns & (1u64 << 12), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_queen_move() {
        let mut board = Board::new();

        board.white_pawns &= !(1u64 << 11);
        board.white_queens = 1u64 << 3;

        board.make_move(3, 19).unwrap();

        assert_eq!(board.white_queens, 1u64 << 19);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_queen_capture() {
        let mut board = Board::new();

        board.white_pawns &= !(1u64 << 11);
        board.black_pawns = (board.black_pawns & !(1u64 << 51)) | (1u64 << 11);
        board.white_queens = 1u64 << 3;

        board.make_move(3, 11).unwrap();

        assert_eq!(board.white_queens, 1u64 << 11);
        assert_eq!(board.black_pawns & (1u64 << 11), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_queen_diagonal_move() {
        let mut board = Board::new();

        board.white_pawns &= !(1u64 << 10);
        board.white_pawns &= !(1u64 << 12);
        board.white_queens = 1u64 << 3;

        board.make_move(3, 10).unwrap();

        assert_eq!(board.white_queens, 1u64 << 10);
        assert_eq!(board.side_to_move, Color::Black);
    }
}
