// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use rust_chess::core::{Move, PieceKind, Square};
use rust_chess::lookup_tables;
use rust_chess::{board, perft};

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }
//
// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    let l = lookup_tables::LookupTables::generate();

    // let mut b = board::Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ", &l).unwrap();
    // b.apply_move(Move(Square::B4, Square::A4, None));
    // b.apply_move(Move(Square::D6, Square::D5, None));
    // b.apply_move(Move(Square::A5, Square::B4, None));
    // b.apply_move(Move(Square::C7, Square::C5, None));
    //
    // b.generate_moves_2(board::Legality::Legal);
    // println!("{:?}", b.is_valid());

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
