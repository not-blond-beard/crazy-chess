use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    None,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl From<Square> for u64 {
    fn from(square: Square) -> u64 {
        match square {
            Square::None => 0,
            Square::A1 => 1u64,
            Square::B1 => 1u64 << 1,
            Square::C1 => 1u64 << 2,
            Square::D1 => 1u64 << 3,
            Square::E1 => 1u64 << 4,
            Square::F1 => 1u64 << 5,
            Square::G1 => 1u64 << 6,
            Square::H1 => 1u64 << 7,
            Square::A2 => 1u64 << 8,
            Square::B2 => 1u64 << 9,
            Square::C2 => 1u64 << 10,
            Square::D2 => 1u64 << 11,
            Square::E2 => 1u64 << 12,
            Square::F2 => 1u64 << 13,
            Square::G2 => 1u64 << 14,
            Square::H2 => 1u64 << 15,
            Square::A3 => 1u64 << 16,
            Square::B3 => 1u64 << 17,
            Square::C3 => 1u64 << 18,
            Square::D3 => 1u64 << 19,
            Square::E3 => 1u64 << 20,
            Square::F3 => 1u64 << 21,
            Square::G3 => 1u64 << 22,
            Square::H3 => 1u64 << 23,
            Square::A4 => 1u64 << 24,
            Square::B4 => 1u64 << 25,
            Square::C4 => 1u64 << 26,
            Square::D4 => 1u64 << 27,
            Square::E4 => 1u64 << 28,
            Square::F4 => 1u64 << 29,
            Square::G4 => 1u64 << 30,
            Square::H4 => 1u64 << 31,
            Square::A5 => 1u64 << 32,
            Square::B5 => 1u64 << 33,
            Square::C5 => 1u64 << 34,
            Square::D5 => 1u64 << 35,
            Square::E5 => 1u64 << 36,
            Square::F5 => 1u64 << 37,
            Square::G5 => 1u64 << 38,
            Square::H5 => 1u64 << 39,
            Square::A6 => 1u64 << 40,
            Square::B6 => 1u64 << 41,
            Square::C6 => 1u64 << 42,
            Square::D6 => 1u64 << 43,
            Square::E6 => 1u64 << 44,
            Square::F6 => 1u64 << 45,
            Square::G6 => 1u64 << 46,
            Square::H6 => 1u64 << 47,
            Square::A7 => 1u64 << 48,
            Square::B7 => 1u64 << 49,
            Square::C7 => 1u64 << 50,
            Square::D7 => 1u64 << 51,
            Square::E7 => 1u64 << 52,
            Square::F7 => 1u64 << 53,
            Square::G7 => 1u64 << 54,
            Square::H7 => 1u64 << 55,
            Square::A8 => 1u64 << 56,
            Square::B8 => 1u64 << 57,
            Square::C8 => 1u64 << 58,
            Square::D8 => 1u64 << 59,
            Square::E8 => 1u64 << 60,
            Square::F8 => 1u64 << 61,
            Square::G8 => 1u64 << 62,
            Square::H8 => 1u64 << 63,
        }
    }
}

impl From<u64> for Square {
    fn from(value: u64) -> Self {
        match (value & (1 << value.trailing_zeros() as u64)).trailing_zeros() {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            64.. => Square::None,
        }
    }
}

impl BitAnd<Square> for u64 {
    type Output = u64;

    fn bitand(self, rhs: Square) -> Self::Output {
        let lhs_u64: u64 = self;
        let rhs_u64: u64 = rhs.into();
        lhs_u64 & rhs_u64
    }
}

impl BitOr<Square> for u64 {
    type Output = u64;

    fn bitor(self, rhs: Square) -> Self::Output {
        let lhs_u64: u64 = self;
        let rhs_u64: u64 = rhs.into();
        lhs_u64 | rhs_u64
    }
}

impl BitXor<Square> for u64 {
    type Output = u64;

    fn bitxor(self, rhs: Square) -> Self::Output {
        let lhs_u64: u64 = self;
        let rhs_u64: u64 = rhs.into();
        lhs_u64 ^ rhs_u64
    }
}

impl BitAndAssign<Square> for u64 {
    fn bitand_assign(&mut self, rhs: Square) {
        let lhs_u64: u64 = (*self);
        let rhs_u64: u64 = rhs.into();
        *self = lhs_u64 & rhs_u64;
    }
}

impl BitOrAssign<Square> for u64 {
    fn bitor_assign(&mut self, rhs: Square) {
        let lhs_u64: u64 = (*self);
        let rhs_u64: u64 = rhs.into();
        *self = lhs_u64 | rhs_u64;
    }
}

impl BitXorAssign<Square> for u64 {
    fn bitxor_assign(&mut self, rhs: Square) {
        let lhs_u64: u64 = *self;
        let rhs_u64: u64 = rhs.into();
        *self = lhs_u64 ^ rhs_u64;
    }
}

impl Not for Square {
    type Output = u64;

    fn not(self) -> Self::Output {
        let value: u64 = self.into();
        !value
    }
}
