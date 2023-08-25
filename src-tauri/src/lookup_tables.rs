use crate::bitboard;

use std::sync::Once;

static mut KNIGHT_TABLE: [u64; 64] = [0; 64];

static NOT_A_FILE: u64 = 18374403900871474942;
static NOT_H_FILE: u64 = 9187201950435737471;
static NOT_AB_FILE: u64 = 18229723555195321596;
static NOT_GH_FILE: u64 = 4557430888798830399;

static INIT: Once = Once::new();

#[cold]
pub fn init_lookup_tables() {
    INIT.call_once(|| {
        gen_knight_moves();
    });
}

pub fn knight_moves(s: bitboard::Square) -> bitboard::BitBoard {
    unsafe { bitboard::BitBoard(KNIGHT_TABLE[s as usize]) }
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
    // offsets 17, 15, 10, 6
    if ((bb << 17) & NOT_A_FILE) > 0 {
        moves |= bb << 17;
    }
    if ((bb << 15) & NOT_H_FILE) > 0 {
        moves |= bb << 15;
    }
    if ((bb << 10) & NOT_AB_FILE) > 0 {
        moves |= bb << 10;
    }
    if ((bb << 6) & NOT_GH_FILE) > 0 {
        moves |= bb << 6;
    }
    if ((bb >> 6) & NOT_AB_FILE) > 0 {
        moves |= bb >> 6;
    }
    if ((bb >> 10) & NOT_GH_FILE) > 0 {
        moves |= bb >> 10;
    }
    if ((bb >> 15) & NOT_A_FILE) > 0 {
        moves |= bb >> 15;
    }
    if ((bb >> 17) & NOT_H_FILE) > 0 {
        moves |= bb >> 17;
    }
    moves
}
