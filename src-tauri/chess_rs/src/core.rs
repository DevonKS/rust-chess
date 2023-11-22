// FIXME: Better name for this module?

use std::fmt;

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

pub const STARTING_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const POS_2_KIWIPETE_FEN: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
pub const POS_3_FEN: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
pub const POS_4_FEN: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const POS_4_MIRRORED_FEN: &str =
    "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ";
pub const POS_5_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const POS_6_FEN: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

pub const EN_PASSANT_FEN: &str = "rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
pub const IN_CHECK_FEN: &str = "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 2 3";

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

pub const WHITE_PIECES: [Piece; 6] = [
    Piece::WhiteRook,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteQueen,
    Piece::WhiteKing,
    Piece::WhitePawn,
];

pub const BLACK_PIECES: [Piece; 6] = [
    Piece::BlackRook,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackQueen,
    Piece::BlackKing,
    Piece::BlackPawn,
];

pub const PIECES: [Piece; 12] = [
    Piece::WhiteRook,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteQueen,
    Piece::WhiteKing,
    Piece::WhitePawn,
    Piece::BlackRook,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackQueen,
    Piece::BlackKing,
    Piece::BlackPawn,
];

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::WhiteRook => "R",
                Piece::WhiteKnight => "N",
                Piece::WhiteBishop => "B",
                Piece::WhiteQueen => "Q",
                Piece::WhiteKing => "K",
                Piece::WhitePawn => "P",
                Piece::BlackRook => "r",
                Piece::BlackKnight => "n",
                Piece::BlackBishop => "b",
                Piece::BlackQueen => "q",
                Piece::BlackKing => "k",
                Piece::BlackPawn => "p",
            }
        )
    }
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Piece::WhiteRook),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'P' => Ok(Piece::WhitePawn),
            'r' => Ok(Piece::BlackRook),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            'p' => Ok(Piece::BlackPawn),
            _ => Err(format!("unknown piece character: {}", c)),
        }
    }
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

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceKind::Rook => "r",
                PieceKind::Knight => "n",
                PieceKind::Bishop => "b",
                PieceKind::Queen => "q",
                PieceKind::King => "k",
                PieceKind::Pawn => "p",
            }
        )
    }
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

impl TryFrom<&str> for PieceKind {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "r" => Ok(PieceKind::Rook),
            "n" => Ok(PieceKind::Knight),
            "b" => Ok(PieceKind::Bishop),
            "q" => Ok(PieceKind::Queen),
            "k" => Ok(PieceKind::King),
            "p" => Ok(PieceKind::Pawn),
            _ => Err(format!("unknown piece kind: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Player {
    White,
    Black,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::White => "w",
                Player::Black => "b",
            }
        )
    }
}

impl TryFrom<char> for Player {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Player::White),
            'b' => Ok(Player::Black),
            _ => Err(format!("unknown player character: {}", c)),
        }
    }
}

impl TryFrom<&str> for Player {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "w" => Ok(Player::White),
            "b" => Ok(Player::Black),
            _ => Err(format!("unknown player character: {}", s)),
        }
    }
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

pub const PLAYERS: [Player; 2] = [Player::White, Player::Black];

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move(pub Square, pub Square, pub Option<PieceKind>);

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let promotion = match self.2 {
            Some(pk) => pk.to_string(),
            None => "".to_string(),
        };
        write!(f, "{}{}{}", self.0, self.1, promotion)
    }
}

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

pub const FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::A => "a",
                File::B => "b",
                File::C => "c",
                File::D => "d",
                File::E => "e",
                File::F => "f",
                File::G => "g",
                File::H => "h",
            }
        )
    }
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

pub const RANKS: [Rank; 8] = [
    Rank::R1,
    Rank::R2,
    Rank::R3,
    Rank::R4,
    Rank::R5,
    Rank::R6,
    Rank::R7,
    Rank::R8,
];

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::R1 => "1",
                Rank::R2 => "2",
                Rank::R3 => "3",
                Rank::R4 => "4",
                Rank::R5 => "5",
                Rank::R6 => "6",
                Rank::R7 => "7",
                Rank::R8 => "8",
            }
        )
    }
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

pub const SQUARES: [Square; 64] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
];

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::A1 => "a1",
                Square::B1 => "b1",
                Square::C1 => "c1",
                Square::D1 => "d1",
                Square::E1 => "e1",
                Square::F1 => "f1",
                Square::G1 => "g1",
                Square::H1 => "h1",
                Square::A2 => "a2",
                Square::B2 => "b2",
                Square::C2 => "c2",
                Square::D2 => "d2",
                Square::E2 => "e2",
                Square::F2 => "f2",
                Square::G2 => "g2",
                Square::H2 => "h2",
                Square::A3 => "a3",
                Square::B3 => "b3",
                Square::C3 => "c3",
                Square::D3 => "d3",
                Square::E3 => "e3",
                Square::F3 => "f3",
                Square::G3 => "g3",
                Square::H3 => "h3",
                Square::A4 => "a4",
                Square::B4 => "b4",
                Square::C4 => "c4",
                Square::D4 => "d4",
                Square::E4 => "e4",
                Square::F4 => "f4",
                Square::G4 => "g4",
                Square::H4 => "h4",
                Square::A5 => "a5",
                Square::B5 => "b5",
                Square::C5 => "c5",
                Square::D5 => "d5",
                Square::E5 => "e5",
                Square::F5 => "f5",
                Square::G5 => "g5",
                Square::H5 => "h5",
                Square::A6 => "a6",
                Square::B6 => "b6",
                Square::C6 => "c6",
                Square::D6 => "d6",
                Square::E6 => "e6",
                Square::F6 => "f6",
                Square::G6 => "g6",
                Square::H6 => "h6",
                Square::A7 => "a7",
                Square::B7 => "b7",
                Square::C7 => "c7",
                Square::D7 => "d7",
                Square::E7 => "e7",
                Square::F7 => "f7",
                Square::G7 => "g7",
                Square::H7 => "h7",
                Square::A8 => "a8",
                Square::B8 => "b8",
                Square::C8 => "c8",
                Square::D8 => "d8",
                Square::E8 => "e8",
                Square::F8 => "f8",
                Square::G8 => "g8",
                Square::H8 => "h8",
            }
        )
    }
}

impl From<(File, Rank)> for Square {
    fn from(value: (File, Rank)) -> Self {
        match value {
            (File::A, Rank::R1) => Square::A1,
            (File::A, Rank::R2) => Square::A2,
            (File::A, Rank::R3) => Square::A3,
            (File::A, Rank::R4) => Square::A4,
            (File::A, Rank::R5) => Square::A5,
            (File::A, Rank::R6) => Square::A6,
            (File::A, Rank::R7) => Square::A7,
            (File::A, Rank::R8) => Square::A8,
            (File::B, Rank::R1) => Square::B1,
            (File::B, Rank::R2) => Square::B2,
            (File::B, Rank::R3) => Square::B3,
            (File::B, Rank::R4) => Square::B4,
            (File::B, Rank::R5) => Square::B5,
            (File::B, Rank::R6) => Square::B6,
            (File::B, Rank::R7) => Square::B7,
            (File::B, Rank::R8) => Square::B8,
            (File::C, Rank::R1) => Square::C1,
            (File::C, Rank::R2) => Square::C2,
            (File::C, Rank::R3) => Square::C3,
            (File::C, Rank::R4) => Square::C4,
            (File::C, Rank::R5) => Square::C5,
            (File::C, Rank::R6) => Square::C6,
            (File::C, Rank::R7) => Square::C7,
            (File::C, Rank::R8) => Square::C8,
            (File::D, Rank::R1) => Square::D1,
            (File::D, Rank::R2) => Square::D2,
            (File::D, Rank::R3) => Square::D3,
            (File::D, Rank::R4) => Square::D4,
            (File::D, Rank::R5) => Square::D5,
            (File::D, Rank::R6) => Square::D6,
            (File::D, Rank::R7) => Square::D7,
            (File::D, Rank::R8) => Square::D8,
            (File::E, Rank::R1) => Square::E1,
            (File::E, Rank::R2) => Square::E2,
            (File::E, Rank::R3) => Square::E3,
            (File::E, Rank::R4) => Square::E4,
            (File::E, Rank::R5) => Square::E5,
            (File::E, Rank::R6) => Square::E6,
            (File::E, Rank::R7) => Square::E7,
            (File::E, Rank::R8) => Square::E8,
            (File::F, Rank::R1) => Square::F1,
            (File::F, Rank::R2) => Square::F2,
            (File::F, Rank::R3) => Square::F3,
            (File::F, Rank::R4) => Square::F4,
            (File::F, Rank::R5) => Square::F5,
            (File::F, Rank::R6) => Square::F6,
            (File::F, Rank::R7) => Square::F7,
            (File::F, Rank::R8) => Square::F8,
            (File::G, Rank::R1) => Square::G1,
            (File::G, Rank::R2) => Square::G2,
            (File::G, Rank::R3) => Square::G3,
            (File::G, Rank::R4) => Square::G4,
            (File::G, Rank::R5) => Square::G5,
            (File::G, Rank::R6) => Square::G6,
            (File::G, Rank::R7) => Square::G7,
            (File::G, Rank::R8) => Square::G8,
            (File::H, Rank::R1) => Square::H1,
            (File::H, Rank::R2) => Square::H2,
            (File::H, Rank::R3) => Square::H3,
            (File::H, Rank::R4) => Square::H4,
            (File::H, Rank::R5) => Square::H5,
            (File::H, Rank::R6) => Square::H6,
            (File::H, Rank::R7) => Square::H7,
            (File::H, Rank::R8) => Square::H8,
        }
    }
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

impl TryFrom<&str> for Square {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "a1" => Ok(Square::A1),
            "b1" => Ok(Square::B1),
            "c1" => Ok(Square::C1),
            "d1" => Ok(Square::D1),
            "e1" => Ok(Square::E1),
            "f1" => Ok(Square::F1),
            "g1" => Ok(Square::G1),
            "h1" => Ok(Square::H1),
            "a2" => Ok(Square::A2),
            "b2" => Ok(Square::B2),
            "c2" => Ok(Square::C2),
            "d2" => Ok(Square::D2),
            "e2" => Ok(Square::E2),
            "f2" => Ok(Square::F2),
            "g2" => Ok(Square::G2),
            "h2" => Ok(Square::H2),
            "a3" => Ok(Square::A3),
            "b3" => Ok(Square::B3),
            "c3" => Ok(Square::C3),
            "d3" => Ok(Square::D3),
            "e3" => Ok(Square::E3),
            "f3" => Ok(Square::F3),
            "g3" => Ok(Square::G3),
            "h3" => Ok(Square::H3),
            "a4" => Ok(Square::A4),
            "b4" => Ok(Square::B4),
            "c4" => Ok(Square::C4),
            "d4" => Ok(Square::D4),
            "e4" => Ok(Square::E4),
            "f4" => Ok(Square::F4),
            "g4" => Ok(Square::G4),
            "h4" => Ok(Square::H4),
            "a5" => Ok(Square::A5),
            "b5" => Ok(Square::B5),
            "c5" => Ok(Square::C5),
            "d5" => Ok(Square::D5),
            "e5" => Ok(Square::E5),
            "f5" => Ok(Square::F5),
            "g5" => Ok(Square::G5),
            "h5" => Ok(Square::H5),
            "a6" => Ok(Square::A6),
            "b6" => Ok(Square::B6),
            "c6" => Ok(Square::C6),
            "d6" => Ok(Square::D6),
            "e6" => Ok(Square::E6),
            "f6" => Ok(Square::F6),
            "g6" => Ok(Square::G6),
            "h6" => Ok(Square::H6),
            "a7" => Ok(Square::A7),
            "b7" => Ok(Square::B7),
            "c7" => Ok(Square::C7),
            "d7" => Ok(Square::D7),
            "e7" => Ok(Square::E7),
            "f7" => Ok(Square::F7),
            "g7" => Ok(Square::G7),
            "h7" => Ok(Square::H7),
            "a8" => Ok(Square::A8),
            "b8" => Ok(Square::B8),
            "c8" => Ok(Square::C8),
            "d8" => Ok(Square::D8),
            "e8" => Ok(Square::E8),
            "f8" => Ok(Square::F8),
            "g8" => Ok(Square::G8),
            "h8" => Ok(Square::H8),
            _ => Err(format!("unknown square: {}", s)),
        }
    }
}
