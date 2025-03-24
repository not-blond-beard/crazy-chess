use crate::bitboard::constants;
use crate::bitboard::operations::square_to_bitboard;
use crate::movement::validator;
use crate::pieces::knight;
use crate::pieces::pawn;
use crate::pieces::piece_type::{Color, MoveError, PieceType};

pub struct Board {
    pub white_pawns: u64,
    pub black_pawns: u64,
    pub white_knights: u64,
    pub black_knights: u64,
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pawns: constants::RANK_2,
            black_pawns: constants::RANK_7,
            white_knights: 0x0000_0000_0000_0042, // b1, g1
            black_knights: 0x4200_0000_0000_0000, // b8, g8
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
        println!("  White: ♙ ♘   Black: ♟ ♞");
    }

    pub fn white_pieces(&self) -> u64 {
        self.white_pawns | self.white_knights
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns | self.black_knights
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
        }

        match self.side_to_move {
            Color::White => {
                self.black_pawns &= !to_bb;
                self.black_knights &= !to_bb;
            }
            Color::Black => {
                self.white_pawns &= !to_bb;
                self.white_knights &= !to_bb;
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

        // 흰색 나이트를 b1에 두고 검은색 폰을 c3에 배치
        board.white_knights = 0x0000_0000_0000_0002; // b1 위치 (1 << 1)
        board.black_pawns = 0x0000_0000_0001_0000; // c3 위치 (1 << 16)

        assert!(board.make_move(1, 16).is_ok()); // knight from b1 captures at c3
        assert_eq!(board.white_knights & (1u64 << 16), 1u64 << 16);
        assert_eq!(board.black_pawns & (1u64 << 16), 0); // pawn should be captured
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
            board.get_piece_type_at(48),
            Some((PieceType::Pawn, Color::Black))
        );
        assert_eq!(
            board.get_piece_type_at(62),
            Some((PieceType::Knight, Color::Black))
        );
        assert_eq!(board.get_piece_type_at(20), None);
    }
}
