// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_chess::bitboard;
use rust_chess::board;
use rust_chess::core::Square;
use rust_chess::lookup_tables;

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

    // let mut blockers_bitboard = bitboard::BitBoard::new();
    // blockers_bitboard.set_bit(core::Square::A3);
    // blockers_bitboard.set_bit(core::Square::B1);
    //
    // l.lookup_moves(
    //     core::Piece::WhiteRook,
    //     core::Square::A1,
    //     blockers_bitboard.0,
    // )
    // .print();
    //
    // let mut b = board::Board::start_pos(&l);
    // b.apply_move(core::Move(core::Square::E2, core::Square::E4));
    // b.apply_move(core::Move(core::Square::H7, core::Square::H6));

    //b.print_bbs();

    // let moves = b.generate_moves();
    // println!("{}: {:?}", moves.len(), moves);

    // let b2 = board::Board::start_pos(&l);
    // perft::perft_pp(&b2, 3);
    //
    let b = board::Board::start_pos(&l);
    b.print();
    let moves = b.generate_moves();
    println!("{}: {:?}", moves.len(), moves);
}
