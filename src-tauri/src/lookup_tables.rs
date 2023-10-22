use crate::bitboard;
use crate::core;
use crate::core::SQUARES;
use crate::core::{
    File, Piece, PieceKind, Rank, Square, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE,
};

use rustc_hash::FxHashMap;

pub struct LookupTables {
    knight_moves_table: [bitboard::BitBoard; 64],
    pawn_captures_table: [[bitboard::BitBoard; 64]; 2],
    pawn_moves_table: [[bitboard::BitBoard; 64]; 2],
    king_moves_table: [bitboard::BitBoard; 64],
    rook_moves_mask: [bitboard::BitBoard; 64],
    bishop_moves_mask: [bitboard::BitBoard; 64],
    rook_moves_table: FxHashMap<(u8, u64), bitboard::BitBoard>,
    bishop_moves_table: FxHashMap<(u8, u64), bitboard::BitBoard>,
    between_sqaures_table: FxHashMap<(u8, u8), bitboard::BitBoard>,
}

impl std::fmt::Debug for LookupTables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LookupTables")
    }
}

impl LookupTables {
    pub fn generate() -> Self {
        let rook_moves_mask = gen_sliding_moves_mask(true);
        let bishop_moves_mask = gen_sliding_moves_mask(false);
        Self {
            knight_moves_table: gen_knight_moves(),
            pawn_moves_table: gen_pawn_moves(),
            pawn_captures_table: gen_pawn_capture_moves(),
            king_moves_table: gen_king_moves(),
            rook_moves_mask,
            bishop_moves_mask,
            rook_moves_table: gen_sliding_moves(&rook_moves_mask, true),
            bishop_moves_table: gen_sliding_moves(&bishop_moves_mask, false),
            between_sqaures_table: gen_between_squares(),
        }
    }

    pub fn lookup_moves(&self, p: Piece, s: Square, all_occupancy: u64) -> bitboard::BitBoard {
        match PieceKind::from(p) {
            PieceKind::Rook => {
                let blockers_key = all_occupancy & self.rook_moves_mask[s as usize].0;
                self.rook_moves_table[&(s as u8, blockers_key)]
            }
            PieceKind::Knight => self.knight_moves_table[s as usize],
            PieceKind::Bishop => {
                let blockers_key = all_occupancy & self.bishop_moves_mask[s as usize].0;
                self.bishop_moves_table[&(s as u8, blockers_key)]
            }
            PieceKind::Queen => {
                let rook_blockers_key = all_occupancy & self.rook_moves_mask[s as usize].0;
                let rook_moves = self.rook_moves_table[&(s as u8, rook_blockers_key)];
                let bishop_blockers_key = all_occupancy & self.bishop_moves_mask[s as usize].0;
                let bishop_moves = self.bishop_moves_table[&(s as u8, bishop_blockers_key)];
                bitboard::BitBoard(rook_moves.0 | bishop_moves.0)
            }
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

    pub fn lookup_between_squares(&self, from: Square, to: Square) -> bitboard::BitBoard {
        self.between_sqaures_table[&(from as u8, to as u8)]
    }
}

fn gen_knight_moves() -> [bitboard::BitBoard; 64] {
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_knight_move(s as u64))
    }
    moves.try_into().unwrap()
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
    let mut white_moves = Vec::with_capacity(64);
    let mut black_moves = Vec::with_capacity(64);
    for s in SQUARES {
        white_moves.push(gen_pawn_move(s as u64, true));
        black_moves.push(gen_pawn_move(s as u64, false));
    }
    [
        white_moves.try_into().unwrap(),
        black_moves.try_into().unwrap(),
    ]
}

fn gen_pawn_move(s: u64, is_white: bool) -> bitboard::BitBoard {
    if is_white {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb << 8;

        bitboard::BitBoard(moves)
    } else {
        let bb = 1 << s;
        let mut moves = 0;
        moves |= bb >> 8;

        bitboard::BitBoard(moves)
    }
}

fn gen_pawn_capture_moves() -> [[bitboard::BitBoard; 64]; 2] {
    let mut white_moves = Vec::with_capacity(64);
    let mut black_moves = Vec::with_capacity(64);
    for s in SQUARES {
        white_moves.push(gen_pawn_capture_move(s as u64, true));
        black_moves.push(gen_pawn_capture_move(s as u64, false));
    }
    [
        white_moves.try_into().unwrap(),
        black_moves.try_into().unwrap(),
    ]
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
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_king_move(s as u64))
    }
    moves.try_into().unwrap()
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

fn gen_sliding_moves(
    move_masks: &[bitboard::BitBoard; 64],
    is_rook: bool,
) -> FxHashMap<(u8, u64), bitboard::BitBoard> {
    let mut moves = FxHashMap::default();
    for (s, mask) in move_masks.iter().enumerate() {
        let total_blocker_combs = 2_u64.pow(mask.0.count_ones());
        for raw_blocker in 0..total_blocker_combs {
            let raw_blocker_bitboard = bitboard::BitBoard(raw_blocker);
            let mut blocker_bitboard = bitboard::BitBoard::new();
            let mut blocker_index = 0;
            let mut blocker_mask = *mask;
            while let Some(mask_index) = blocker_mask.pop_lsb() {
                let blocker_set =
                    raw_blocker_bitboard.get_bit(Square::try_from(blocker_index).unwrap());
                if blocker_set {
                    blocker_bitboard.set_bit(mask_index);
                } else {
                    blocker_bitboard.unset_bit(mask_index);
                }
                blocker_index += 1
            }
            moves.insert(
                (s as u8, blocker_bitboard.0),
                gen_sliding_move(s as u8, blocker_bitboard.0, is_rook),
            );
        }
    }
    moves
}

fn gen_sliding_move(s: u8, blockers: u64, is_rook: bool) -> bitboard::BitBoard {
    let directions = if is_rook {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
    } else {
        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
    };

    let square = Square::try_from(s).unwrap();
    let mut moves_bitboard = bitboard::BitBoard::new();
    let blockers_bitboard = bitboard::BitBoard(blockers);
    for dir in directions {
        let mut current_rank: i8 = (Rank::from(square) as i8) + 1;
        let mut current_file: i8 = (File::from(square) as i8) + 1;
        for _ in 1..8 {
            current_rank += dir.1;
            current_file += dir.0;

            if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
                break;
            }

            let bit_index =
                Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

            moves_bitboard.set_bit(bit_index);

            if blockers_bitboard.get_bit(bit_index) {
                break;
            }
        }
    }

    moves_bitboard
}

fn gen_sliding_moves_mask(is_rook: bool) -> [bitboard::BitBoard; 64] {
    let mut moves = Vec::with_capacity(64);
    for s in SQUARES {
        moves.push(gen_sliding_move_mask(s as u8, is_rook));
    }
    moves.try_into().unwrap()
}

fn gen_sliding_move_mask(s: u8, is_rook: bool) -> bitboard::BitBoard {
    let directions = if is_rook {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
    } else {
        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
    };

    let square = Square::try_from(s).unwrap();
    let mut moves_bitboard = bitboard::BitBoard::new();

    for dir in directions {
        let mut current_rank: i8 = (Rank::from(square) as i8) + 1;
        let mut current_file: i8 = (File::from(square) as i8) + 1;
        for _ in 1..8 {
            current_rank += dir.1;
            current_file += dir.0;

            if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
                break;
            }

            let bit_index =
                Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

            moves_bitboard.set_bit(bit_index);
        }
    }

    let file = File::from(square);
    let rank = Rank::from(square);
    if file != File::A {
        moves_bitboard.0 &= !core::FILE_A;
    }
    if file != File::H {
        moves_bitboard.0 &= !core::FILE_H;
    }
    if rank != Rank::R1 {
        moves_bitboard.0 &= !core::RANK_1
    }
    if rank != Rank::R8 {
        moves_bitboard.0 &= !core::RANK_8
    }

    moves_bitboard
}

fn gen_between_squares() -> FxHashMap<(u8, u8), bitboard::BitBoard> {
    let mut table = FxHashMap::default();
    for from in SQUARES {
        for to in SQUARES {
            let from_file = File::from(from);
            let from_rank = Rank::from(from);
            let to_file = File::from(to);
            let to_rank = Rank::from(to);
            let same_file = from_file == to_file;
            let same_rank = from_rank == to_rank;
            let same_diag =
                (from_file as i8 - to_file as i8).abs() == (from_rank as i8 - to_rank as i8).abs();
            if from != to && (same_file || same_rank || same_diag) {
                table.insert((from as u8, to as u8), gen_between_squares_inner(from, to));
            }
        }
    }

    table
}

fn gen_between_squares_inner(from: Square, to: Square) -> bitboard::BitBoard {
    let from_file = File::from(from) as u8;
    let from_rank = Rank::from(from) as u8;
    let to_file = File::from(to) as u8;
    let to_rank = Rank::from(to) as u8;

    let direction: (i8, i8) = if from_file == to_file {
        if from_file < to_file {
            (1, 0)
        } else {
            (-1, 0)
        }
    } else if from_rank == to_rank {
        if from_rank < to_rank {
            (0, 1)
        } else {
            (0, -1)
        }
    } else if from_file < to_file {
        if from_rank < to_rank {
            (1, 1)
        } else {
            (1, -1)
        }
    } else if from_rank < to_rank {
        (-1, 1)
    } else {
        (-1, -1)
    };

    let mut moves = bitboard::BitBoard::new();
    let mut current_file = from_file as i8 + 1;
    let mut current_rank = from_rank as i8 + 1;
    for _ in 0..8 {
        current_file += direction.0;
        current_rank += direction.1;

        if !(1..=8).contains(&current_rank) || !(1..=8).contains(&current_file) {
            break;
        }

        if current_file as u8 - 1 == to_file && current_rank as u8 - 1 == to_rank {
            break;
        }

        let bit_index =
            Square::try_from(((current_file - 1) + (current_rank - 1) * 8) as u8).unwrap();

        moves.set_bit(bit_index);
    }

    moves
}
