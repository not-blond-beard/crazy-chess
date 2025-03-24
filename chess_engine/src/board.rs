pub struct Board {
    pub white_pawns: u64,
    pub black_pawns: u64,
    pub side_to_move: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NoPieceAtSource,
    WrongColorPiece,
    InvalidDestination,
    PathBlocked,
    DestinationOccupiedBySameColor,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_ff00,
            black_pawns: 0x00ff_0000_0000_0000,
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
        println!("  White: ♙  Black: ♟");
    }

    pub fn all_pieces(&self) -> u64 {
        self.white_pawns | self.black_pawns
    }

    pub fn square_to_bitboard(square: usize) -> u64 {
        1u64 << square
    }

    pub fn bitboard_to_square(bitboard: u64) -> Option<usize> {
        if bitboard == 0 || (bitboard & (bitboard - 1)) != 0 {
            return None; // Not exactly one bit set
        }
        Some(bitboard.trailing_zeros() as usize)
    }

    pub fn toggle_side_to_move(&mut self) {
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    // Get all legal pawn moves from a given square
    pub fn get_pawn_moves(&self, from: usize) -> u64 {
        let from_bb = Self::square_to_bitboard(from);
        let occupied = self.all_pieces();
        let mut moves = 0u64;

        match self.side_to_move {
            Color::White => {
                // Check if the source has a white pawn
                if self.white_pawns & from_bb == 0 {
                    return 0;
                }

                // Single push
                let single_push = from_bb << 8;
                if single_push & occupied == 0 {
                    moves |= single_push;

                    // Double push (only from rank 2)
                    if from / 8 == 1 {
                        let double_push = single_push << 8;
                        if double_push & occupied == 0 {
                            moves |= double_push;
                        }
                    }
                }

                // Captures
                if from % 8 != 0 { // Not on a-file
                    let capture_left = from_bb << 7;
                    if capture_left & self.black_pawns != 0 {
                        moves |= capture_left;
                    }
                }
                if from % 8 != 7 { // Not on h-file
                    let capture_right = from_bb << 9;
                    if capture_right & self.black_pawns != 0 {
                        moves |= capture_right;
                    }
                }
            },
            Color::Black => {
                // Check if the source has a black pawn
                if self.black_pawns & from_bb == 0 {
                    return 0;
                }

                // Single push
                let single_push = from_bb >> 8;
                if single_push & occupied == 0 {
                    moves |= single_push;

                    // Double push (only from rank 7)
                    if from / 8 == 6 {
                        let double_push = single_push >> 8;
                        if double_push & occupied == 0 {
                            moves |= double_push;
                        }
                    }
                }

                // Captures
                if from % 8 != 7 { // Not on h-file
                    let capture_right = from_bb >> 7;
                    if capture_right & self.white_pawns != 0 {
                        moves |= capture_right;
                    }
                }
                if from % 8 != 0 { // Not on a-file
                    let capture_left = from_bb >> 9;
                    if capture_left & self.white_pawns != 0 {
                        moves |= capture_left;
                    }
                }
            }
        }

        moves
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<(), MoveError> {
        let from_bb = Self::square_to_bitboard(from);
        let to_bb = Self::square_to_bitboard(to);
        
        // Check if there's a piece at the source square
        let is_white_piece = self.white_pawns & from_bb != 0;
        let is_black_piece = self.black_pawns & from_bb != 0;
        
        if !is_white_piece && !is_black_piece {
            return Err(MoveError::NoPieceAtSource);
        }
        
        // Check if the piece belongs to the side to move
        match self.side_to_move {
            Color::White if !is_white_piece => return Err(MoveError::WrongColorPiece),
            Color::Black if !is_black_piece => return Err(MoveError::WrongColorPiece),
            _ => {}
        }
        
        // Check if the move is legal
        let legal_moves = self.get_pawn_moves(from);
        if legal_moves & to_bb == 0 {
            return Err(MoveError::InvalidDestination);
        }
        
        // Make the move
        match self.side_to_move {
            Color::White => {
                self.white_pawns &= !from_bb;
                self.white_pawns |= to_bb;
                self.black_pawns &= !to_bb; // Capture any black piece at destination
            },
            Color::Black => {
                self.black_pawns &= !from_bb;
                self.black_pawns |= to_bb;
                self.white_pawns &= !to_bb; // Capture any white piece at destination
            }
        }
        
        // Switch side to move
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
        assert_eq!(board.white_pawns, 0x0000_0000_0000_ff00);
        assert_eq!(board.black_pawns, 0x00ff_0000_0000_0000);
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_white_pawn_single_push() {
        let mut board = Board::new();
        assert!(board.make_move(8, 16).is_ok());
        assert_eq!(board.white_pawns & (1u64 << 16), 1u64 << 16);
        assert_eq!(board.white_pawns & (1u64 << 8), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_white_pawn_double_push() {
        let mut board = Board::new();
        assert!(board.make_move(8, 24).is_ok());
        assert_eq!(board.white_pawns & (1u64 << 24), 1u64 << 24);
        assert_eq!(board.white_pawns & (1u64 << 8), 0);
        assert_eq!(board.side_to_move, Color::Black);
    }

    #[test]
    fn test_black_pawn_single_push() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;
        assert!(board.make_move(48, 40).is_ok());
        assert_eq!(board.black_pawns & (1u64 << 40), 1u64 << 40);
        assert_eq!(board.black_pawns & (1u64 << 48), 0);
        assert_eq!(board.side_to_move, Color::White);
    }

    #[test]
    fn test_pawn_capture() {
        let mut board = Board::new();
        // Set up a capture scenario
        board.white_pawns = 0;
        board.black_pawns = 0;
        board.white_pawns |= 1u64 << 18; // e3
        board.black_pawns |= 1u64 << 27; // d4
        
        assert!(board.make_move(18, 27).is_ok());
        assert_eq!(board.white_pawns & (1u64 << 27), 1u64 << 27);
        assert_eq!(board.black_pawns & (1u64 << 27), 0);
    }

    #[test]
    fn test_wrong_color_piece() {
        let mut board = Board::new();
        board.side_to_move = Color::White;
        assert_eq!(board.make_move(48, 40).unwrap_err(), MoveError::WrongColorPiece);
    }

    #[test]
    fn test_invalid_pawn_move() {
        let mut board = Board::new();
        // Try to move diagonally without capture
        assert_eq!(board.make_move(8, 17).unwrap_err(), MoveError::InvalidDestination);
    }
}
