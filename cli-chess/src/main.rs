use clap::Parser;
use chess_engine::board::{Board, Color, MoveError};
use std::io::{self, Write};

#[derive(Parser)]
#[clap(author, version, about = "A bitboard-based chess engine CLI")]
struct Cli {
    #[clap(short, long, help = "Run in interactive mode")]
    interactive: bool,
}

fn algebraic_to_index(notation: &str) -> Option<usize> {
    if notation.len() != 2 {
        return None;
    }
    
    let file = notation.chars().nth(0)?;
    let rank = notation.chars().nth(1)?;
    
    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return None;
    }
    
    let file_idx = (file as u8 - b'a') as usize;
    let rank_idx = (rank as u8 - b'1') as usize;
    
    Some(rank_idx * 8 + file_idx)
}

fn index_to_algebraic(index: usize) -> String {
    let file = (index % 8) as u8 + b'a';
    let rank = (index / 8) as u8 + b'1';
    
    format!("{}{}", file as char, rank as char)
}

fn display_move_error(error: MoveError) {
    match error {
        MoveError::NoPieceAtSource => println!("Error: No piece at the source square"),
        MoveError::WrongColorPiece => println!("Error: That's not your piece to move"),
        MoveError::InvalidDestination => println!("Error: Invalid destination for this piece"),
        MoveError::PathBlocked => println!("Error: The path is blocked by another piece"),
        MoveError::DestinationOccupiedBySameColor => println!("Error: Destination is occupied by your own piece"),
    }
}

fn print_legal_moves(board: &Board, square: usize) {
    let moves = board.get_pawn_moves(square);
    if moves == 0 {
        println!("No legal moves for piece at {}", index_to_algebraic(square));
        return;
    }
    
    println!("Legal moves from {}:", index_to_algebraic(square));
    
    let mut move_count = 0;
    for i in 0..64 {
        if (moves & (1u64 << i)) != 0 {
            print!("{} ", index_to_algebraic(i));
            move_count += 1;
            if move_count % 8 == 0 {
                println!();
            }
        }
    }
    
    if move_count % 8 != 0 {
        println!();
    }
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("  e2e4       - Move a piece from e2 to e4");
    println!("  legal e2   - Show legal moves from square e2");
    println!("  print      - Display the current board");
    println!("  help       - Show this help message");
    println!("  quit/exit  - Exit the program\n");
}

fn run_interactive_mode() {
    let mut board = Board::new();
    
    println!("\n=== Welcome to Crazy Chess! ===\n");
    println!("A bitboard-based chess engine with an interactive CLI");
    println!("Type 'help' for a list of commands");
    
    board.print();
    
    loop {
        let side = match board.side_to_move {
            Color::White => "White (W)",
            Color::Black => "Black (B)",
        };
        
        print!("\n{} to move > ", side);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        if input == "quit" || input == "exit" {
            println!("Thanks for playing!");
            break;
        }
        
        if input == "help" {
            print_help();
            continue;
        }
        
        if input == "print" {
            board.print();
            continue;
        }
        
        if input.starts_with("legal ") {
            if let Some(square_str) = input.strip_prefix("legal ") {
                if let Some(square) = algebraic_to_index(square_str) {
                    print_legal_moves(&board, square);
                } else {
                    println!("Invalid square notation: {}. Use format like 'e2'", square_str);
                }
            }
            continue;
        }
        
        if input.len() == 4 {
            let from_str = &input[0..2];
            let to_str = &input[2..4];
            
            match (algebraic_to_index(from_str), algebraic_to_index(to_str)) {
                (Some(from), Some(to)) => {
                    match board.make_move(from, to) {
                        Ok(_) => {
                            println!("Moved from {} to {}", from_str, to_str);
                            board.print();
                        },
                        Err(err) => display_move_error(err),
                    }
                },
                _ => println!("Invalid move notation. Use format like 'e2e4'"),
            }
        } else {
            println!("Unrecognized command. Type 'help' for available commands");
        }
    }
}

fn main() {
    let args = Cli::parse();
    
    if args.interactive {
        run_interactive_mode();
    } else {
        println!("Starting with a new board:");
        let board = Board::new();
        board.print();
        println!("\nUse --interactive flag to play the game");
        println!("Example: cargo run -p cli-chess -- --interactive");
    }
}
