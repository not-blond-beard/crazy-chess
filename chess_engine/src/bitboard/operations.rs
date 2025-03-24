pub fn square_to_bitboard(square: usize) -> u64 {
    1u64 << square
}

pub fn bitboard_to_square(bitboard: u64) -> Option<usize> {
    if bitboard == 0 || (bitboard & (bitboard - 1)) != 0 {
        return None;
    }
    Some(bitboard.trailing_zeros() as usize)
}

pub fn set_bit(bitboard: &mut u64, square: usize) {
    *bitboard |= square_to_bitboard(square);
}

pub fn clear_bit(bitboard: &mut u64, square: usize) {
    *bitboard &= !square_to_bitboard(square);
}

pub fn test_bit(bitboard: u64, square: usize) -> bool {
    (bitboard & square_to_bitboard(square)) != 0
}

pub fn count_bits(mut bitboard: u64) -> u32 {
    let mut count = 0;
    while bitboard != 0 {
        count += 1;
        bitboard &= bitboard - 1;
    }
    count
}

pub fn get_lsb_index(bitboard: u64) -> Option<usize> {
    if bitboard == 0 {
        return None;
    }
    Some(bitboard.trailing_zeros() as usize)
}
