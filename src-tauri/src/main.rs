// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bitboard;
mod board;
mod lookup_tables;
mod perft;

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
    lookup_tables::init_lookup_tables();

    let mut b = board::Board::start_pos();
    // b.apply_move(board::Move(bitboard::Square::A2, bitboard::Square::A3));
    // b.apply_move(board::Move(bitboard::Square::H7, bitboard::Square::H6));
    //
    // b.print_bbs();
    //
    // let moves = b.generate_moves();
    // println!("{:?}", moves);

    perft::perft_pp(&b, 3);
}
