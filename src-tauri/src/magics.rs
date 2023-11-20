use core::panic;

use rand::Rng;
use rustc_hash::FxHashSet;

use crate::{
    bitboard::BitBoard,
    core::Square,
    lookup_tables::{apply_magic_number, create_blocker_bitboard, gen_sliding_moves_mask},
};

pub fn generate_magic_numbers() {
    let rook_moves_mask = gen_sliding_moves_mask(true);
    let bishop_moves_mask = gen_sliding_moves_mask(false);

    let mut rook_magics_shifts: [(u64, u64); 64] = [(0, u64::MAX); 64];
    let mut bishop_magics_shifts: [(u64, u64); 64] = [(0, u64::MAX); 64];

    // println!("generating magic numbers");

    for s in 0..64 {
        let rook_mask = rook_moves_mask[s];
        let rook_total_blocker_combs = 2_u64.pow(rook_mask.pop_count());

        let rook_magic_opt = find_magic(rook_mask, rook_total_blocker_combs);
        match rook_magic_opt {
            Some(m) => {
                println!(
                    "Found a rook magic for {} that needs {} bits",
                    Square::try_from(s as u8).unwrap(),
                    m.1
                );
                rook_magics_shifts[s] = m;
            }
            None => panic!(
                "Couldn't find rook magic number for {}",
                Square::try_from(s as u8).unwrap()
            ),
        }

        let bishop_mask = bishop_moves_mask[s];
        let bishop_total_blocker_combs = 2_u64.pow(bishop_mask.pop_count());

        let bishop_magic_opt = find_magic(bishop_mask, bishop_total_blocker_combs);
        match bishop_magic_opt {
            Some(m) => {
                println!(
                    "Found a bishop magic for {} that needs {} bits",
                    Square::try_from(s as u8).unwrap(),
                    m.1
                );
                bishop_magics_shifts[s] = m;
            }
            None => panic!(
                "Couldn't find bishop magic number for {}",
                Square::try_from(s as u8).unwrap()
            ),
        }
    }

    println!("{:?}\n{:?}", rook_magics_shifts, bishop_magics_shifts);
}

fn gen_magic() -> u64 {
    random_u64() & random_u64() & random_u64()
}

fn find_magic(mask: BitBoard, total_blocker_combs: u64) -> Option<(u64, u64)> {
    'magics: for _ in 0..100_000_000 {
        let magic = gen_magic();
        if ((mask * magic) & 0xFF00000000000000).pop_count() < 6 {
            continue;
        }

        // For some squares I could probably use less bit shifts but I'm not too concerned
        // about memory.
        let shifts = 12;
        let mut used: FxHashSet<u64> = FxHashSet::default();
        for raw_blocker in 0..total_blocker_combs {
            let blocker_bitboard = create_blocker_bitboard(mask, raw_blocker);
            let index = apply_magic_number(blocker_bitboard, magic, shifts);
            if used.contains(&index.0) {
                continue 'magics;
            } else {
                used.insert(index.0);
            }
        }

        return Some((magic, shifts));
    }

    None
}

fn random_u64() -> u64 {
    let mut rng = rand::thread_rng();

    let u1: u64 = rng.gen::<u64>() & 0xFFFF;
    let u2: u64 = rng.gen::<u64>() & 0xFFFF;
    let u3: u64 = rng.gen::<u64>() & 0xFFFF;
    let u4: u64 = rng.gen::<u64>() & 0xFFFF;

    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}
