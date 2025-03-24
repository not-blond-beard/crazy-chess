# Crazy Chess CLI

A command-line interface to play chess using the Crazy Chess engine.

## Features

- Bitboard-based chess representation for efficient move generation
- Interactive command-line interface
- Algebraic notation support
- Move validation
- Pawn movement with correct rules (single push, double push, captures)

## How to Run

```bash
# From the project root
cargo run -p cli-chess -- --interactive

# Or from this directory
cargo run -- --interactive
```

## Commands

- `e2e4` - Move a piece from e2 to e4
- `legal e2` - Show legal moves from the piece at square e2
- `print` - Display the current board
- `quit` or `exit` - Exit the program

## Implementation Details

The CLI interfaces with the chess engine to provide a complete chess experience. It currently supports:

- Full pawn movement rules
- Turn-based play (white/black alternating)
- Move validation with error handling
- Algebraic notation for chess squares

## Planned Features

- Support for all chess pieces
- Special moves (castling, en passant)
- Check and checkmate detection
- PGN export/import
- Game history and move tracking 
