use std::env;

use chess_rs::core::{Move, PieceKind, Square};
use chess_rs::lookup_tables;
use chess_rs::{board, perft};

fn main() {
    let l = lookup_tables::LookupTables::generate();

    perftree(&l);
}

fn perftree(l: &lookup_tables::LookupTables) {
    let args: Vec<String> = env::args().collect();

    let depth = if let Some(depth_str) = args.get(1) {
        depth_str.parse().unwrap()
    } else {
        6
    };

    let fen = if let Some(fen) = args.get(2) {
        fen.clone()
    } else {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    };

    let mut b = board::Board::from_fen(&fen, l).unwrap();

    if let Some(moves) = args.get(3) {
        for m_str in moves.split(' ') {
            let m = if m_str.len() == 4 {
                let source = Square::try_from(&m_str[0..2]).unwrap();
                let dest = Square::try_from(&m_str[2..]).unwrap();
                Move(source, dest, None)
            } else if m_str.len() == 5 {
                let source = Square::try_from(&m_str[0..2]).unwrap();
                let dest = Square::try_from(&m_str[2..4]).unwrap();
                let promotion = Some(PieceKind::try_from(&m_str[4..]).expect("cannot parse move"));
                Move(source, dest, promotion)
            } else {
                panic!("invalid move");
            };

            let legal_moves = b.generate_moves(board::Legality::Legal);
            b.apply_move(m, &legal_moves);
        }
    }

    perft::perft_pp(&b, depth);
}
