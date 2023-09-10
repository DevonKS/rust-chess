// FIXME: Better name for this?

pub const MAX_MOVES: usize = 250;

pub const NOT_A_FILE: u64 = 18374403900871474942;
pub const NOT_H_FILE: u64 = 9187201950435737471;
pub const NOT_AB_FILE: u64 = 18229723555195321596;
pub const NOT_GH_FILE: u64 = 4557430888798830399;

/// Bit representation of file A.
pub const FILE_A: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
/// Bit representation of file B.
pub const FILE_B: u64 = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
/// Bit representation of file C.
pub const FILE_C: u64 = 0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
/// Bit representation of file D.
pub const FILE_D: u64 = 0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
/// Bit representation of file E.
pub const FILE_E: u64 = 0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
/// Bit representation of file F.
pub const FILE_F: u64 = 0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
/// Bit representation of file H.
pub const FILE_G: u64 = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
/// Bit representation of file G.
pub const FILE_H: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

/// Bit representation of rank 1.
pub const RANK_1: u64 = 0x0000_0000_0000_00FF;
/// Bit representation of rank 2.
pub const RANK_2: u64 = 0x0000_0000_0000_FF00;
/// Bit representation of rank 3.
pub const RANK_3: u64 = 0x0000_0000_00FF_0000;
/// Bit representation of rank 4.
pub const RANK_4: u64 = 0x0000_0000_FF00_0000;
/// Bit representation of rank 5.
pub const RANK_5: u64 = 0x0000_00FF_0000_0000;
/// Bit representation of rank 6.
pub const RANK_6: u64 = 0x0000_FF00_0000_0000;
/// Bit representation of rank 7.
pub const RANK_7: u64 = 0x00FF_0000_0000_0000;
/// Bit representation of rank 8.
pub const RANK_8: u64 = 0xFF00_0000_0000_0000;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Piece {
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    WhitePawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    BlackPawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceKind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl From<Piece> for PieceKind {
    fn from(p: Piece) -> Self {
        match p {
            Piece::WhiteRook | Piece::BlackRook => PieceKind::Rook,
            Piece::WhiteKnight | Piece::BlackKnight => PieceKind::Knight,
            Piece::WhiteBishop | Piece::BlackBishop => PieceKind::Bishop,
            Piece::WhiteQueen | Piece::BlackQueen => PieceKind::Queen,
            Piece::WhiteKing | Piece::BlackKing => PieceKind::King,
            Piece::WhitePawn | Piece::BlackPawn => PieceKind::Pawn,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Player {
    White,
    Black,
}

impl From<Piece> for Player {
    fn from(p: Piece) -> Self {
        match p {
            Piece::WhiteRook
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteQueen
            | Piece::WhiteKing
            | Piece::WhitePawn => Player::White,
            Piece::BlackRook
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackQueen
            | Piece::BlackKing
            | Piece::BlackPawn => Player::Black,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move(pub Square, pub Square);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<Square> for File {
    fn from(s: Square) -> Self {
        match s {
            Square::A1 => File::A,
            Square::B1 => File::B,
            Square::C1 => File::C,
            Square::D1 => File::D,
            Square::E1 => File::E,
            Square::F1 => File::F,
            Square::G1 => File::G,
            Square::H1 => File::H,
            Square::A2 => File::A,
            Square::B2 => File::B,
            Square::C2 => File::C,
            Square::D2 => File::D,
            Square::E2 => File::E,
            Square::F2 => File::F,
            Square::G2 => File::G,
            Square::H2 => File::H,
            Square::A3 => File::A,
            Square::B3 => File::B,
            Square::C3 => File::C,
            Square::D3 => File::D,
            Square::E3 => File::E,
            Square::F3 => File::F,
            Square::G3 => File::G,
            Square::H3 => File::H,
            Square::A4 => File::A,
            Square::B4 => File::B,
            Square::C4 => File::C,
            Square::D4 => File::D,
            Square::E4 => File::E,
            Square::F4 => File::F,
            Square::G4 => File::G,
            Square::H4 => File::H,
            Square::A5 => File::A,
            Square::B5 => File::B,
            Square::C5 => File::C,
            Square::D5 => File::D,
            Square::E5 => File::E,
            Square::F5 => File::F,
            Square::G5 => File::G,
            Square::H5 => File::H,
            Square::A6 => File::A,
            Square::B6 => File::B,
            Square::C6 => File::C,
            Square::D6 => File::D,
            Square::E6 => File::E,
            Square::F6 => File::F,
            Square::G6 => File::G,
            Square::H6 => File::H,
            Square::A7 => File::A,
            Square::B7 => File::B,
            Square::C7 => File::C,
            Square::D7 => File::D,
            Square::E7 => File::E,
            Square::F7 => File::F,
            Square::G7 => File::G,
            Square::H7 => File::H,
            Square::A8 => File::A,
            Square::B8 => File::B,
            Square::C8 => File::C,
            Square::D8 => File::D,
            Square::E8 => File::E,
            Square::F8 => File::F,
            Square::G8 => File::G,
            Square::H8 => File::H,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl From<Square> for Rank {
    fn from(s: Square) -> Self {
        match s {
            Square::A1 => Rank::R1,
            Square::B1 => Rank::R1,
            Square::C1 => Rank::R1,
            Square::D1 => Rank::R1,
            Square::E1 => Rank::R1,
            Square::F1 => Rank::R1,
            Square::G1 => Rank::R1,
            Square::H1 => Rank::R1,
            Square::A2 => Rank::R2,
            Square::B2 => Rank::R2,
            Square::C2 => Rank::R2,
            Square::D2 => Rank::R2,
            Square::E2 => Rank::R2,
            Square::F2 => Rank::R2,
            Square::G2 => Rank::R2,
            Square::H2 => Rank::R2,
            Square::A3 => Rank::R3,
            Square::B3 => Rank::R3,
            Square::C3 => Rank::R3,
            Square::D3 => Rank::R3,
            Square::E3 => Rank::R3,
            Square::F3 => Rank::R3,
            Square::G3 => Rank::R3,
            Square::H3 => Rank::R3,
            Square::A4 => Rank::R4,
            Square::B4 => Rank::R4,
            Square::C4 => Rank::R4,
            Square::D4 => Rank::R4,
            Square::E4 => Rank::R4,
            Square::F4 => Rank::R4,
            Square::G4 => Rank::R4,
            Square::H4 => Rank::R4,
            Square::A5 => Rank::R5,
            Square::B5 => Rank::R5,
            Square::C5 => Rank::R5,
            Square::D5 => Rank::R5,
            Square::E5 => Rank::R5,
            Square::F5 => Rank::R5,
            Square::G5 => Rank::R5,
            Square::H5 => Rank::R5,
            Square::A6 => Rank::R6,
            Square::B6 => Rank::R6,
            Square::C6 => Rank::R6,
            Square::D6 => Rank::R6,
            Square::E6 => Rank::R6,
            Square::F6 => Rank::R6,
            Square::G6 => Rank::R6,
            Square::H6 => Rank::R6,
            Square::A7 => Rank::R7,
            Square::B7 => Rank::R7,
            Square::C7 => Rank::R7,
            Square::D7 => Rank::R7,
            Square::E7 => Rank::R7,
            Square::F7 => Rank::R7,
            Square::G7 => Rank::R7,
            Square::H7 => Rank::R7,
            Square::A8 => Rank::R8,
            Square::B8 => Rank::R8,
            Square::C8 => Rank::R8,
            Square::D8 => Rank::R8,
            Square::E8 => Rank::R8,
            Square::F8 => Rank::R8,
            Square::G8 => Rank::R8,
            Square::H8 => Rank::R8,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Square {
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

impl TryFrom<u8> for Square {
    type Error = &'static str;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Square::A1),
            1 => Ok(Square::B1),
            2 => Ok(Square::C1),
            3 => Ok(Square::D1),
            4 => Ok(Square::E1),
            5 => Ok(Square::F1),
            6 => Ok(Square::G1),
            7 => Ok(Square::H1),
            8 => Ok(Square::A2),
            9 => Ok(Square::B2),
            10 => Ok(Square::C2),
            11 => Ok(Square::D2),
            12 => Ok(Square::E2),
            13 => Ok(Square::F2),
            14 => Ok(Square::G2),
            15 => Ok(Square::H2),
            16 => Ok(Square::A3),
            17 => Ok(Square::B3),
            18 => Ok(Square::C3),
            19 => Ok(Square::D3),
            20 => Ok(Square::E3),
            21 => Ok(Square::F3),
            22 => Ok(Square::G3),
            23 => Ok(Square::H3),
            24 => Ok(Square::A4),
            25 => Ok(Square::B4),
            26 => Ok(Square::C4),
            27 => Ok(Square::D4),
            28 => Ok(Square::E4),
            29 => Ok(Square::F4),
            30 => Ok(Square::G4),
            31 => Ok(Square::H4),
            32 => Ok(Square::A5),
            33 => Ok(Square::B5),
            34 => Ok(Square::C5),
            35 => Ok(Square::D5),
            36 => Ok(Square::E5),
            37 => Ok(Square::F5),
            38 => Ok(Square::G5),
            39 => Ok(Square::H5),
            40 => Ok(Square::A6),
            41 => Ok(Square::B6),
            42 => Ok(Square::C6),
            43 => Ok(Square::D6),
            44 => Ok(Square::E6),
            45 => Ok(Square::F6),
            46 => Ok(Square::G6),
            47 => Ok(Square::H6),
            48 => Ok(Square::A7),
            49 => Ok(Square::B7),
            50 => Ok(Square::C7),
            51 => Ok(Square::D7),
            52 => Ok(Square::E7),
            53 => Ok(Square::F7),
            54 => Ok(Square::G7),
            55 => Ok(Square::H7),
            56 => Ok(Square::A8),
            57 => Ok(Square::B8),
            58 => Ok(Square::C8),
            59 => Ok(Square::D8),
            60 => Ok(Square::E8),
            61 => Ok(Square::F8),
            62 => Ok(Square::G8),
            63 => Ok(Square::H8),
            _ => Err("Square only accepts values less than 63"),
        }
    }
}
