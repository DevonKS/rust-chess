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

    print_king_moves(bitboard::Square::A1);
    print_king_moves(bitboard::Square::H1);
    print_king_moves(bitboard::Square::A8);
    print_king_moves(bitboard::Square::H8);
    print_king_moves(bitboard::Square::E4);

    let b = board::Board::start_pos();
    let moves = b.generate_moves();
    println!("{:?}", moves);
}

fn print_king_moves(s: bitboard::Square) {
    let mut bb = bitboard::BitBoard::new();
    bb.set_bit(s);

    println!("{s:?} King moves:");
    let moves = lookup_tables::king_moves(s);
    moves.print();
    println!("-------------------------------------------");
}
