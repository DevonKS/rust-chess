use crate::board;

// TODO: version with nice printing like stockfish does
//
pub fn perft_pp(b: &board::Board, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut b2 = b.shallow_clone();
        let n = inner_perft(&mut b2, depth, true);
        println!("\n{n}");
        n
    }
}

pub fn perft(b: &board::Board, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut b2 = b.shallow_clone();
        inner_perft(&mut b2, depth, false)
    }
}

fn inner_perft(b: &mut board::Board, depth: u8, print_results: bool) -> u64 {
    let moves = b.generate_moves(board::Legality::Legal);

    let mut count = 0;

    for m in moves {
        let count_for_move = if depth == 1 {
            1
        } else {
            b.apply_move(m);
            let n = inner_perft(b, depth - 1, false);
            b.undo_move();
            n
        };

        if print_results {
            println!("{} {}", m, count_for_move);
        }

        count += count_for_move;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::core;
    use crate::lookup_tables::LookupTables;

    #[test]
    fn start_pos_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::start_pos(&l);
        assert_eq!(1, perft(&b, 0));
        assert_eq!(20, perft(&b, 1));
        assert_eq!(400, perft(&b, 2));
        assert_eq!(8902, perft(&b, 3));
        assert_eq!(197_281, perft(&b, 4));
        assert_eq!(4_865_609, perft(&b, 5));
        assert_eq!(119_060_324, perft(&b, 6));
    }

    #[test]
    fn pos_2_kiwipete_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_2_KIWIPETE_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(48, perft(&b, 1));
        assert_eq!(2039, perft(&b, 2));
        assert_eq!(97862, perft(&b, 3));
        assert_eq!(4_085_603, perft(&b, 4));
        assert_eq!(193_690_690, perft(&b, 5));
    }

    #[test]
    #[ignore]
    fn pos_2_kiwipete_perft_slow() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_2_KIWIPETE_FEN, &l).unwrap();
        // This one is too slow
        assert_eq!(8_031_647_685, perft(&b, 6));
    }

    #[test]
    fn pos_3_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_3_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(14, perft(&b, 1));
        assert_eq!(191, perft(&b, 2));
        assert_eq!(2812, perft(&b, 3));
        assert_eq!(43_238, perft(&b, 4));
        assert_eq!(674_624, perft(&b, 5));
        assert_eq!(11_030_083, perft(&b, 6));
    }

    #[test]
    fn pos_4_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_4_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(6, perft(&b, 1));
        assert_eq!(264, perft(&b, 2));
        assert_eq!(9467, perft(&b, 3));
        assert_eq!(422_333, perft(&b, 4));
        assert_eq!(15_833_292, perft(&b, 5));
        assert_eq!(706_045_033, perft(&b, 6));
    }

    #[test]
    fn pos_4_mirrored_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_4_MIRRORED_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(6, perft(&b, 1));
        assert_eq!(264, perft(&b, 2));
        assert_eq!(9467, perft(&b, 3));
        assert_eq!(422_333, perft(&b, 4));
        assert_eq!(15_833_292, perft(&b, 5));
        assert_eq!(706_045_033, perft(&b, 6));
    }

    #[test]
    fn pos_5_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_5_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(44, perft(&b, 1));
        assert_eq!(1486, perft(&b, 2));
        assert_eq!(62_379, perft(&b, 3));
        assert_eq!(2_103_487, perft(&b, 4));
        assert_eq!(89_941_194, perft(&b, 5));
    }

    #[test]
    fn pos_6_perft() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_6_FEN, &l).unwrap();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(46, perft(&b, 1));
        assert_eq!(2079, perft(&b, 2));
        assert_eq!(89_890, perft(&b, 3));
        assert_eq!(3_894_594, perft(&b, 4));
        assert_eq!(164_075_551, perft(&b, 5));
    }

    #[test]
    #[ignore]
    fn pos_6_perft_slow() {
        let l = LookupTables::generate();

        let b: Board = board::Board::from_fen(core::POS_6_FEN, &l).unwrap();
        assert_eq!(6_923_051_137, perft(&b, 6));
    }
}
