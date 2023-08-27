use crate::bitboard;

use std::sync::Once;

static mut KNIGHT_TABLE: [u64; 64] = [0; 64];
static mut PAWN_CAPTURE_TABLE: [[u64; 64]; 2] = [[0; 64]; 2];
static mut PAWN_TABLE: [[u64; 64]; 2] = [[0; 64]; 2];
static mut KING_TABLE: [u64; 64] = [0; 64];

static NOT_A_FILE: u64 = 18374403900871474942;
static NOT_H_FILE: u64 = 9187201950435737471;
static NOT_AB_FILE: u64 = 18229723555195321596;
static NOT_GH_FILE: u64 = 4557430888798830399;

static INIT: Once = Once::new();

#[cold]
pub fn init_lookup_tables() {
    INIT.call_once(|| {
        gen_knight_moves();
        gen_pawn_moves();
        gen_pawn_capture_moves();
        gen_king_moves();
    });
}

pub fn knight_moves(s: bitboard::Square) -> bitboard::BitBoard {
    unsafe { bitboard::BitBoard(KNIGHT_TABLE[s as usize]) }
}

pub fn pawn_moves(s: bitboard::Square, is_white: bool) -> bitboard::BitBoard {
    unsafe { bitboard::BitBoard(PAWN_TABLE[if is_white { 0 } else { 1 }][s as usize]) }
}

pub fn pawn_capture_moves(s: bitboard::Square, is_white: bool) -> bitboard::BitBoard {
    unsafe { bitboard::BitBoard(PAWN_CAPTURE_TABLE[if is_white { 0 } else { 1 }][s as usize]) }
}

pub fn king_moves(s: bitboard::Square) -> bitboard::BitBoard {
    unsafe { bitboard::BitBoard(KING_TABLE[s as usize]) }
}

fn gen_knight_moves() {
    unsafe {
        for s in 0..64 {
            KNIGHT_TABLE[s] = gen_knight_move(s as u64);
        }
    }
}

fn gen_knight_move(s: u64) -> u64 {
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
    moves
}

fn gen_pawn_moves() {
    unsafe {
        for s in 0..64 {
            PAWN_TABLE[0][s] = gen_pawn_move(s as u64, true);
            PAWN_TABLE[1][s] = gen_pawn_move(s as u64, false);
        }
    }
}

fn gen_pawn_move(s: u64, is_white: bool) -> u64 {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb << 8;

        if s >= 8 && s <= 15 {
            moves |= bb << 16;
        }

        moves
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb >> 8;

        if s >= 48 && s <= 55 {
            moves |= bb >> 16;
        }

        moves
    }
}

fn gen_pawn_capture_moves() {
    unsafe {
        for s in 0..64 {
            PAWN_CAPTURE_TABLE[0][s] = gen_pawn_capture_move(s as u64, true);
            PAWN_CAPTURE_TABLE[1][s] = gen_pawn_capture_move(s as u64, false);
        }
    }
}

fn gen_pawn_capture_move(s: u64, is_white: bool) -> u64 {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb << 9) & NOT_A_FILE;
        moves |= (bb << 7) & NOT_H_FILE;
        moves
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= (bb >> 9) & NOT_H_FILE;
        moves |= (bb >> 7) & NOT_A_FILE;
        moves
    }
}

fn gen_king_moves() {
    unsafe {
        for s in 0..64 {
            KING_TABLE[s] = gen_king_move(s as u64);
        }
    }
}

fn gen_king_move(s: u64) -> u64 {
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

    moves
}
