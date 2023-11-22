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
        for m in moves.split(' ') {
            if m.len() == 4 {
                let source = Square::try_from(&m[0..2]).unwrap();
                let dest = Square::try_from(&m[2..]).unwrap();
                b.apply_move(Move(source, dest, None));
            } else if m.len() == 5 {
                let source = Square::try_from(&m[0..2]).unwrap();
                let dest = Square::try_from(&m[2..4]).unwrap();
                let promotion = Some(PieceKind::try_from(&m[4..]).expect("cannot parse move"));
                b.apply_move(Move(source, dest, promotion));
            } else {
                panic!("invalid move");
            }
        }
    }

    perft::perft_pp(&b, depth);
}
