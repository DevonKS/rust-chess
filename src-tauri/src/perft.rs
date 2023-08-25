use crate::board;

// TODO: version with nice printing like stockfish does

pub fn perft(b: &board::Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    } else {
        let mut b2 = b.shallow_clone();
        inner_perft(&mut b2, depth)
    }
}

fn inner_perft(b: &mut board::Board, depth: u8) -> u64 {
    let moves = b.generate_moves();

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut count = 0;

    for m in moves {
        b.apply_move(m);
        count += inner_perft(b, depth - 1);
        b.undo_move();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::lookup_tables::init_lookup_tables;

    #[test]
    fn start_pos_perft() {
        // FIXME: How do I do this test setup?
        init_lookup_tables();

        let b: Board = board::Board::start_pos();
        b.print_bbs();
        assert_eq!(1, perft(&b, 0));
        assert_eq!(20, perft(&b, 1));
        assert_eq!(400, perft(&b, 2));
        assert_eq!(8902, perft(&b, 3));
        assert_eq!(197_281, perft(&b, 4));
        assert_eq!(4_865_609, perft(&b, 5));
    }
}
