use crate::bitboard;
use crate::core;
use crate::core::{
    File, Piece, PieceKind, Rank, Square, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE,
};

pub struct LookupTables {
    knight_moves_table: [bitboard::BitBoard; 64],
    pawn_captures_table: [[bitboard::BitBoard; 64]; 2],
    pawn_moves_table: [[bitboard::BitBoard; 64]; 2],
    king_moves_table: [bitboard::BitBoard; 64],
    rook_moves_mask: [bitboard::BitBoard; 64],
}

impl std::fmt::Debug for LookupTables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LookupTables")
    }
}

impl LookupTables {
    // FIXME: Is it bad practice for new to do heavy lifting?
    pub fn new() -> Self {
        Self {
            knight_moves_table: gen_knight_moves(),
            pawn_moves_table: gen_pawn_moves(),
            pawn_captures_table: gen_pawn_capture_moves(),
            king_moves_table: gen_king_moves(),
            rook_moves_mask: gen_rook_moves_mask(),
        }
    }

    pub fn lookup_moves(&self, p: Piece, s: Square) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Rook => bitboard::BitBoard::new(),
            PieceKind::Knight => self.knight_moves_table[s as usize],
            PieceKind::Bishop => bitboard::BitBoard::new(),
            PieceKind::Queen => bitboard::BitBoard::new(),
            PieceKind::King => self.king_moves_table[s as usize],
            PieceKind::Pawn => {
                self.pawn_moves_table[if p == Piece::WhitePawn { 0 } else { 1 }][s as usize]
            }
        }
    }

    pub fn lookup_capture_moves(&self, p: Piece, s: Square) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Pawn => {
                self.pawn_captures_table[if p == Piece::WhitePawn { 0 } else { 1 }][s as usize]
            }
            _ => panic!("lookup_capture_moves is only supported for Pawns"),
        }
    }

    pub fn lookup_moves_mask(&self, p: Piece, s: Square) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Rook => self.rook_moves_mask[s as usize],
            _ => panic!("lookup_moves_mask is only supported for Rooks"),
        }
    }
}

fn gen_knight_moves() -> [bitboard::BitBoard; 64] {
    let mut moves = [bitboard::BitBoard::new(); 64];
    for s in 0..64 {
        moves[s] = gen_knight_move(s as u64);
    }
    moves
}

fn gen_knight_move(s: u64) -> bitboard::BitBoard {
    let bb = 1 << s;
    let mut moves = 0;
    moves |= (bb << 17) & NOT_A_FILE;
    moves |= (bb << 15) & NOT_H_FILE;
    moves |= (bb << 10) & NOT_AB_FILE;
    moves |= (bb << 6) & NOT_GH_FILE;
    moves |= (bb >> 6) & NOT_AB_FILE;
    moves |= (bb >> 10) & NOT_GH_FILE;
    moves |= (bb >> 15) & NOT_A_FILE;
    moves |= (bb >> 17) & NOT_H_FILE;
    bitboard::BitBoard(moves)
}

fn gen_pawn_moves() -> [[bitboard::BitBoard; 64]; 2] {
    let mut moves = [[bitboard::BitBoard::new(); 64]; 2];
    for s in 0..64 {
        moves[0][s] = gen_pawn_move(s as u64, true);
        moves[1][s] = gen_pawn_move(s as u64, false);
    }
    moves
}

fn gen_pawn_move(s: u64, is_white: bool) -> bitboard::BitBoard {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb << 8;

        // if s >= 8 && s <= 15 {
        //     moves |= bb << 16;
        // }

        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb >> 8;

        // if s >= 48 && s <= 55 {
        //     moves |= bb >> 16;
        // }

        bitboard::BitBoard(moves)
    }
}

fn gen_pawn_capture_moves() -> [[bitboard::BitBoard; 64]; 2] {
    let mut moves = [[bitboard::BitBoard::new(); 64]; 2];
    for s in 0..64 {
        moves[0][s] = gen_pawn_capture_move(s as u64, true);
        moves[1][s] = gen_pawn_capture_move(s as u64, false);
    }
    moves
}

fn gen_pawn_capture_move(s: u64, is_white: bool) -> bitboard::BitBoard {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb << 9) & NOT_A_FILE;
        moves |= (bb << 7) & NOT_H_FILE;
        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb >> 9) & NOT_H_FILE;
        moves |= (bb >> 7) & NOT_A_FILE;
        bitboard::BitBoard(moves)
    }
}

fn gen_king_moves() -> [bitboard::BitBoard; 64] {
    let mut moves = [bitboard::BitBoard::new(); 64];
    for s in 0..64 {
        moves[s] = gen_king_move(s as u64);
    }
    moves
}

fn gen_king_move(s: u64) -> bitboard::BitBoard {
    let bb = 1 << s;
    let mut moves = 0;
    moves |= bb << 8;
    moves |= bb >> 8;
    moves |= (bb >> 9) & NOT_H_FILE;
    moves |= (bb >> 7) & NOT_A_FILE;
    moves |= (bb << 9) & NOT_A_FILE;
    moves |= (bb << 7) & NOT_H_FILE;
    moves |= (bb << 1) & NOT_A_FILE;
    moves |= (bb >> 1) & NOT_H_FILE;

    bitboard::BitBoard(moves)
}
